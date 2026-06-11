#![no_std]

extern crate zephyr;

#[no_mangle]
extern "C" fn rust_main() {
    unsafe {
        zephyr::set_logger().ok();
    }

    let _ = poc_adapter::log_info("hello from Rust through C++ ABI");
    let _ = poc_adapter::uptime_ms();
}
