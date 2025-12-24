# kfs_2

[Introduction](#introduction)<br>
[What we want to achieve](#what-we-want-to-achieve)<br>
[The Fun Stuff](#the-fun-stuff)<br>

## Introduction

On todays episode on ~~~"Why the hell do I do this to myself"~~~ "I'm having so much fun", we are going to learn how to code a stack and integrate it with GDT

Now your'e probably asking yourself "But Michael, I have a stack", "What's GDT?" and maybe "It's 5am, can I go to sleep yet?".

Firstly we did include a stack in our bootloader. Check ✓

Secondly, GDT stands for "Global Descriptor Table". It is a data structure used to define the different memory areas: the base address, the size and access privileges like execute and write.

And lastly, no.....of course we don't sleep, this is way too much fun

These memory areas are called ```segments```

In a GDT, you can find:

- Kernel code, used to store the executable binary code
- Kernel data
- Kernel stack, used to store the call stack during kernel execution
- User code, used to store the executable binary code for user programs
- User program data
- User stack, used  to store the call stack during execution in userland

Next, the stack. As stack is defined as:

*an abstract data type that serves as a collection of elements, with two principal operations: push, which adds an element to the collection, and pop, which removes the most recently added element that was not yet removed. The order in which elements come off a stack gives rise to its alternative name, LIFO (for last in, first out).*

## What we want to achieve

- ***```create```, ```fill``` and ```link```*** - Build a Global Descriptor Table into the Kernel
    The GDT must contain:
    - Kernel Code
    - Kernel Data
    - Kernel stack
    - User code
    - User data
    - User stack

    GDT must be declared to the BIOS and set to address ```0x00000800```
    Then we need to code a tool to print the kernel stack in a human-friendly way. We have already built printk() so thats great.

- ***Understand how memory works*** - Understand how the stack and RAM work, how to use it, how to fill it and how to link it with the BIOS

***REMEMBER TO KEEP IT UNDER 10MB***

## The Fun Stuff

Because we have keyboards working, lets build a Shell (prototype name: NPS - Not a POSIX Shell)

It its suggested to add the print-kernel-stack function, ```reboot```, ```halt``` and the like.

# Michael's Famous InfoDump

[GDT - Global Descriptor Table](#gdt---global-descriptor-table)

# GDT - Global Descriptor Table

The ***Global Descriptor Table*** is a binary structure specific to ```IA-32``` and ```x86_64``` architectures, and more precisely in Protected Mode or Long Mode, Interrupt Service Routines and a good deal of memory management are controlled through tables of descriptor. It contains entries telling the CPU about memory segments. A similar ```Interrupt Descriptor Table``` exists containing task and interrupt descriptors.

The ***GDT*** is pointed to by the value in the ***GDTR*** register. This is loaded using the ***LGDT*** assembly instruction, whose argument is a pointer to a ***GDT Descriptor*** structure: 

***GDT Descriptor (GDTR)***

|79 (64-bit mode)   |         |
|47 (32-bit mode)   |15     0|
|:-------------------|-------:|
| ***Offset***      |Size    |
|63 (64-bit mode)   |        |
|31 (32-bit mode)   |15     0|

- ***Size***: The size of the table in bytes subtracted by 1. This subtraction occurs because the maximum value of Size is 65535, while the GDT can be up to 65536 bytes in length (8192 entries). Further, no GDT can have a size of 0 bytes.

- ***Offset***: The linear address of the GDT (not the physical address, paging applies).

Note that the amount of data loaded by LGDT differs in 32-bit and 64-bit modes, the offset is 4 bytes long in 32-bit mode and 8 bytes long in 64-bit mode. 

Each descriptor stores information about a single object (e.g. a service routine, a task, a chunk of code or data) the CPU might need at some time. If you try, for instance, to load a new value into a Segment Register, the CPU needs to perform safety and access checks to see whether you're actually entitled to access that specific memory area. Once the checks are performed, useful values (such as the lowest and the highest addresses) are cached in invisible CPU registers.

On these architectures, there are three types of table: The Global Descriptor Table, The Local Descriptor Table and the Interrupt Descriptor Table (which supplants the Interrupt Vector Table). Each table is defined using size and linear address to the CPU thriugh the LGDT, LLDT and LIDT instructions respectively. In almost all cases, these tables are only placed inti memory once, at boot time, and then edited later when needed.

## Glossary

- ***Segment***
    A locally contiguous chuck of memory with consistent properties (from the CPUs perspective)

- ***Segment Register***
    A register of your CPU that refers to a segment for a particular purpose (***CS, DS, SS, ES***)

- ***Segment Selector***
    A reference to a descriptor, which you can load into a segment register; the sector is an offset into a descriptor table pointing to one of its entries. These entries are typically 8 bytes long, therefore bits 3 and up only declare the descriptor table entry offset, while bit 2 specifies if this selector is a GDT of LDT selector (LDT-bit set, GDT-bit cleared), and bits 0 - 1 declare the ring level that needs to correspond to the descriptor table entry's DPL field. If it doesn't, a General Protection Fault occurs; if it does correspond then the CPL level of the selector used is changed accordingly.

- ***Segment Descriptor***
    An entry in a description table. These are a binary data structure that tells the CPU the attributes of a given segment

## Create a GDT

GRUB already sets up a basic GDT, thats why it boots at all. But we want to implement our own to understand memory segmentation.

## The Flat Memory Model

For modern i386 kernels in Rust, we'll use the "Flat/Long Mode Setup" from the OSDev documents.This means:

- All segments cover the full ***4GB*** address space
- Base = 0, Limit = 0xFFFFF (with granularity flag = 4KB pages)
- Segmentation is effectively disabled (rely on paging instead)

Your GDT should look like this:

| Offset | Segment                | Base | Limit         | Access | Flags |
|--------|------------------------|------|---------------|--------|-------|
| 0x00   | Null Descriptor        | 0    | 0             | 0x00   | 0x0   |
| 0x08   | Kernel Code            | 0    | 0xFFFFF       | 0x9A   | 0xC   |
| 0x10   | Kernel Data            | 0    | 0xFFFFF       | 0x92   | 0xC   |
| 0x18   | User Code              | 0    | 0xFFFFF       | 0xFA   | 0xC   |
| 0x20   | User Data              | 0    | 0xFFFFF       | 0xF2   | 0xC   |
| 0x28   | TSS (optional for now) | &TSS | sizeof(TSS)-1 | 0x89   | 0x0   |

## GDT Location: 0x00000800

We have to set GDT at this specific address

```rust
// Place GDT at 0x800
static mut GDT: [GdtEntry; 6] = [/* your entries */];

// You can also use the linker to do this
```

## 1. What the GDT Is (OSDev summary, simplified)

The Global Descriptor Table (GDT) tells the CPU how memory segments behave in protected mode.

Each entry (descriptor) defines:
- Where a segment starts (base)
- How big it is (limit)
- What it’s allowed to do (access byte)
- How addressing works (granularity)

Even though modern kernels use paging, a valid GDT is still mandatory on i386.

---

## 2. GDT Entry Layout (matches OSDev exactly)

Each GDT entry is 8 bytes (64 bits):

| Bits            | Fields                                    |
|-----------------|--------------------------------------------|
| 31 – 24         | Base 31:24                                 |
| 23 – 20         | Flags (G, D/B, L, AVL)                     |
| 19 – 16         | Limit 19:16                                |
| 15 – 8          | Access Byte                                |
| 7 – 0           | Base 23:16                                 |
| 15 – 0          | Base 15:0 / Limit 15:0 (lower dwords)      |



Your Rust struct maps 1:1 to this layout:

```rust
#[repr(C, packed)]
pub struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}
```
✅ #[repr(C, packed)] is required so Rust doesn’t reorder or pad fields.


## 3.  Why the Null Descriptor Is Required

```rust
GdtEntry::null()
```

- Index 0 must always be zero
- Segment selector 0x00 is invalid by design
- The CPU uses this to detect errors

If you remove this → ***triple fault***

## 4. Segment Selectors & Offsets

Each descriptor is 8 bytes, so selectors are:

| Selector	| Meaning     |
|-----------|-------------|
| 0x00	    | Null        |
| 0x08	    | Kernel Code |
| 0x10	    | Kernel Data |
| 0x18	    | User Code   |
| 0x20	    | User Data   |
| 0x28	    | TSS         |

That’s why your ASM uses:

```asm
mov ax, 0x10   ; kernel data
jmp 0x08:.flush ; kernel code
```

Selector = ```index * 8```


## 5. Access Byte (Most Important Part)

From OSDev, the access byte layout:

```bash
Bit:  7   6 5   4   3    2    1   0
      P | DPL | S | E | DC | RW | A
```

### Kernel Code (```0x9A``` = ```10011010```)

- Present = 1
- DPL = 0 (ring 0)
- Code segment
- Executable
- Readable

```rust
GdtEntry::new(0, 0xFFFFF, 0x9A, 0xC0)
```

### Kernel Data (```0x92```)
- Present
- Ring 0
- Data segment
- Read/Write

### User Code (0xFA)
- Same as kernel code
- DPL = 3 → user mode allowed

### User Data (0xF2)
- Same as kernel data
- DPL = 3

✅ These values exactly match OSDev’s recommended setup.


## 6. Granularity Byte

Granularity byte layout:

```bash
Bits:  7   6   5   4 | 3 2 1 0
       G | D | L | A | Limit
```

You use:
```rust
granularity: ((limit >> 16) & 0x0F) | (gran & 0xF0)
```

And pass:
```rust
0xC0 = 11000000
```

***Meaning:***
- G = 1 → limit is in 4 KB pages
- D = 1 → 32-bit protected mode
- Limit becomes:
    ```0xFFFFF * 4 KB ≈ 4 GB```

This creates a ***flat memory model***.


## 7. Why Base = 0 and Limit = 0xFFFFF

This is the classic ***flat segmentation model***:
- All segments start at 0
- All segments span the full address space
- Paging (later) handles isolation

OSDev explicitly recommends this.

## 8. GDT Pointer (LGDT Format)

LGDT expects ***exactly this structure***:

```rust
#[repr(C, packed)]
struct GdtPointer {
    limit: u16,
    base: u32,
}
```
- ```limit = sizeof(GDT) - 1```
- ```base = address of GDT``

Your code does this correctly:

```rust
limit: (size_of::<[GdtEntry; 6]>() - 1) as u16,
base: &GDT as *const _ as u32,
```

## 9. Loading the GDT (ASM side)

From OSDev’s canonical sequence:

```asm
lgdt [gdt_ptr]
mov ax, data_selector
mov ds, ax
...
jmp code_selector:flush
```

Your gdt_flush follows this exactly:

```asm
lgdt [eax]

mov ax, 0x10
mov ds, ax
mov es, ax
mov fs, ax
mov gs, ax
mov ss, ax

jmp 0x08:.flush
```

Why the far jump?
- Reloads CS
- Flushes the instruction pipeline
- Required after changing the GDT

## 10. Why the TSS Is Null (For Now)
```rust
GdtEntry::null(), // TSS placeholder
```

OSDev notes:
- TSS is not required unless you do:
    - Hardware task switching
    - Ring 3 → Ring 0 stack switching

You correctly reserve the slot now so:
- Offsets stay stable
- You can add it later without breaking ABI