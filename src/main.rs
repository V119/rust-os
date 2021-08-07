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

    // 触发中断
    blog_os::init();

    // 触发三重异常
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    // // 触发page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // }

    #[cfg(test)]
    test_main();

    println!("it did not crash");
    loop {
        use crate::print;
        print!("-");
    }
}
