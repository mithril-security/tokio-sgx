import re
import shutil
import subprocess
from dataclasses import dataclass
from collections import OrderedDict
from typing import List
import treelib

def removesuffix(self: str, suffix: str, /) -> str:
    if self.endswith(suffix):
        return self[:-len(suffix)]
    else:
        return self[:]

@dataclass
class Crate(treelib.Node):
    info: str
    name: str
    features : List[str]
    dependencies: List['Crate']

    def __str__(self, level=0):
        ret = "    " * level + "├── "  + str(self.info) + "\n"
        for child in self.dependencies:
            ret += child.__str__(level + 1)
        return ret


shutil.copy("Cargo_target.toml", "Cargo.toml")

result = subprocess.run(
    r"""RUST_LOG=error RUSTFLAGS="-Z force-unstable-if-unmarked" RUST_TARGET_PATH=$XARGO_PATH xargo tree --target x86_64-mesalock-unknown-sgx --edges no-build --prefix depth --format '# {p} # {f}' """,
    check=True, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)

lines = result.stdout.decode().splitlines()

for index_start, line in enumerate(lines):
    if line.startswith('0#'):
        break
lines = lines[index_start:]
lines = [ line.split('# ') for line in lines]
lines = [line for line in lines if line]
lines = [(int(depth), info, features) for (depth, info, features) in lines]

def parse_dep(lines):
    cur_depth, info, features = lines.pop(0)
    cur_depth = int(cur_depth)
    features = removesuffix(features, ' (*)').split(',')
    features = [feature.strip() for feature in features if feature.strip()]
    # print(features)
    # print(info, "depth = ", cur_depth)
    dependencies = []
    # lines[0][0] is next_node_depth
    while lines and int(lines[0][0]) == cur_depth + 1:
        dependencies.append(parse_dep(lines))
    # print(info)
    name = re.search(r"^\b([\w\-]+) .*$", info)[1]
    return Crate(info = info, name = name, dependencies=dependencies, features=features)

crate_tree = parse_dep(lines)

def tree_filter_crate_proc_macro(crate_tree):
    if '(proc-macro)' in crate_tree.info:
        return None
    return Crate(info=crate_tree.info, name= crate_tree.name, features=crate_tree.features, dependencies=[ dep for dependency in crate_tree.dependencies if (dep := tree_filter_crate_proc_macro(dependency)) ])

def get_list(crate_tree):
    return [{'name':crate_tree.name, 'features': crate_tree.features}] + [ dep for deps in crate_tree.dependencies for dep in get_list(deps)]

crate_tree_sgx = tree_filter_crate_proc_macro(crate_tree)
print(crate_tree_sgx)
sgx_crates_dict = OrderedDict(sorted({crate['name'] : crate for crate in get_list(crate_tree_sgx)}.items()))
sgx_crates_dict.pop("template")

# print(sgx_crates_dict)


for crate in sgx_crates_dict.values():
    f = open("Cargo.toml", "w")
    f.write("""cargo-features = ["resolver"]
[package]
name = "template"
version = "1.0.0"
edition = "2018"
authors = ["The Teaclave Authors"]
resolver = "2"
[lib]
name = "sample" # Library name. If you change this, please reflect those changes in the Makefile on the variable ENCLAVE_CARGO_LIB
crate-type = ["staticlib"] 
[features]
default = []
[dependencies]
""" + "" + crate['name'] + " = { default-features= false, features = " + str(crate['features'])  +" }" +
"""
""")
    f.close()
    try:
        result = subprocess.run(
            r"""RUST_LOG=error RUSTFLAGS="-Z force-unstable-if-unmarked" RUST_TARGET_PATH=$XARGO_PATH xargo check --target x86_64-mesalock-unknown-sgx""",
            check=True, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        print(f"✓ {crate['name']} {','.join(crate['features'])}")
    except subprocess.CalledProcessError as error:
        print(f"✘ {crate['name']} {','.join(crate['features'])}")
        # print(error.output.decode())