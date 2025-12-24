// pic.rs - Programmable Interrupt Controller (8259 PIC)

use core::arch::asm;

// PIC ports
const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

// PIC commands
const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;
const PIC_EOI: u8 = 0x20;

// Port I/O functions
#[inline]
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags)
    );
}

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

// Small delay for PIC
unsafe fn io_wait() {
    outb(0x80, 0);
}

// Remap the PIC to avoid conflicts with CPU exceptions
pub fn remap() {
    unsafe {
        // Save masks
        let mask1 = inb(PIC1_DATA);
        let mask2 = inb(PIC2_DATA);

        // Start initialization sequence
        outb(PIC1_COMMAND, ICW1_INIT);
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT);
        io_wait();

        // Set vector offsets
        // Map IRQ0-7 to interrupts 32-39
        // Map IRQ8-15 to interrupts 40-47
        outb(PIC1_DATA, 32);
        io_wait();
        outb(PIC2_DATA, 40);
        io_wait();

        // Tell PICs about each other
        outb(PIC1_DATA, 4);  // PIC2 at IRQ2
        io_wait();
        outb(PIC2_DATA, 2);  // Cascade identity
        io_wait();

        // Set 8086 mode
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // Restore masks (except keyboard - IRQ1)
        outb(PIC1_DATA, mask1 & 0xFD); // Enable IRQ1 (keyboard)
        outb(PIC2_DATA, mask2);
    }
}

// Send End of Interrupt signal
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);
        }
        outb(PIC1_COMMAND, PIC_EOI);
    }
}