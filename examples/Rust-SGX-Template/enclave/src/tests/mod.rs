//use futures::executor::block_on;

mod buffered;
mod io_async_read;
mod io_buf_reader;
mod io_buf_writer;
mod io_chain;
mod io_copy_bidirectional;
mod io_driver;
mod io_driver_drop;
mod io_lines;
mod io_mem_stream;
mod io_read;
mod io_read_buf;
mod io_read_exact;
mod io_read_line;
mod io_read_to_end;
mod io_read_to_string;
mod io_read_until;
mod io_split;
mod io_take;
mod io_write;
mod io_write_all;
mod io_write_all_buf;
mod io_write_buf;
mod io_write_int;
mod macros_join;
mod macros_pin;
mod macros_select;
mod macros_test;
mod macros_try_join;
mod named_pipe;
mod net_bind_resource;
mod net_lookup_host;
mod no_rt;
mod rt_basic;
mod rt_common;

pub mod support;

pub fn test_buffered() {
    println!("Testing functions of mod buffered inside enclave.");

    print!("Testing function echo_server() ...");
    
    buffered::echo_server();

    print!("Succeeded.\n");

    println!("Done testing functions of mod buffered.\n")

}

pub fn test_io_async_read() {
    println!("Testing functions of mod io_async_read inside enclave.");

    print!("Testing function assert_obj_safe() ...");
    
    io_async_read::assert_obj_safe();

    print!("Succeeded.\n");

    println!("Done testing functions of mod io_async_read.\n")
}

pub fn test_io_buf_reader() {
    println!("Testing functions of mod io_buf_reader inside enclave.");

    print!("Testing function test_buffered_reader() ...");
    io_buf_reader::test_buffered_reader();
    print!("Succeeded.\n");

    print!("Testing function test_buffered_reader_seek() ...");
    io_buf_reader::test_buffered_reader_seek();
    print!("Succeeded.\n");

    print!("Testing function test_buffered_reader_seek_underflow() ...");
    io_buf_reader::test_buffered_reader_seek_underflow();
    print!("Succeeded.\n");

    print!("Testing function test_short_reads() ...");
    io_buf_reader::test_short_reads();
    print!("Succeeded.\n");

    print!("Testing function maybe_pending() ...");
    io_buf_reader::maybe_pending();
    print!("Succeeded.\n");

    print!("Testing function maybe_pending_buf_read() ...");
    io_buf_reader::maybe_pending_buf_read();
    print!("Succeeded.\n");

    print!("Testing function maybe_pending_seek() ...");
    io_buf_reader::maybe_pending_seek();
    print!("Succeeded.\n");

    print!("Testing function test_fill_buf_wrapper() ...");
    io_buf_reader::test_fill_buf_wrapper();
    print!("Succeeded.\n");
    

    println!("Done testing functions of mod io_buf_reader.\n")
}

pub fn test_io_buf_writer() {
    println!("Testing functions of mod io_buf_writer inside enclave.");

    print!("Testing function buf_writer() ...");
    io_buf_writer::buf_writer();
    print!("Succeeded.\n");

    print!("Testing function buf_writer_inner_flushes() ...");
    io_buf_writer::buf_writer_inner_flushes();
    print!("Succeeded.\n");

    print!("Testing function buf_writer_seek() ...");
    io_buf_writer::buf_writer_seek();
    print!("Succeeded.\n");

    print!("Testing function maybe_pending_buf_writer() ...");
    io_buf_writer::maybe_pending_buf_writer();
    print!("Succeeded.\n");

    print!("Testing function maybe_pending_buf_writer_inner_flushes() ...");
    io_buf_writer::maybe_pending_buf_writer_inner_flushes();
    print!("Succeeded.\n");

    print!("Testing function maybe_pending_buf_writer_seek() ...");
    io_buf_writer::maybe_pending_buf_writer_seek();
    print!("Succeeded.\n");
    
    print!("Testing function write_vectored_empty_on_non_vectored() ...");
    io_buf_writer::write_vectored_empty_on_non_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_empty_on_vectored() ...");
    io_buf_writer::write_vectored_empty_on_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_basic_on_non_vectored() ...");
    io_buf_writer::write_vectored_basic_on_non_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_basic_on_vectored() ...");
    io_buf_writer::write_vectored_basic_on_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_large_total_on_non_vectored() ...");
    io_buf_writer::write_vectored_large_total_on_non_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_large_total_on_vectored() ...");
    io_buf_writer::write_vectored_large_total_on_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_odd_on_non_vectored() ...");
    io_buf_writer::write_vectored_odd_on_non_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_odd_on_vectored() ...");
    io_buf_writer::write_vectored_odd_on_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_large_slice_on_non_vectored() ...");
    io_buf_writer::write_vectored_large_slice_on_non_vectored();
    print!("Succeeded.\n");

    print!("Testing function write_vectored_large_slice_on_vectored() ...");
    io_buf_writer::write_vectored_large_slice_on_vectored();
    print!("Succeeded.\n");

    
    

    println!("Done testing functions of mod io_buf_writer.\n")
}

pub fn test_io_chain() {
    println!("Testing functions of mod io_chain inside enclave.");

    print!("Testing function chain() ...");
    
    io_chain::chain();

    print!("Succeeded.\n");

    println!("Done testing functions of mod io_chain.\n")
}

pub fn test_io_copy_bidirectional() {

    println!("Testing functions of mod io_copy_bidirectional inside enclave.");
    

    print!("Testing function test_transfer_after_close()...");
    io_copy_bidirectional::test_transfer_after_close();
    print!("Succeeded.\n");
        

    print!("Testing function test_basic_transfer()...");
    io_copy_bidirectional::test_basic_transfer();
    print!("Succeeded.\n");
        

    print!("Testing function blocking_one_side_does_not_block_other()...");
    //io_copy_bidirectional::blocking_one_side_does_not_block_other();
    //print!("Succeeded.\n");
    println!("Not tested. Enclave memory issue.");

    print!("Testing function immediate_exit_on_error()...");
    io_copy_bidirectional::immediate_exit_on_error();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_copy_bidirectional.\n");
    
}

pub fn test_io_driver() {

    println!("Testing functions of mod io_driver inside enclave.");
    

    print!("Testing function test_drop_on_notify()...");
    io_driver::test_drop_on_notify();
    print!("Succeeded.\n");
        

    print!("Testing function panics_when_io_disabled()...");
    //io_driver::panics_when_io_disabled();
    print!("Should panic. Not tested.\n");
        

    println!("Done testing functions of mod io_driver.\n");
}

pub fn test_io_driver_drop() {

    println!("Testing functions of mod io_driver_drop inside enclave.");
    

    print!("Testing function drop_wakes()...");
    io_driver_drop::drop_wakes();
    print!("Succeeded.\n");
        

    print!("Testing function tcp_doesnt_block()...");
    io_driver_drop::tcp_doesnt_block();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_driver_drop.\n");
    
}

pub fn test_io_lines() {

    println!("Testing functions of mod io_lines inside enclave.");
    

    print!("Testing function lines_inherent()...");
    io_lines::lines_inherent();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_lines.\n");
    
}

pub fn test_io_mem_stream() {

    println!("Testing functions of mod io_mem_stream inside enclave.");
    

    print!("Testing function ping_pong()...");
    io_mem_stream::ping_pong();
    print!("Succeeded.\n");
        

    print!("Testing function disconnect_reader()...");
    io_mem_stream::disconnect_reader();
    print!("Succeeded.\n");
        print!("Testing function max_write_size()...");
    io_mem_stream::max_write_size();
    print!("Succeeded.\n");
        

    print!("Testing function disconnect()...");
    io_mem_stream::disconnect();
    print!("Succeeded.\n");
        

    print!("Testing function across_tasks()...");
    io_mem_stream::across_tasks();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_mem_stream.\n");
    
}


pub fn test_io_read() {

    println!("Testing functions of mod io_read inside enclave.");
    

    print!("Testing function read()...");
    io_read::read();
    print!("Succeeded.\n");
    
    print!("Testing function read_buf_bad_async_read()...");
    //io_read::read_buf_bad_async_read();
    print!("Should panic. Not tested.\n");
        
    println!("Done testing functions of mod io_read.\n");
    
}


pub fn test_io_read_buf() {

    println!("Testing functions of mod io_read_buf inside enclave.");
    

    print!("Testing function read_buf()...");
    io_read_buf::read_buf();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_read_buf.\n");
    
}

pub fn test_io_read_exact() {

    println!("Testing functions of mod io_read_exact inside enclave.");
    

    print!("Testing function read_exact()...");
    io_read_exact::read_exact();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_read_exact.\n");
    
}



pub fn test_io_read_line() {

    println!("Testing functions of mod io_read_line inside enclave.");
    

    print!("Testing function read_line_fail_and_utf8_fail()...");
    io_read_line::read_line_fail_and_utf8_fail();
    print!("Succeeded.\n");
        

    print!("Testing function read_line_fail()...");
    io_read_line::read_line_fail();
    print!("Succeeded.\n");
        

    print!("Testing function read_line_not_all_ready()...");
    io_read_line::read_line_not_all_ready();
    print!("Succeeded.\n");
        

    print!("Testing function read_line_invalid_utf8()...");
    io_read_line::read_line_invalid_utf8();
    print!("Succeeded.\n");
        

    print!("Testing function read_line()...");
    io_read_line::read_line();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_read_line.\n");
    
}



pub fn test_io_read_to_end() {

    println!("Testing functions of mod io_read_to_end inside enclave.");
    

    print!("Testing function read_to_end()...");
    io_read_to_end::read_to_end();
    print!("Succeeded.\n");
        

    print!("Testing function read_to_end_uninit()...");
    io_read_to_end::read_to_end_uninit();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_read_to_end.\n");
    
}



pub fn test_io_read_to_string() {

    println!("Testing functions of mod io_read_to_string inside enclave.");
    

    print!("Testing function to_string_appends()...");
    io_read_to_string::to_string_appends();
    print!("Succeeded.\n");
        

    print!("Testing function read_to_string()...");
    io_read_to_string::read_to_string();
    print!("Succeeded.\n");
        

    print!("Testing function to_string_does_not_truncate_on_utf8_error()...");
    io_read_to_string::to_string_does_not_truncate_on_utf8_error();
    print!("Succeeded.\n");
        

    print!("Testing function to_string_does_not_truncate_on_io_error()...");
    io_read_to_string::to_string_does_not_truncate_on_io_error();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_read_to_string.\n");
    
}



pub fn test_io_read_until() {

    println!("Testing functions of mod io_read_until inside enclave.");
    

    print!("Testing function read_until_fail()...");
    io_read_until::read_until_fail();
    print!("Succeeded.\n");
        

    print!("Testing function read_until()...");
    io_read_until::read_until();
    print!("Succeeded.\n");
        

    print!("Testing function read_until_not_all_ready()...");
    io_read_until::read_until_not_all_ready();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_read_until.\n");
    
}



pub fn test_io_split() {

    println!("Testing functions of mod io_split inside enclave.");
    

    print!("Testing function is_send_and_sync()...");
    io_split::is_send_and_sync();
    print!("Succeeded.\n");
        

    print!("Testing function split_stream_id()...");
    io_split::split_stream_id();
    print!("Succeeded.\n");
        

    print!("Testing function unsplit_ok()...");
    io_split::unsplit_ok();
    print!("Succeeded.\n");
        

    print!("Testing function unsplit_err1()...");
    //io_split::unsplit_err1();
    print!("Should panic. Not tested.\n");
        

    print!("Testing function unsplit_err2()...");
    //io_split::unsplit_err2();
    print!("Should panic. Not tested.\n");
        

    println!("Done testing functions of mod io_split.\n");
    
}



pub fn test_io_take() {

    println!("Testing functions of mod io_take inside enclave.");
    

    print!("Testing function take()...");
    io_take::take();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_take.\n");
    
}



pub fn test_io_write() {

    println!("Testing functions of mod io_write inside enclave.");
    

    print!("Testing function write_cursor()...");
    io_write::write_cursor();
    print!("Succeeded.\n");
        

    print!("Testing function write()...");
    io_write::write();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_write.\n");
    
}



pub fn test_io_write_all() {

    println!("Testing functions of mod io_write_all inside enclave.");
    

    print!("Testing function write_all()...");
    io_write_all::write_all();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_write_all.\n");
    
}



pub fn test_io_write_all_buf() {

    println!("Testing functions of mod io_write_all_buf inside enclave.");
    

    print!("Testing function write_all_buf()...");
    io_write_all_buf::write_all_buf();
    print!("Succeeded.\n");
        

    print!("Testing function write_buf_err()...");
    io_write_all_buf::write_buf_err();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_write_all_buf.\n");
    
}



pub fn test_io_write_buf() {

    println!("Testing functions of mod io_write_buf inside enclave.");
    

    print!("Testing function write_all()...");
    io_write_buf::write_all();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_write_buf.\n");
    
}



pub fn test_io_write_int() {

    println!("Testing functions of mod io_write_int inside enclave.");
    

    print!("Testing function write_int_should_err_if_write_count_0()...");
    io_write_int::write_int_should_err_if_write_count_0();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod io_write_int.\n");
    
}

pub fn test_macros_join() {

    println!("Testing functions of mod macros_join inside enclave.");
    

    print!("Testing function join_size()...");
    macros_join::join_size();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_lit_expr_comma()...");
    macros_join::sync_one_lit_expr_comma();
    print!("Succeeded.\n");
        

    print!("Testing function sync_two_lit_expr_comma()...");
    macros_join::sync_two_lit_expr_comma();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_lit_expr_no_comma()...");
    macros_join::sync_one_lit_expr_no_comma();
    print!("Succeeded.\n");
        

    print!("Testing function sync_two_lit_expr_no_comma()...");
    macros_join::sync_two_lit_expr_no_comma();
    print!("Succeeded.\n");
        

    print!("Testing function two_await()...");
    macros_join::two_await();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod macros_join.\n");
    
}



pub fn test_macros_pin() {

    println!("Testing functions of mod macros_pin inside enclave.");
    

    print!("Testing function multi_pin()...");
    macros_pin::multi_pin();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod macros_pin.\n");
    
}



pub fn test_macros_select() {

    println!("Testing functions of mod macros_select inside enclave.");
    

    print!("Testing function biased_one_not_ready()...");
    macros_select::biased_one_not_ready();
    print!("Succeeded.\n");
        

    print!("Testing function disable_with_if()...");
    macros_select::disable_with_if();
    print!("Succeeded.\n");
        

    print!("Testing function move_uncompleted_futures()...");
    macros_select::move_uncompleted_futures();
    print!("Succeeded.\n");
        

    print!("Testing function join_with_select()...");
    macros_select::join_with_select();
    print!("Succeeded.\n");
        

    print!("Testing function many_branches()...");
    macros_select::many_branches();
    print!("Succeeded.\n");
        

    print!("Testing function biased_eventually_ready()...");
    macros_select::biased_eventually_ready();
    print!("Succeeded.\n");
        

    print!("Testing function future_panics_after_poll()...");
    macros_select::future_panics_after_poll();
    print!("Succeeded.\n");
        

    print!("Testing function drop_in_fut()...");
    macros_select::drop_in_fut();
    print!("Succeeded.\n");
        

    print!("Testing function mut_on_left_hand_side()...");
    macros_select::mut_on_left_hand_side();
    print!("Succeeded.\n");
        

    print!("Testing function nested()...");
    macros_select::nested();
    print!("Succeeded.\n");
        

    print!("Testing function mutable_borrowing_future_with_same_borrow_in_block_and_else()...");
    macros_select::mutable_borrowing_future_with_same_borrow_in_block_and_else();
    print!("Succeeded.\n");
        

    print!("Testing function one_ready()...");
    macros_select::one_ready();
    print!("Succeeded.\n");
        

    print!("Testing function nested_one()...");
    macros_select::nested_one();
    print!("Succeeded.\n");
        

    print!("Testing function mutable_borrowing_future_with_same_borrow_in_block()...");
    macros_select::mutable_borrowing_future_with_same_borrow_in_block();
    print!("Succeeded.\n");
        

    print!("Testing function select_streams()...");
    macros_select::select_streams();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_await()...");
    macros_select::sync_one_await();
    print!("Succeeded.\n");
        

    print!("Testing function struct_size()...");
    macros_select::struct_size();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_ident()...");
    macros_select::sync_one_ident();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_lit_expr_block()...");
   macros_select::sync_one_lit_expr_block();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_lit_expr_comma()...");
    macros_select::sync_one_lit_expr_comma();
    print!("Succeeded.\n");
        

    print!("Testing function use_future_in_if_condition()...");
    macros_select::use_future_in_if_condition();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_lit_expr_no_comma()...");
    macros_select::sync_one_lit_expr_no_comma();
    print!("Succeeded.\n");
        

    print!("Testing function use_future_in_if_condition_biased()...");
    macros_select::use_future_in_if_condition_biased();
    print!("Succeeded.\n");
        

    print!("Testing function never_branch_no_warnings()...");
    macros_select::never_branch_no_warnings();
    print!("Succeeded.\n");
        

    print!("Testing function sync_two()...");
    macros_select::sync_two();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod macros_select.\n");
    
}



pub fn test_macros_test() {

    println!("Testing functions of mod macros_test inside enclave.");
    

    /* print!("Testing function unused_braces_test()...");
    macros_test::unused_braces_test();
    print!("Succeeded.\n");*/
    //`rt-multi-thread` feature is disabled.
        

    print!("Testing function test_macro_can_be_used_via_use()...");
    macros_test::test_macro_can_be_used_via_use();
    print!("Succeeded.\n");
        

    print!("Testing function test_macro_is_resilient_to_shadowing()...");
    macros_test::test_macro_is_resilient_to_shadowing();
    print!("Succeeded.\n");
        

    /*print!("Testing function trait_method()...");
    macros_test::trait_method();
    print!("Succeeded.\n");*/
    //`rt-multi-thread` feature is disabled.
        

    println!("Done testing functions of mod macros_test.\n");
    
}



pub fn test_macros_try_join() {

    println!("Testing functions of mod macros_try_join inside enclave.");
    

    print!("Testing function join_size()...");
    macros_try_join::join_size();
    print!("Succeeded.\n");
        

    print!("Testing function two_await()...");
    macros_try_join::two_await();
    print!("Succeeded.\n");
        

    print!("Testing function err_abort_early()...");
    macros_try_join::err_abort_early();
    print!("Succeeded.\n");
        

    print!("Testing function sync_two_lit_expr_no_comma()...");
    macros_try_join::sync_two_lit_expr_no_comma();
    print!("Succeeded.\n");
        

    print!("Testing function sync_two_lit_expr_comma()...");
    macros_try_join::sync_two_lit_expr_comma();
    print!("Succeeded.\n");
        

    print!("Testing function sync_one_lit_expr_no_comma()...");
    macros_try_join::sync_one_lit_expr_no_comma();
    print!("Succeeded.\n");
        print!("Testing function sync_one_lit_expr_comma()...");
    macros_try_join::sync_one_lit_expr_comma();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod macros_try_join.\n");
    
}



pub fn test_net_bind_resource() {

    println!("Testing functions of mod net_bind_resource inside enclave.");
    

    print!("Testing function no_runtime_panics_binding_net_tcp_listener()...");
    //net_bind_resource::no_runtime_panics_binding_net_tcp_listener();
    print!("Should panic. Not tested.\n");
        

    println!("Done testing functions of mod net_bind_resource.\n");
    
}



pub fn test_net_lookup_host() {

    println!("Testing functions of mod net_lookup_host inside enclave.");
    

    print!("Testing function lookup_socket_addr()...");
    net_lookup_host::lookup_socket_addr();
    print!("Succeeded.\n");
        

    print!("Testing function lookup_str_socket_addr()...");
    net_lookup_host::lookup_str_socket_addr();
    print!("Succeeded.\n");
        

    print!("Testing function resolve_dns()...");
    net_lookup_host::resolve_dns().unwrap();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod net_lookup_host.\n");
    
}



pub fn test_no_rt() {

    println!("Testing functions of mod no_rt inside enclave.");
    

    print!("Testing function io_panics_when_no_tokio_context()...");
    //no_rt::io_panics_when_no_tokio_context();
    print!("Should panic. Not tested.\n");


    print!("Testing function panics_when_no_reactor()...");
    //no_rt::panics_when_no_reactor();
    print!("Should panic. Not tested.\n");
        

    print!("Testing function timeout_panics_when_no_tokio_context()...");
    //no_rt::timeout_panics_when_no_tokio_context();
    print!("Should panic. Not tested.\n");
        

    println!("Done testing functions of mod no_rt.\n");
    
}

pub fn test_rt_basic() {

    println!("Testing functions of mod rt_basic inside enclave.");
    

    print!("Testing function acquire_mutex_in_drop()...");
    rt_basic::acquire_mutex_in_drop();
    print!("Succeeded.\n");
        

    print!("Testing function no_extra_poll()...");
    rt_basic::no_extra_poll();
    print!("Succeeded.\n");
        

    print!("Testing function timeout_panics_when_no_time_handle()...");
    //rt_basic::timeout_panics_when_no_time_handle();
    print!("Should panic. Not tested.\n");
        

    print!("Testing function spawned_task_does_not_progress_without_block_on()...");
    rt_basic::spawned_task_does_not_progress_without_block_on();
    print!("Succeeded.\n");
        

    println!("Done testing functions of mod rt_basic.\n");
    
}


pub fn test_rt_common() {

    println!("Testing functions of mod rt_common inside enclave.");
    

    print!("Testing function current_thread_scheduler::block_on_sync()...");
    rt_common::current_thread_scheduler::block_on_sync();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::always_active_parker()...");
    rt_common::current_thread_scheduler::always_active_parker();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::block_on_socket()...");
    rt_common::current_thread_scheduler::block_on_socket();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::client_server_block_on()...");
    rt_common::current_thread_scheduler::client_server_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::create_rt_in_block_on()...");
    rt_common::current_thread_scheduler::create_rt_in_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::eagerly_drops_futures_on_shutdown()...");
    rt_common::current_thread_scheduler::eagerly_drops_futures_on_shutdown();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::complete_block_on_under_load()...");
    rt_common::current_thread_scheduler::complete_block_on_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::block_on_async()...");
    rt_common::current_thread_scheduler::block_on_async();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::enter_and_spawn()...");
    rt_common::current_thread_scheduler::enter_and_spawn();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::complete_task_under_load()...");
    rt_common::current_thread_scheduler::complete_task_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::local_set_client_server_block_on()...");
    rt_common::current_thread_scheduler::local_set_client_server_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::local_set_block_on_socket()...");
    rt_common::current_thread_scheduler::local_set_block_on_socket();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::io_driver_called_when_under_load()...");
    rt_common::current_thread_scheduler::io_driver_called_when_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::outstanding_tasks_dropped()...");
    rt_common::current_thread_scheduler::outstanding_tasks_dropped();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::runtime_in_thread_local()...");
    rt_common::current_thread_scheduler::runtime_in_thread_local();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::ping_pong_saturation()...");
    rt_common::current_thread_scheduler::ping_pong_saturation();
    print!("Succeeded.\n");
        

    print!("Testing functioncurrent_thread_scheduler::shutdown_timeout_0()...");
    rt_common::current_thread_scheduler::shutdown_timeout_0();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::coop()...");
    rt_common::current_thread_scheduler::coop();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::coop_unconstrained()...");
    rt_common::current_thread_scheduler::coop_unconstrained();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::sleep_from_blocking()...");
    rt_common::current_thread_scheduler::sleep_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::panic_in_task()...");
    rt_common::current_thread_scheduler::panic_in_task();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::panic_in_block_on()...");
    //rt_common::current_thread_scheduler::panic_in_block_on();
    print!("Should panic. Not tested.\n");
        

    print!("Testing function current_thread_scheduler::io_notify_while_shutting_down()...");
    //rt_common::current_thread_scheduler::io_notify_while_shutting_down();
    print!("Test failed in original crate.\n");
        

    print!("Testing function current_thread_scheduler::nested_rt()...");
    //rt_common::current_thread_scheduler::nested_rt();
    print!("Should panic. Not tested.\n");
        

    print!("Testing function current_thread_scheduler::spawn_await_chain()...");
    rt_common::current_thread_scheduler::spawn_await_chain();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_from_blocking()...");
    rt_common::current_thread_scheduler::spawn_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_blocking_from_blocking()...");
    rt_common::current_thread_scheduler::spawn_blocking_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_from_other_thread_under_load()...");
    rt_common::current_thread_scheduler::spawn_from_other_thread_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_many_from_block_on()...");
    rt_common::current_thread_scheduler::spawn_many_from_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_one_bg()...");
    rt_common::current_thread_scheduler::spawn_one_bg();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_one_join()...");
    rt_common::current_thread_scheduler::spawn_one_join();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_two()...");
    rt_common::current_thread_scheduler::spawn_two();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::wake_while_rt_is_dropping()...");
    rt_common::current_thread_scheduler::wake_while_rt_is_dropping();
    print!("Succeeded.\n");
        

    print!("Testing function send_sync_bound()...");
    rt_common::send_sync_bound();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_many_from_task()...");
    rt_common::current_thread_scheduler::spawn_many_from_task();
    print!("Succeeded.\n");
        

    /* print!("Testing function threaded_scheduler_1_thread::always_active_parker()...");
    rt_common::threaded_scheduler_1_thread::always_active_parker();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::block_on_socket()...");
    rt_common::threaded_scheduler_1_thread::block_on_socket();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::block_on_sync()...");
    rt_common::threaded_scheduler_1_thread::block_on_sync();
    print!("Succeeded.\n");*/
        

    print!("Testing function current_thread_scheduler::sleep_at_root()...");
    rt_common::current_thread_scheduler::sleep_at_root();
    print!("Succeeded.\n");
        

    /*print!("Testing function threaded_scheduler_1_thread::client_server_block_on()...");
    rt_common::threaded_scheduler_1_thread::client_server_block_on();
    print!("Succeeded.\n");*/
        

    print!("Testing function current_thread_scheduler::sleep_in_spawn()...");
    rt_common::current_thread_scheduler::sleep_in_spawn();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::spawn_from_other_thread_idle()...");
    rt_common::current_thread_scheduler::spawn_from_other_thread_idle();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::shutdown_timeout()...");
    rt_common::current_thread_scheduler::shutdown_timeout();
    print!("Succeeded.\n");
        

    /*print!("Testing function threaded_scheduler_1_thread::create_rt_in_block_on()...");
    rt_common::threaded_scheduler_1_thread::create_rt_in_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::eagerly_drops_futures_on_shutdown()...");
    rt_common::threaded_scheduler_1_thread::eagerly_drops_futures_on_shutdown();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::enter_and_spawn()...");
    rt_common::threaded_scheduler_1_thread::enter_and_spawn();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::block_on_async()...");
    rt_common::threaded_scheduler_1_thread::block_on_async();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::socket_from_blocking()...");
    rt_common::current_thread_scheduler::socket_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::io_notify_while_shutting_down()...");
    rt_common::threaded_scheduler_1_thread::io_notify_while_shutting_down();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::local_set_block_on_socket()...");
    rt_common::threaded_scheduler_1_thread::local_set_block_on_socket();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::local_set_client_server_block_on()...");
    rt_common::threaded_scheduler_1_thread::local_set_client_server_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::nested_rt()...");
    rt_common::threaded_scheduler_1_thread::nested_rt();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::outstanding_tasks_dropped()...");
    rt_common::threaded_scheduler_1_thread::outstanding_tasks_dropped();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::panic_in_block_on()...");
    //rt_common::threaded_scheduler_1_thread::panic_in_block_on();
    print!("Should panic. Not tested.\n");
        

    print!("Testing function threaded_scheduler_1_thread::panic_in_task()...");
    rt_common::threaded_scheduler_1_thread::panic_in_task();
    print!("Succeeded.\n");
        

    print!("Testing function current_thread_scheduler::shutdown_wakeup_time()...");
    rt_common::current_thread_scheduler::shutdown_wakeup_time();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::runtime_in_thread_local()...");
    rt_common::threaded_scheduler_1_thread::runtime_in_thread_local();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::shutdown_timeout_0()...");
    rt_common::threaded_scheduler_1_thread::shutdown_timeout_0();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::complete_block_on_under_load()...");
    rt_common::threaded_scheduler_1_thread::complete_block_on_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::complete_task_under_load()...");
    rt_common::threaded_scheduler_1_thread::complete_task_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::sleep_from_blocking()...");
    rt_common::threaded_scheduler_1_thread::sleep_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::io_driver_called_when_under_load()...");
    rt_common::threaded_scheduler_1_thread::io_driver_called_when_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::socket_from_blocking()...");
    rt_common::threaded_scheduler_1_thread::socket_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_await_chain()...");
    rt_common::threaded_scheduler_1_thread::spawn_await_chain();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_blocking_from_blocking()...");
    rt_common::threaded_scheduler_1_thread::spawn_blocking_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_from_blocking()...");
    rt_common::threaded_scheduler_1_thread::spawn_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::sleep_at_root()...");
    rt_common::threaded_scheduler_1_thread::sleep_at_root();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_from_other_thread_under_load()...");
    rt_common::threaded_scheduler_1_thread::spawn_from_other_thread_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::sleep_in_spawn()...");
    rt_common::threaded_scheduler_1_thread::sleep_in_spawn();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_many_from_block_on()...");
    rt_common::threaded_scheduler_1_thread::spawn_many_from_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::coop()...");
    rt_common::threaded_scheduler_1_thread::coop();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_one_bg()...");
    rt_common::threaded_scheduler_1_thread::spawn_one_bg();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_one_join()...");
    rt_common::threaded_scheduler_1_thread::spawn_one_join();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_two()...");
    rt_common::threaded_scheduler_1_thread::spawn_two();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::wake_while_rt_is_dropping()...");
    rt_common::threaded_scheduler_1_thread::wake_while_rt_is_dropping();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::coop_unconstrained()...");
    rt_common::threaded_scheduler_1_thread::coop_unconstrained();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::spawn_many_from_task()...");
    rt_common::threaded_scheduler_1_thread::spawn_many_from_task();
    print!("Succeeded.\n"); */
        

    /* print!("Testing function threaded_scheduler_4_threads::block_on_socket()...");
    rt_common::threaded_scheduler_4_threads::block_on_socket();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::client_server_block_on()...");
    rt_common::threaded_scheduler_4_threads::client_server_block_on();
    print!("Succeeded.\n");
     print!("Testing function threaded_scheduler_1_thread::spawn_from_other_thread_idle()...");
    rt_common::threaded_scheduler_1_thread::spawn_from_other_thread_idle();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::block_on_sync()...");
    rt_common::threaded_scheduler_4_threads::block_on_sync();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::always_active_parker()...");
    rt_common::threaded_scheduler_4_threads::always_active_parker();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::shutdown_timeout()...");
    rt_common::threaded_scheduler_1_thread::shutdown_timeout();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_1_thread::shutdown_wakeup_time()...");
    rt_common::threaded_scheduler_1_thread::shutdown_wakeup_time();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::create_rt_in_block_on()...");
    rt_common::threaded_scheduler_4_threads::create_rt_in_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::eagerly_drops_futures_on_shutdown()...");
    rt_common::threaded_scheduler_4_threads::eagerly_drops_futures_on_shutdown();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::enter_and_spawn()...");
    rt_common::threaded_scheduler_4_threads::enter_and_spawn();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::io_driver_called_when_under_load()...");
    rt_common::threaded_scheduler_4_threads::io_driver_called_when_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::block_on_async()...");
    rt_common::threaded_scheduler_4_threads::block_on_async();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::io_notify_while_shutting_down()...");
    rt_common::threaded_scheduler_4_threads::io_notify_while_shutting_down();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::complete_block_on_under_load()...");
    rt_common::threaded_scheduler_4_threads::complete_block_on_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::local_set_client_server_block_on()...");
    rt_common::threaded_scheduler_4_threads::local_set_client_server_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::local_set_block_on_socket()...");
    rt_common::threaded_scheduler_4_threads::local_set_block_on_socket();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::complete_task_under_load()...");
    rt_common::threaded_scheduler_4_threads::complete_task_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::nested_rt()...");
    rt_common::threaded_scheduler_4_threads::nested_rt();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::panic_in_block_on()...");
    rt_common::threaded_scheduler_4_threads::panic_in_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::outstanding_tasks_dropped()...");
    rt_common::threaded_scheduler_4_threads::outstanding_tasks_dropped();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::shutdown_timeout_0()...");
    rt_common::threaded_scheduler_4_threads::shutdown_timeout_0();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::panic_in_task()...");
    rt_common::threaded_scheduler_4_threads::panic_in_task();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::runtime_in_thread_local()...");
    rt_common::threaded_scheduler_4_threads::runtime_in_thread_local();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::sleep_from_blocking()...");
    rt_common::threaded_scheduler_4_threads::sleep_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::coop()...");
    rt_common::threaded_scheduler_4_threads::coop();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::coop_unconstrained()...");
    rt_common::threaded_scheduler_4_threads::coop_unconstrained();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_await_chain()...");
    rt_common::threaded_scheduler_4_threads::spawn_await_chain();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::socket_from_blocking()...");
    rt_common::threaded_scheduler_4_threads::socket_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::sleep_at_root()...");
    rt_common::threaded_scheduler_4_threads::sleep_at_root();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_blocking_from_blocking()...");
    rt_common::threaded_scheduler_4_threads::spawn_blocking_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_from_blocking()...");
    rt_common::threaded_scheduler_4_threads::spawn_from_blocking();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::sleep_in_spawn()...");
    rt_common::threaded_scheduler_4_threads::sleep_in_spawn();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_from_other_thread_under_load()...");
    rt_common::threaded_scheduler_4_threads::spawn_from_other_thread_under_load();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_many_from_block_on()...");
    rt_common::threaded_scheduler_4_threads::spawn_many_from_block_on();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_one_bg()...");
    rt_common::threaded_scheduler_4_threads::spawn_one_bg();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_one_join()...");
    rt_common::threaded_scheduler_4_threads::spawn_one_join();print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::wake_while_rt_is_dropping()...");
    rt_common::threaded_scheduler_4_threads::wake_while_rt_is_dropping();
    print!("Succeeded.\n");
        
print!("Testing function threaded_scheduler_4_threads::spawn_two()...");
    rt_common::threaded_scheduler_4_threads::spawn_two();
    print!("Succeeded.\n");
        
print!("Testing function threaded_scheduler_4_threads::spawn_many_from_task()...");
    rt_common::threaded_scheduler_4_threads::spawn_many_from_task();
    print!("Succeeded.\n");
        
print!("Testing function threaded_scheduler_4_threads::shutdown_wakeup_time()...");
    rt_common::threaded_scheduler_4_threads::shutdown_wakeup_time();
    print!("Succeeded.\n");
        print!("Testing function threaded_scheduler_4_threads::shutdown_timeout()...");
    rt_common::threaded_scheduler_4_threads::shutdown_timeout();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::spawn_from_other_thread_idle()...");
    rt_common::threaded_scheduler_4_threads::spawn_from_other_thread_idle();
    print!("Succeeded.\n");
        

    print!("Testing function threaded_scheduler_4_threads::ping_pong_saturation()...");
    rt_common::threaded_scheduler_4_threads::ping_pong_saturation();
    print!("Succeeded.\n"); */
        

    /* print!("Testing function threaded_scheduler_1_thread::ping_pong_saturation()...");
    rt_common::threaded_scheduler_1_thread::ping_pong_saturation();
    print!("Succeeded.\n"); */
        

    println!("Done testing functions of mod rt_common.\n");
    
}

