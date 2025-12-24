// keyboard.rs - Check status port to prevent interrupt storms

use core::arch::asm;
use crate::pic;

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

static mut COUNTER: u8 = b'0';

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

unsafe fn write_char_at(row: usize, col: usize, ch: u8, color: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    let offset = (row * 80 + col) * 2;
    *vga_buffer.add(offset) = ch;
    *vga_buffer.add(offset + 1) = color;
}

#[no_mangle]
pub extern "C" fn rust_keyboard_handler() {
    unsafe {
        // Check if there's actually data available
        let status = inb(KEYBOARD_STATUS_PORT);
        
        // Bit 0 = output buffer status (1 = full, data available)
        if (status & 0x01) == 0 {
            // No data available, spurious interrupt
            pic::send_eoi(1);
            return;
        }
        
        // Read scan code (this clears the keyboard buffer)
        let scancode = inb(KEYBOARD_DATA_PORT);
        
        // Send EOI immediately after reading
        pic::send_eoi(1);
        
        // Only process key press (not release)
        if scancode < 128 && scancode != 0 {
            // Increment and display counter
            COUNTER += 1;
            if COUNTER > b'9' {
                COUNTER = b'0';
            }
            
            // Display at row 12, col 40 (middle of screen)
            write_char_at(12, 40, COUNTER, 0x0F); // White on black
            
            // Also write the scancode in hex next to it for debugging
            let hex_hi = (scancode >> 4) & 0x0F;
            let hex_lo = scancode & 0x0F;
            let hi_char = if hex_hi < 10 { b'0' + hex_hi } else { b'A' + hex_hi - 10 };
            let lo_char = if hex_lo < 10 { b'0' + hex_lo } else { b'A' + hex_lo - 10 };
            
            write_char_at(12, 43, hi_char, 0x0E); // Yellow
            write_char_at(12, 44, lo_char, 0x0E);
        }
    }
}