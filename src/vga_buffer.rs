use core::fmt::{self, Write};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGH: usize = 25;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGH],
}

pub struct Writer {
    colume_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.colume_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGH - 1;
                let col = self.colume_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                self.colume_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20...0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGH {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGH - 1);
        self.colume_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: ColorCode::new(Color::Black, Color::Black),
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// pub fn write_something() {
//     let mut writer = Writer {
//         colume_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Blue),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     };

//     writer.write_byte(b'h');
//     writer.write_string("ello");
//     writer.write_string("Wörld!");
//     write!(writer, "Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin`
//     ^Cqemu-system-x86_64: terminating on signal 2 from pid 60304 (<unknown process>)").unwrap();
//     writer.write_string("Running: `qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin`
//     ^Cqemu-system-x86_64: terminating on signal 2 from pid 60304 (<unknown process>)");
// }

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        colume_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Blue),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
