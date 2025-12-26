// idt.rs - Complete IDT with all exception handlers

use core::arch::asm;
use crate::vga;
use crate::vga::Color;

// IDT entry structure
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_high: u16,
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
        self.selector = 0x08;
        self.zero = 0;
        self.type_attr = 0x8E;
    }
}

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

#[repr(C, packed)]
struct IdtPointer {
    limit: u16,
    base: u32,
}

static mut IDT: Idt = Idt::new();

// Import ALL handlers
extern "C" {
    fn kb_pic_handler();
    fn divide_by_zero_handler();
    fn invalid_opcode_handler();
    fn double_fault_handler();
    fn general_protection_fault_handler();
    fn page_fault_handler();
    fn default_interrupt_handler();
}

pub fn init() {
    vga::writer().printc("[1/4] Initializing IDT...\n", Color::Yellow, Color::Black);
    unsafe {
        // Exception handlers (0-31)
        IDT.entries[0].set_handler(divide_by_zero_handler);
        IDT.entries[6].set_handler(invalid_opcode_handler);
        IDT.entries[8].set_handler(double_fault_handler);
        IDT.entries[13].set_handler(general_protection_fault_handler);
        IDT.entries[14].set_handler(page_fault_handler);
        
        // Set default handler for ALL other interrupts (1-31, 32-255)
        // This catches timer, spurious interrupts, etc.
        for i in 1..256 {
            if i != 0 && i != 6 && i != 8 && i != 13 && i != 14 && i != 33 {
                IDT.entries[i].set_handler(default_interrupt_handler);
            }
        }
        
        // Keyboard interrupt (IRQ1 = interrupt 33)
        IDT.entries[33].set_handler(kb_pic_handler);

        // Load IDT
        let idt_ptr = IdtPointer {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: core::ptr::addr_of!(IDT) as u32,
        };

        asm!(
            "lidt [{}]",
            in(reg) &idt_ptr,
            options(readonly, nostack, preserves_flags)
        );
    }
    vga::writer().printc("      IDT initialized!\n\n", Color::Green, Color::Black);
}

pub fn enable_interrupts() {
    vga::writer().printc("[3/4] Initializing Interrupts...\n", Color::Yellow, Color::Black);
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
    vga::writer().printc("      Interrupts loaded!\n\n", Color::Green, Color::Black);
}
