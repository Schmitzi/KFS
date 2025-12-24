// keyboard.rs - Simple test version (no naked functions, uses external asm)

use core::arch::asm;
use crate::pic;

const KEYBOARD_DATA_PORT: u16 = 0x60;

// Simple counter at a fixed screen position
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

// Write directly to VGA memory (bypass print! macro)
unsafe fn write_char_at(row: usize, col: usize, ch: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    let offset = (row * 80 + col) * 2;
    *vga_buffer.add(offset) = ch;
    *vga_buffer.add(offset + 1) = 0x0F; // White on black
}

// Rust handler - called from assembly wrapper in keyboard_int.asm
#[no_mangle]
pub extern "C" fn rust_keyboard_handler() {
    unsafe {
        // Read and discard scan code
        let _scancode = inb(KEYBOARD_DATA_PORT);
        
        // Send EOI immediately
        pic::send_eoi(1);
        
        // Increment counter and display it
        COUNTER += 1;
        if COUNTER > b'9' {
            COUNTER = b'0';
        }
        
        // Write counter to screen at position (10, 40)
        write_char_at(10, 40, COUNTER);
    }
}

// Note: keyboard_interrupt_handler is defined in keyboard_int.asm