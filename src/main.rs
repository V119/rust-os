#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{fmt::Write, panic::PanicInfo};

pub mod serial;
pub mod vga_buffer;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(_info);
}

static HELLO: &[u8] = b"hello, world";

//操作系统入口
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello word{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}
