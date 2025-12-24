// exceptions.rs - Complete exception handlers

pub mod exceptions {
    const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
    
    unsafe fn write_error(row: usize, msg: &str, color: u8) {
        for (i, byte) in msg.bytes().enumerate() {
            let offset = (row * 80 + i) * 2;
            *VGA_BUFFER.add(offset) = byte;
            *VGA_BUFFER.add(offset + 1) = color;
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_divide_by_zero() {
        unsafe {
            write_error(10, "EXCEPTION #0: DIVIDE BY ZERO", 0x4F);
            loop {}
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_invalid_opcode() {
        unsafe {
            write_error(10, "EXCEPTION #6: INVALID OPCODE", 0x4F);
            loop {}
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_double_fault() {
        unsafe {
            write_error(10, "EXCEPTION #8: DOUBLE FAULT (STACK OVERFLOW?)", 0x4F);
            loop {}
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_general_protection_fault() {
        unsafe {
            write_error(10, "EXCEPTION #13: GENERAL PROTECTION FAULT", 0x4F);
            loop {}
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_page_fault() {
        unsafe {
            write_error(10, "EXCEPTION #14: PAGE FAULT", 0x4F);
            loop {}
        }
    }
    
    #[no_mangle]
    pub extern "C" fn rust_default_interrupt() {
        unsafe {
            write_error(10, "UNHANDLED INTERRUPT!", 0x4F);
            loop {}
        }
    }
}
