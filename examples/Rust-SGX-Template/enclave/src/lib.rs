// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

#![crate_name = "sample"]
#![crate_type = "staticlib"]

#![cfg_attr(not(target_env = "sgx"), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate sgx_types;
#[cfg(not(target_env = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;
extern crate sgx_libc as libc;
extern crate tokio;
extern crate tokio_test;
extern crate futures;
extern crate bytes;

extern crate core;

mod tests;

use sgx_types::*;
use std::io::{self, Write};
use std::slice;

#[no_mangle]
pub extern "C" fn ecall_test(some_string: *const u8, some_len: usize) -> sgx_status_t {

    let str_slice = unsafe { slice::from_raw_parts(some_string, some_len) };
    let _ = io::stdout().write(str_slice);

    tests::test_buffered();
    tests::test_io_async_read();
    tests::test_io_buf_reader();
    tests::test_io_buf_writer();
    tests::test_io_chain();
    tests::test_io_copy_bidirectional();
    tests::test_io_driver();
    tests::test_io_driver_drop();
    tests::test_io_lines();
    tests::test_io_mem_stream();
    tests::test_io_read();
    tests::test_io_read_buf();
    tests::test_io_read_exact();
    tests::test_io_read_line();
    tests::test_io_read_to_end();
    tests::test_io_read_to_string();
    tests::test_io_read_until();
    tests::test_io_split();
    tests::test_io_take();
    tests::test_io_write();
    tests::test_io_write_all();
    tests::test_io_write_all_buf();
    tests::test_io_write_buf();
    tests::test_io_write_int();
    tests::test_macros_join();
    tests::test_macros_pin();
    tests::test_macros_select();
    tests::test_macros_test();
    tests::test_macros_try_join();
    tests::test_net_bind_resource();
    tests::test_net_lookup_host();
    tests::test_no_rt();
    tests::test_rt_basic();
    tests::test_rt_common();
    
    sgx_status_t::SGX_SUCCESS
}
