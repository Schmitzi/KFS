// lib.rs - Clean production version

#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
mod vga;
mod idt;
mod pic;
mod kb;
mod exceptions;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga::writer().set_color(vga::Color::White, vga::Color::Red);
    vga::writer().clear_screen();
    println!("KERNEL PANIC!");
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Clear screen
    vga::writer().clear_screen();
    vga::writer().set_cursor_visible(false);
    
    // Display mandatory "42"
    vga::writer().set_color(vga::Color::LightGreen, vga::Color::Black);
    println!("42");
    println!();
    
    // Welcome message
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("KFS_1 - Kernel From Scratch");
    println!("===========================");
    println!();
    println!("A bare-metal i386 kernel written in Rust");
    println!();
    
    // Initialize system
    idt::init();
    pic::remap();
    idt::enable_interrupts();
    
    // Ready message
    vga::writer().set_color(vga::Color::Yellow, vga::Color::Black);
    println!("System initialized. Keyboard ready!");
    println!();
    vga::writer().set_color(vga::Color::LightCyan, vga::Color::Black);
    println!("Start typing:");
    println!();
    
    // Main loop
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
