// vga.rs - VGA text mode driver (warnings fixed)

use core::fmt;

const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

const VGA_CTRL_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;

#[allow(dead_code)]
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

pub const fn color_byte(fg: Color, bg: Color) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

pub struct Writer {
    column: usize,
    row: usize,
    color: u8,
}

impl Writer {
    pub const fn new() -> Writer {
        Writer {
            column: 0,
            row: 0,
            color: color_byte(Color::White, Color::Black),
        }
    }

    pub fn set_color(&mut self, fg: Color, bg: Color) {
        self.color = color_byte(fg, bg);
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column >= VGA_WIDTH {
                    self.new_line();
                }

                let offset = (self.row * VGA_WIDTH + self.column) * 2;

                unsafe {
                    *VGA_BUFFER.add(offset) = byte;
                    *VGA_BUFFER.add(offset + 1) = self.color;
                }

                self.column += 1;
            }
        }
        self.update_cursor();
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.column = 0;
        
        if self.row < VGA_HEIGHT - 1 {
            self.row += 1;
        } else {
            self.scroll();
        }
    }

    pub fn backspace(&mut self) {
        if self.column == 0 {
            return;
        }

        self.column -= 1;

        let offset = (self.row * VGA_WIDTH + self.column) * 2;

        unsafe {
            *VGA_BUFFER.add(offset) = b' ';
            *VGA_BUFFER.add(offset + 1) = self.color;
        }

        self.update_cursor();
    }

    fn scroll(&mut self) {
        unsafe {
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let src_offset = (row * VGA_WIDTH + col) * 2;
                    let dst_offset = ((row - 1) * VGA_WIDTH + col) * 2;
                    
                    *VGA_BUFFER.add(dst_offset) = *VGA_BUFFER.add(src_offset);
                    *VGA_BUFFER.add(dst_offset + 1) = *VGA_BUFFER.add(src_offset + 1);
                }
            }
            
            for col in 0..VGA_WIDTH {
                let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + col) * 2;
                *VGA_BUFFER.add(offset) = b' ';
                *VGA_BUFFER.add(offset + 1) = self.color;
            }
        }
        
        self.row = VGA_HEIGHT - 1;
        self.column = 0;
    }

    pub fn clear_screen(&mut self) {
        for row in 0..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let offset = (row * VGA_WIDTH + col) * 2;
                unsafe {
                    *VGA_BUFFER.add(offset) = b' ';
                    *VGA_BUFFER.add(offset + 1) = self.color;
                }
            }
        }
        self.column = 0;
        self.row = 0;
        self.update_cursor();
    }

    fn update_cursor(&self) {
        let pos = self.row * VGA_WIDTH + self.column;
        
        unsafe {
            outb(VGA_CTRL_PORT, 0x0E);
            outb(VGA_DATA_PORT, (pos >> 8) as u8);
            
            outb(VGA_CTRL_PORT, 0x0F);
            outb(VGA_DATA_PORT, pos as u8);
        }
    }

    pub fn set_cursor_visible(&self, visible: bool) {
        unsafe {
            outb(VGA_CTRL_PORT, 0x0A);
            let cursor_start = if visible { 0x0E } else { 0x20 };
            outb(VGA_DATA_PORT, cursor_start);
        }
    }

    // These methods are provided for future use
    #[allow(dead_code)]
    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        if row < VGA_HEIGHT && col < VGA_WIDTH {
            self.row = row;
            self.column = col;
            self.update_cursor();
        }
    }

    #[allow(dead_code)]
    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.row, self.column)
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

static mut WRITER: Writer = Writer::new();

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        let writer = &mut *core::ptr::addr_of_mut!(WRITER);
        writer.write_fmt(args).unwrap();
    }
}

pub fn writer() -> &'static mut Writer {
    unsafe { &mut *core::ptr::addr_of_mut!(WRITER) }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ($crate::print!($($arg)*));
}
