// gdt.rs - Global Descriptor Table implementation

use core::arch::asm;
use crate::vga;
use crate::vga::Color;

// GDT Entry structure (8 bytes)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct GdtEntry {
    limit_low: u16,    // Lower 16 bits of limit
    base_low: u16,     // Lower 16 bits of base
    base_middle: u8,   // Next 8 bits of base
    access: u8,        // Access flags
    granularity: u8,   // Granularity and limit bits 16-19
    base_high: u8,     // Last 8 bits of base
}

impl GdtEntry {
    // Create a null entry
    const fn null() -> GdtEntry {
        GdtEntry {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }

    // Create a GDT entry
    // base: Base address
    // limit: Limit (max offset)
    // access: Access byte
    // gran: Granularity byte
    const fn new(base: u32, limit: u32, access: u8, gran: u8) -> GdtEntry {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access,
            granularity: ((limit >> 16) & 0x0F) as u8 | (gran & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

// GDT Pointer structure for LGDT instruction
#[repr(C, packed)]
struct GdtPointer {
    limit: u16,  // Size of GDT - 1
    base: u32,   // Address of GDT
}

// The Global Descriptor Table (6 entries)
// Must be placed at 0x800 according to subject
static mut GDT: [GdtEntry; 6] = [
    // Null descriptor (required)
    GdtEntry::null(),
    
    // Kernel Code Segment (0x08)
    // Base=0, Limit=0xFFFFF, Access=0x9A, Granularity=0xC
    // Access: Present=1, DPL=0, Type=Code/Execute/Read
    // Gran: 4KB pages, 32-bit
    GdtEntry::new(0, 0xFFFFF, 0x9A, 0xC0),
    
    // Kernel Data Segment (0x10)
    // Base=0, Limit=0xFFFFF, Access=0x92, Granularity=0xC
    // Access: Present=1, DPL=0, Type=Data/Read/Write
    GdtEntry::new(0, 0xFFFFF, 0x92, 0xC0),
    
    // User Code Segment (0x18)
    // Base=0, Limit=0xFFFFF, Access=0xFA, Granularity=0xC
    // Access: Present=1, DPL=3 (user), Type=Code/Execute/Read
    GdtEntry::new(0, 0xFFFFF, 0xFA, 0xC0),
    
    // User Data Segment (0x20)
    // Base=0, Limit=0xFFFFF, Access=0xF2, Granularity=0xC
    // Access: Present=1, DPL=3 (user), Type=Data/Read/Write
    GdtEntry::new(0, 0xFFFFF, 0xF2, 0xC0),
    
    // Task State Segment (0x28) - minimal for now
    // Will be properly set up when we implement task switching
    GdtEntry::null(),
];

// External assembly function to load GDT
extern "C" {
    fn gdt_flush(gdt_ptr: *const GdtPointer);
}

// Initialize and load the GDT
pub fn init() {
    vga::writer().printc("[4/4] Initializing GDT...\n", Color::Yellow, Color::Black);
    unsafe {
        let gdt_ptr = GdtPointer {
            limit: (core::mem::size_of::<[GdtEntry; 6]>() - 1) as u16,
            base: &GDT as *const _ as u32,
        };
        
        gdt_flush(&gdt_ptr);
    }
    vga::writer().printc("      GDT loaded!\n\n", Color::Green, Color::Black);
}

// Print kernel stack information
pub fn print_stack() {
    unsafe {
        let esp: u32;
        let ebp: u32;
        
        // Get current stack and base pointers
        asm!(
            "mov {0}, esp",
            "mov {1}, ebp",
            out(reg) esp,
            out(reg) ebp,
        );

        println!("=== Kernel Stack Information ===");
        println!("Stack Pointer (ESP): 0x{:08x}", esp);
        println!("Base Pointer  (EBP): 0x{:08x}", ebp);
        println!();
        println!("Stack contents (top 16 dwords):");
        
        let stack_ptr = esp as *const u32;
        for i in 0..16 {
            let addr = esp + (i * 4);
            let value = *stack_ptr.offset(i as isize);
            println!("  0x{:08x}: 0x{:08x}", addr, value);
        }
        println!();
    }
}

// Print GDT information
pub fn print_gdt() {
    unsafe {
        println!("=== Global Descriptor Table ===");
        println!("GDT Address: 0x{:08x}", &GDT as *const _ as u32);
        println!("GDT Size: {} bytes", core::mem::size_of::<[GdtEntry; 6]>());
        println!();
        
        let entries = [
            "Null Descriptor",
            "Kernel Code",
            "Kernel Data", 
            "User Code",
            "User Data",
            "TSS",
        ];
        
        for (i, name) in entries.iter().enumerate() {
            let entry = &GDT[i];
            let base = (entry.base_low as u32) 
                     | ((entry.base_middle as u32) << 16)
                     | ((entry.base_high as u32) << 24);
            let limit = (entry.limit_low as u32) 
                      | (((entry.granularity & 0x0F) as u32) << 16);
            
            println!("[{}] {} (offset 0x{:02x}):", i, name, i * 8);
            println!("    Base:  0x{:08x}", base);
            println!("    Limit: 0x{:05x}", limit);
            println!("    Access: 0x{:02x}", entry.access);
            println!("    Gran:   0x{:02x}", entry.granularity);
        }
    }
}