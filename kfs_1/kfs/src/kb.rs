// kb.rs - Keyboard handler (warnings fixed)

use crate::pic;

const KEYBOARD_DATA_PORT: u16 = 0x60;

// Complete scan code to ASCII lookup table (US QWERTY, lowercase)
static SCANCODE_TO_ASCII: [u8; 128] = [
    0,    27,  b'1', b'2', b'3', b'4', b'5', b'6',  // 0x00-0x07
    b'7', b'8', b'9', b'0', b'-', b'=', 8,   b'\t', // 0x08-0x0F (backspace, tab)
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', // 0x10-0x17
    b'o', b'p', b'[', b']', b'\n', 0,   b'a', b's', // 0x18-0x1F (enter, ctrl)
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', // 0x20-0x27
    b'\'', b'`', 0,   b'\\', b'z', b'x', b'c', b'v', // 0x28-0x2F (shift)
    b'b', b'n', b'm', b',', b'.', b'/', 0,   b'*',  // 0x30-0x37 (shift, *)
    0,    b' ', 0,   0,   0,   0,   0,   0,          // 0x38-0x3F (alt, caps, F1-F5)
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x40-0x47 (F6-F10, num lock, scroll lock)
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x48-0x4F
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x50-0x57
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x58-0x5F
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x60-0x67
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x68-0x6F
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x70-0x77
    0,    0,   0,   0,   0,   0,   0,   0,           // 0x78-0x7F
];

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") value,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    value
}

#[no_mangle]
pub extern "C" fn rust_keyboard_handler() {
    unsafe {
        // Read scan code to clear keyboard buffer
        let scancode = inb(KEYBOARD_DATA_PORT);
        
        // Send EOI to acknowledge interrupt
        pic::send_eoi(1);
        
        // Only process key presses (not releases)
        if scancode >= 128 || scancode == 0 {
            return;
        }
        
        // Convert scan code to ASCII
        let ascii = SCANCODE_TO_ASCII[scancode as usize];
        
        // Print character if printable
        if ascii != 0 {
            // Use print! macro from the prelude
            crate::print!("{}", ascii as char);
        }
    }
}
