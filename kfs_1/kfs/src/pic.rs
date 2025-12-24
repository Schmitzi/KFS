// pic.rs - Programmable Interrupt Controller (warnings fixed)

use core::arch::asm;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x11;
const ICW4_8086: u8 = 0x01;
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
    outb(0x80, 0);
}

pub fn remap() {
    unsafe {
        // Start initialization
        outb(PIC1_COMMAND, ICW1_INIT);
        io_wait();
        outb(PIC2_COMMAND, ICW1_INIT);
        io_wait();

        // Set vector offsets
        outb(PIC1_DATA, 32);  // IRQ0-7 -> INT 32-39
        io_wait();
        outb(PIC2_DATA, 40);  // IRQ8-15 -> INT 40-47
        io_wait();

        // Tell PICs about each other
        outb(PIC1_DATA, 4);
        io_wait();
        outb(PIC2_DATA, 2);
        io_wait();

        // Set 8086 mode
        outb(PIC1_DATA, ICW4_8086);
        io_wait();
        outb(PIC2_DATA, ICW4_8086);
        io_wait();

        // CRITICAL: Mask ALL except keyboard (IRQ1)
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
}

pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);
        }
        outb(PIC1_COMMAND, PIC_EOI);
    }
}
