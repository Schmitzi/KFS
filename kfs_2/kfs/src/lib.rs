// lib.rs - Clean production version

#![no_std]
#![no_main]

use core::panic::PanicInfo;

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
    println!("KFS_2 - Kernel From Scratch");
    println!("===========================");
    println!();
    println!("A bare-metal i386 kernel written in Rust");
    println!();
    
    // Initialize system
    println!("=== Starting System initialization ===");
    println!("");
    vga::writer().set_color(vga::Color::Yellow, vga::Color::Black);
    println!("[1/4] Initializing IDT...");
    idt::init();
    vga::writer().set_color(vga::Color::Green, vga::Color::Black);
    println!("      IDT initialized!");
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("");
    vga::writer().set_color(vga::Color::Yellow, vga::Color::Black);
    println!("[2/4] Remapping PIC...");
    pic::remap();
    vga::writer().set_color(vga::Color::Green, vga::Color::Black);
    println!("      PIC Remapped!");  
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("");
    vga::writer().set_color(vga::Color::Yellow, vga::Color::Black);
    println!("[3/4] Initializing Interrupts...");
    idt::enable_interrupts();
    vga::writer().set_color(vga::Color::Green, vga::Color::Black);
    println!("      Interrupts loaded!");   
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("");
    vga::writer().set_color(vga::Color::Yellow, vga::Color::Black);
    println!("[4/4] Initializing GDT...");
    gdt::init();
    vga::writer().set_color(vga::Color::Green, vga::Color::Black);
    println!("      GDT loaded!");
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("");
    
    // Ready message
    vga::writer().set_color(vga::Color::Green, vga::Color::Black);
    println!("System initialized. Lets go!");
    println!();
    
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
