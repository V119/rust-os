#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

static HELLO: &[u8] = b"hello, world";
//操作系统入口
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello word{}", "!");
    panic!("Some panic message");
    loop {}
}

mod vga_buffer;
