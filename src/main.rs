#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"hello, world";
//操作系统入口
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    // vga_buffer::write_something();
    vga_buffer::WRITER.lock().write_str("hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        " , some number {}, {}",
        2,
        24.02977
    )
    .unwrap();
    loop {}
}

mod vga_buffer;
