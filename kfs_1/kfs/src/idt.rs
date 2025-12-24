// idt.rs - Interrupt Descriptor Table

use core::arch::asm;

// IDT entry structure
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low: u16,    // Lower 16 bits of handler address
    selector: u16,      // Code segment selector
    zero: u8,           // Always 0
    type_attr: u8,      // Type and attributes
    offset_high: u16,   // Upper 16 bits of handler address
}

impl IdtEntry {
    const fn new() -> IdtEntry {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            zero: 0,
            type_attr: 0,
            offset_high: 0,
        }
    }

    fn set_handler(&mut self, handler: unsafe extern "C" fn()) {
        let handler_addr = handler as usize;
        self.offset_low = (handler_addr & 0xFFFF) as u16;
        self.offset_high = ((handler_addr >> 16) & 0xFFFF) as u16;
        self.selector = 0x08; // Kernel code segment
        self.zero = 0;
        self.type_attr = 0x8E; // Present, DPL=0, 32-bit interrupt gate
    }
}

// IDT with 256 entries
#[repr(C, packed)]
struct Idt {
    entries: [IdtEntry; 256],
}

impl Idt {
    const fn new() -> Idt {
        Idt {
            entries: [IdtEntry::new(); 256],
        }
    }
}

// IDT pointer structure for lidt instruction
#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u32,
}

// Global IDT
static mut IDT: Idt = Idt::new();

// Initialize and load the IDT
pub fn init() {
    unsafe {
        // Set keyboard interrupt handler (IRQ1 = interrupt 33)
        IDT.entries[33].set_handler(keyboard_interrupt_handler);

        // Load IDT
        let idt_ptr = IdtPointer {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: &IDT as *const _ as u32,
        };

        asm!(
            "lidt [{}]",
            in(reg) &idt_ptr,
            options(readonly, nostack, preserves_flags)
        );
    }
}

// Enable interrupts
pub fn enable_interrupts() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}

// Keyboard interrupt handler (will be defined in keyboard module)
extern "C" {
    fn keyboard_interrupt_handler();
}