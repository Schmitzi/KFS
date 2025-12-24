#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[macro_use]
mod vga;
mod idt;
mod pic;
mod kb;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Clear the screen
    vga::writer().clear_screen();
    
    // Make cursor visible
    vga::writer().set_cursor_visible(true);
    
    // Display mandatory "42"
    vga::writer().set_color(vga::Color::LightGreen, vga::Color::Black);
    println!("42");
    println!();
    
    // Welcome message
    vga::writer().set_color(vga::Color::White, vga::Color::Black);
    println!("KFS_1 Kernel - Keyboard Input Demo");
    println!("===================================");
    println!();
    
    // Initialize interrupts
    println!("Initializing IDT...");
    idt::init();
    
    println!("Configuring PIC...");
    pic::remap();
    
    println!("Enabling interrupts...");
    idt::enable_interrupts();
    
    println!();
    vga::writer().set_color(vga::Color::Yellow, vga::Color::Black);
    println!("Keyboard ready! Start typing:");
    vga::writer().set_color(vga::Color::LightCyan, vga::Color::Black);
    println!();
    
    // Halt forever (interrupts will wake us up)
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}