// lib.rs - Clean production version

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga::Color;

#[macro_use]
mod vga;
mod idt;
mod pic;
mod kb;
mod exc;
mod gdt;
mod nps;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    vga::writer().set_color(vga::Color::White, vga::Color::Red);
    vga::writer().clear_screen();
    println!("KERNEL PANIC!");
    loop {}
}

fn init_and_print() {
    // Clear screen
    vga::writer().clear_screen();
    vga::writer().set_cursor_visible(true);
    
    
    // Welcome message
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("\nKFS_2 - Kernel From Scratch");
    println!("===========================\n");
    println!("A bare-metal i386 kernel written in Rust\n");
    
    // Initialize system
    println!("=== Starting System initialization ===\n");
    
    idt::init();
    pic::remap();
    idt::enable_interrupts();
    gdt::init();
    
    // Ready message
    vga::writer().printc("System initialized. Lets go!\n", Color::Green, Color::Black);
    
    // Enable NPS shell
    println!("");
    nps::init();
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    init_and_print();
    
    // Main loop
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}
