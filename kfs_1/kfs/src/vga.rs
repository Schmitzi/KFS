use core::fmt;

// VGA text buffer is located at physical address 0xB8000
const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

// VGA cursor control ports
const VGA_CTRL_PORT: u16 = 0x3D4;
const VGA_DATA_PORT: u16 = 0x3D5;

// Color codes for VGA text mode
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

// Combines foreground and background colors into a single byte
pub const fn color_byte(fg: Color, bg: Color) -> u8 {
    (bg as u8) << 4 | (fg as u8)
}

// Writer for VGA text mode
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
                    *VGA_BUFFER.add(offset) = byte;          // Character
                    *VGA_BUFFER.add(offset + 1) = self.color; // Color
                }

                self.column += 1;
            }
        }
        self.update_cursor();
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // Printable ASCII or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // Not printable, use � character
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        self.column = 0;
        
        if self.row < VGA_HEIGHT - 1 {
            self.row += 1;
        } else {
            // At bottom of screen - scroll up
            self.scroll();
        }
    }

    // Scroll the screen up by one line
    fn scroll(&mut self) {
        unsafe {
            // Move each line up by one
            for row in 1..VGA_HEIGHT {
                for col in 0..VGA_WIDTH {
                    let src_offset = (row * VGA_WIDTH + col) * 2;
                    let dst_offset = ((row - 1) * VGA_WIDTH + col) * 2;
                    
                    // Copy character and color
                    *VGA_BUFFER.add(dst_offset) = *VGA_BUFFER.add(src_offset);
                    *VGA_BUFFER.add(dst_offset + 1) = *VGA_BUFFER.add(src_offset + 1);
                }
            }
            
            // Clear the last line
            for col in 0..VGA_WIDTH {
                let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + col) * 2;
                *VGA_BUFFER.add(offset) = b' ';
                *VGA_BUFFER.add(offset + 1) = self.color;
            }
        }
        
        // Stay on the last row
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

    // Update hardware cursor position
    fn update_cursor(&self) {
        let pos = self.row * VGA_WIDTH + self.column;
        
        unsafe {
            // Send high byte
            outb(VGA_CTRL_PORT, 0x0E);
            outb(VGA_DATA_PORT, (pos >> 8) as u8);
            
            // Send low byte
            outb(VGA_CTRL_PORT, 0x0F);
            outb(VGA_DATA_PORT, pos as u8);
        }
    }

    // Show or hide the cursor
    pub fn set_cursor_visible(&self, visible: bool) {
        unsafe {
            outb(VGA_CTRL_PORT, 0x0A);
            let cursor_start = if visible { 0x0E } else { 0x20 };
            outb(VGA_DATA_PORT, cursor_start);
        }
    }

    // Move cursor to specific position
    pub fn set_cursor_position(&mut self, row: usize, col: usize) {
        if row < VGA_HEIGHT && col < VGA_WIDTH {
            self.row = row;
            self.column = col;
            self.update_cursor();
        }
    }

    // Get current cursor position
    pub fn get_cursor_position(&self) -> (usize, usize) {
        (self.row, self.column)
    }
}

// Implement fmt::Write trait for use with write! macro
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// Port I/O functions for cursor control
#[inline]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

// Global writer instance
static mut WRITER: Writer = Writer::new();

// Print to VGA buffer (without newline)
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        let writer = &mut *core::ptr::addr_of_mut!(WRITER);
        writer.write_fmt(args).unwrap();
    }
}

// Get access to global writer for special operations
pub fn writer() -> &'static mut Writer {
    unsafe { &mut *core::ptr::addr_of_mut!(WRITER) }
}

// Macros for easy printing
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Kernel print (same as print, but sounds more kernel-y!)
#[macro_export]
macro_rules! printk {
    ($($arg:tt)*) => ($crate::print!($($arg)*));
}