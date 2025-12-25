// pic.rs - Programmable Interrupt Controller (warnings fixed)

use core::arch::asm;
use crate::vga;
use crate::Color;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x11; // Initialization - required!
const ICW4_8086: u8 = 0x01; // 8086/88 (MCS-80/85) mode
const PIC_EOI: u8 = 0x20;

#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

unsafe fn io_wait() {
    // Wait a very small amount of time (1 to 4 microseconds, generally). 
    // Useful for implementing a small delay for PIC remapping on old 
    // hardware or generally as a simple but imprecise wait.
    // You can do an IO operation on any unused port: the Linux 
    // kernel by default uses port 0x80, which is often used during 
    // POST to log information on the motherboard's hex display but 
    // almost always unused after boot.
    outb(0x80, 0);
}

pub fn remap() {
    vga::writer().printc("[2/4] Remapping PIC...\n", Color::Yellow, Color::Black);
    unsafe {
        // Start initialization - ICW 1
        outb(PIC1_COMMAND, ICW1_INIT);
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT);
        io_wait();

        // Set vector offsets - ICW 2
        outb(PIC1_DATA, 32);  // IRQ0-7 -> INT 32-39
        io_wait();
        outb(PIC2_DATA, 40);  // IRQ8-15 -> INT 40-47
        io_wait();

        // Tell PICs about each other - ICW 3
        outb(PIC1_DATA, 4); 
        io_wait();
        outb(PIC2_DATA, 2);
        io_wait();

        // Set 8086 mode - ICW 4
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // MUY IMPORTANTE: Mask ALL except keyboard (IRQ1)
        // 0xFD = 11111101 binary
        // Bit 0 (IRQ0/Timer) = 1 (DISABLED)
        // Bit 1 (IRQ1/Keyboard) = 0 (ENABLED)
        // Bits 2-7 = 1 (DISABLED)
        outb(PIC1_DATA, 0xFD);
        io_wait();
        
        // Mask ALL on PIC2
        outb(PIC2_DATA, 0xFF);
        io_wait();
    }
    vga::writer().printc("      PIC Remapped!\n\n", Color::Green, Color::Black);
}

pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);
        }
        outb(PIC1_COMMAND, PIC_EOI);
    }
}
