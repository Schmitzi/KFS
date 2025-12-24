# KFS_1 - Complete Learning Notes & Documentation

This document contains all the learning materials, tutorials, and detailed explanations used while building the KFS_1 kernel. This is for personal reference and deep learning.

**For the project README, see:** [README.md](README.md)

---

## Table of Contents

- [Project Overview](#project-overview)
  - [Introduction](#introduction)
  - [Objectives](#objectives)
  - [Instructions](#instructions)
  - [Compilation](#compilation)
  - [Mandatory Requirements](#mandatory)
  - [Bonus Features](#non-mandatory-part)
- [Knowledge Base](#knowledge-dump)
- [Rust Bare-Metal Tutorial](#freestanding-rust-binary)
- [A Minimal Rust Kernel](#a-minimal-rust-kernel)
- [Writing a Bootloader](#writing-a-bootloader)
- [Linker Scripts Explained](#what-is-a-linker-script)
- [VGA Text Mode](#vga-text-mode-guide)
- [Print Macros](#print-macros-guide)
- [Cursor and Scroll Support](#cursor-and-scroll-support-guide)
- [Keyboard Input - Complete Guide](#keyboard-input-complete-guide)
- [Interrupt Handling Deep Dive](#interrupt-handling-deep-dive)
- [Exception Handlers](#exception-handlers)
- [Debugging Interrupt Storms](#debugging-interrupt-storms)
- [Fixing the Reboot Issue](#fixing-the-reboot-issue)
- [Final Polishing](#final-polishing)
- [Learning Resources](#sources--learning-resources)

---

# Project Overview

## Introduction

This is it. Kernel From Scratch. We are actually building a kernel without any existing 
software, API or anything

There are many parts to this project and they are all linked together. So as we build these
features, keep in mind that the kernel must be flexible and that functions must easily fit 
in. Half of the time you'll spend on these projects will be adding links between different
aspects of your kernel.

That means, you have to write memory code before processus & execution code. But processus
must use memory, right? So those two have to be linked! That means, keep it ***clean***,
and the internal API simple.

# Objectives

By the end of this journey, you will have:

- A kernel you can boot via ```GRUB```
- An ```ASM``` bootable base
- A basic kernel library, with basic functions and types
- Some basic code to print some stuff on the screen
- A basic "Hello World' kernel
- **Full keyboard input via hardware interrupts**
- **Exception handling to catch CPU faults**
- **Robust interrupt system**

# Instructions

## Code and Execution

It is advised to use ```KVM```, the Kernel Virtual Manager. It has advanced execution and
debug functions. We used QEMU for testing.

## Language

Kernel From Scratch allows you to pick your own language. Although the current kernel is
mostly C, you are free to build it in whatever you want. So of course we picked Rust 🦀

Do remember that because there is a lot of documentation in C, we want to take on this
challenge of 'code translation'. Also, we are not allowed to use any of the language 
features in a basic kernel. For instance, C++ uses 'new' to make allocation, class and
structure declaration. We do not have a memory interface (yet).

# Compilation

## Compilers

We will be using cargo to make our kernel

## Flags

There are several flags that need to be used. In C++ they are:
- -fno-builtin
- -fno-exception
- -fno-stack-protector
- -fno-rtti
- -nostdlib
- -nodefaultlibs

The kernel will be compiled on a host system but cannot be linked to any existing library
on that host.

## Linking

You cannot use an existing linker in order to link your kernel. As written above, your kernel will not boot. That means we have to create a linker for your kernel.

***BEWARE*** you CAN use the 'ld' binary available on your host, but you CANNOT use there .ld file of your host

## Architecture

Its mandatory to build an i386 (x86) architecture

## Documentation

Check out the [OSDev](http://wiki.osdev.org/Main_Page) wiki

# Mandatory

## Base

The kernel must be able to boot with ```GRUB```, that can write characters on screen.

That means we have to:

- Install ```GRUB``` on a virtual image
- Write an ```ASM``` boot code that handles multiboot header, and use ```GRUB``` to
init and call the main function of the kernel itself.
- Compile it with correct flags, and link it to make it bootable
- Write helpers like kernel types or basic functions (strlen, strcmp, ...)
- Must be smaller than 10MB
- Code the interface between the kernel and the screen.
- Display "42" on the screen

For the linker, you must create a linker file with the [GNU linker (ld)](http://www.math.utah.edu/docs/info/ld_3.html#SEC4). 

## Makefile

The Makefile must compile all the source files with the right flags and the right compiler.
Keep in mind that the kernel will use at least two different languages (ASM and Rust), so
make sure the Makefile rules are correct

After compilation, all of the objects must be linked together in order to create the final
Kernel binary (Cf. Linker part)

# Non Mandatory part

This part is not necessary but we implemented all of it:

- ✅ Add scroll and cursor support to the I/O interface.
- ✅ Add colors support to the I/O interface.
- ✅ Add helpers like printf/printk to print and debug easier.
- ✅ Handle keyboard entries (interrupts) and print them.
- Handle different screen and keyboard shortcuts to switch easily between them.

# Knowledge Dump

## What is a Kernel

The kernel is the hidden piece of the operating system that enables any other programs to execute. It handles events generated by hardware (called ``interrupts``) and software (called system calls), and manages access to resources.

The hardware event handlers (interrupt handlers) will for instance get the number of key you just pressed, and convert it to the corresponding character stored in a buffer so some program can retrieve it.

The system calls are initiated by user-level programs, for opening files, starting other programs, etc. Each system call handler will have to check whether the argument  passed are valid, then perform the internal operation to complete the request.

Most user programs do not directly issue system calls (except for ```ASM``` programs, for instance), but instead use a ```standard library``` which does the ugly job of formatting arguments as required by the kernel and generating the system call (For example, the C function ```fopen()``` eventually calls a kernel function that actually opens the file)

The kernel usually defines a few ```abstractions``` like files, processes, sockets, directories, etc. which correspond to an internal state it remembers about last operations, so that a program may issue a session of operation more efficiently.

# Freestanding Rust Binary

[Introduction](#introduction)
[Disabling the Standard Library](#disabling-the-standard-library)
[The no_std Attribute](#the-no_std-attribute)
[Panic Implementation](#panic-implementation)
[Disabling Unwinding](#disabling-unwinding)
[What do you do without a main()?](#what-do-you-do-without-a-main)
[Linker Errors](#linker-errors)
[Building for a Bare Metal Target](#building-for-a-bare-metal-target)
[End Result](#end-result)

## Introduction
So the first real step we will take is creating a Rust executable that does not link to the standard library. That way we can run Rust code on ```bare metal``` without an underlying operating system.

This means we will not have threads, files, heap memory, the network, random numbers, standard output or any features requiring OS abstractions or specific hardware. We have to make all of these on our own.

However, there are some functions we can use. For example, we can use iterators, closures, pattern matching, option and result, string formatting and the ownership system. These features make it possible to write a kernel in a very expressive, high level way without worrying about undefined behavior or memory safety.

All of this means we need to create a freestanding or bare metal executable.

## Disabling the Standard Library
By default, all Rust crates link the standard library, which depends on the operating system for features such as threads, files, or networking. It also depends on the C standard library libc, which closely interacts with OS services.

We create a new cargo application like this

```bash
cargo new kfs --bin --edition 2024
```

To break that down:
- kfs - name of the crate
- --bin - specifies that we want to create an executable binary (not a library)
- --edition 2024 - specifies that we want to use the 2024 edition of Rust for our crate

Our structure
```bash
kfs
├── Cargo.toml
└── src
    └── main.rs
```

The Cargo.toml contains the crate configuration, for example the crate name, the author, the semantic version number, and dependencies.

The src/main.rs file contains the root module of our crate and our main function. You can compile your crate through ```cargo build``` and them run the compiled ```kfs``` binary in the ```target/debug``` subfolder.

## The no_std Attribute

As it stands, our crate links the standard library

We can disable that by changing the src/main.rs file like so

```rust
#![no_std]  <- Add this

fn main() {
    println!("Hello, world!");
}
```

You'll now see an error when you run cargo build because println! requires the standard library.

## Panic Implementation

The panic_handler attribute defines the function that the compiler should invoke when a panic occurs. The standard library provides its own panic handler function, but in a no_std environment we have to define it ourselves.

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

The PanicInfo parameter contains the file and line where the panic happened and the optional panic message. The function should never return, so it is marked as a diverging function by returning the "never" type !. There is not much we can do in this function for now, so we just loop indefinitely.

## Disabling Unwinding

There are several reasons to do this so Rust gives us an option to disable it. We just need to add these to our Cargo.toml

```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```
This sets the panic strategy to abort for both the dev and the release profile.

## What do you do without a main()?

A main function is the entrypoint to a program and is the stepping off point for all further execution. However most languages have a runtime system, which is responsible for things such as garbage collection or software threads. This runtime needs to be called before main() to initialize itself.

In a typical binary with stdlib, execution starts in a C runtime library called ```crt0``` (C runtime zero), which sets up the environment for a C application. this includes creating a stack and placing the arguments in the right registers. The C runtime then invokes the entry point of the Rust runtime, which is marked by the start language item. Rust only has a very minimal runtime, which takes care of some small things such as setting up stack overflow guards or printing a backtrace on panic, The runtime them finally calls the main function.

We dont have access to a runtime or crt0, so we need to define our own entry point. Implementing the start language item wouldn't help, since it would still require crt0. Instead, we need to overwrite the crt0 entry point directly.

## Overwriting the Entry Point

To do this we tell the Rust compiler that we dont want to use the normal entry point chain, we add the #![no_main] attribute

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
```

Now we need to make our own _start function (or kernel_main in our case)

```rust
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    loop {}
}
```

By using the no_mangle attribute, we disable name mangling to ensure that the Rust compiler really outputs a function with the name we specify. Without the attribute, the compiler would generate some cryptic symbol to give every function a unique name. The attribute is required because we need to tell the name of the entry point function to the linker in the next step.

Marking the function as extern 'C' tells the compiler that it should use the C calling convention for this function (instead of the Rust calling convention)

The ! return type means that the function is diverging, i.e. not allowed to ever return. This is required because the entry point is not called by any function, but invoked directly by the operating system or bootloader. So instead of returning, the entry point should e.g. invoke the exit system call of the operating system. In our case, shutting down the machine could be a reasonable action, since there's nothing left to do if a freestanding binary returns. For now, we fulfill the requirement by looping endlessly.

## Building for a Bare Metal Target

By default, Rust tries to build an executable that is able to run in your current system environment. To avoid linker errors, we can compile for a different environment with no underlying operating system.

```bash
rustup run nightly cargo build -Z build-std=core --target i386-unknown-none.json
```

By passing a --target argument we cross compile our executable for a bare metal target system. Since the target system has no operating system, the linker does not try to link the C runtime and our build succeeds without any linker errors.

## End Result

A minimal freestanding Rust binary looks like this:

src/lib.rs:

```rust
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Your kernel code here
    loop {}
}
```

Cargo.toml

```toml
[package]
name = "kfs"
version = "0.1.0"
edition = "2021"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[lib]
crate-type = ["staticlib"]

[dependencies]
```

# A Minimal Rust Kernel

[The Boot Process](#the-boot-process)
[BIOS Boot](#bios-boot)
[The Multiboot Standard](#the-multiboot-standard)

## The Boot Process

When you turn on a computer, it begins executing firmware code that is stored in motherboard ROM. This code performs a power-on self-test, detects available RAM, and pre-initializes the CPU and hardware. Afterwards, it looks for a bootable disk and starts booting the operating system kernel.

On x86, there are two firmware standards: the "Basic Input/Output System" (BIOS) and the newer "Unified Extensible Firmware Interface" (UEFI). The BIOS standard is old and outdated, but simple and well-supported on any x86 machine since the 1980s.

## BIOS Boot

Almost all x86 systems have support for BIOS booting. When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self-test and initialization routines of the hardware, then it looks for bootable disks.

The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit real mode first to the 32-bit protected mode, and then to the 64-bit long mode, where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.

## The Multiboot Standard

To avoid that every operating system implements its own bootloader, the Free Software Foundation created an open bootloader standard called Multiboot in 1995. The standard defines an interface between the bootloader and the operating system, so that any Multiboot-compliant bootloader can load any Multiboot-compliant operating system. The reference implementation is GNU GRUB, which is the most popular bootloader for Linux systems.

To make a kernel Multiboot compliant, one just needs to insert a so-called Multiboot header at the beginning of the kernel file.

# Writing a Bootloader

## What is Multiboot?

**The Problem:**
Every OS needs a bootloader to load it into memory and start execution. Without a standard, every OS would need its own custom bootloader.

**The Solution:**
Multiboot is a **specification** that says:
- "If your kernel has THIS specific header..."
- "...then ANY Multiboot-compliant bootloader (like GRUB) can load it"

**What GRUB Does:**
1. GRUB scans the first 8KB of your kernel file
2. Looks for the magic number `0x1BADB002`
3. Reads the Multiboot header to understand what the kernel needs
4. Loads the kernel into memory at address 1MB (0x00100000)
5. Switches CPU to 32-bit protected mode
6. Jumps to your kernel's entry point (`_start`)

## Assembly Language Basics

### What is Assembly?
Assembly is the lowest-level human-readable programming language. It maps almost 1:1 with CPU machine instructions.

### Key Assembly Concepts:

**Registers** - Small, super-fast storage inside the CPU:
- `ESP` = Stack Pointer (points to top of stack)
- `EAX`, `EBX`, `ECX`, `EDX` = General purpose registers
- `EIP` = Instruction Pointer (points to next instruction to execute)

**Instructions:**
- `mov dest, src` = Copy data from src to dest
- `call function` = Jump to a function (saves return address)
- `jmp label` = Unconditional jump to label
- `cli` = Clear Interrupts (disable interrupts)
- `hlt` = Halt CPU until next interrupt
- `sti` = Set Interrupts (enable interrupts)
- `pusha` = Push all general-purpose registers
- `popa` = Pop all general-purpose registers
- `iretd` = Return from interrupt (32-bit)

## The Multiboot Header Explained

```asm
MULTIBOOT_MAGIC     equ 0x1BADB002
MULTIBOOT_FLAGS     equ 0x00000003
MULTIBOOT_CHECKSUM  equ -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

dd MULTIBOOT_MAGIC
dd MULTIBOOT_FLAGS
dd MULTIBOOT_CHECKSUM
```

### Magic Number: 0x1BADB002
This is just a special number GRUB searches for.

### Flags: 0x00000003
- Bit 0 (value 1): "Align loaded modules on page boundaries"
- Bit 1 (value 2): "Provide memory map information"

### Checksum: -(magic + flags)
Error detection. The formula ensures: `magic + flags + checksum = 0`

## The Stack Explained

### What is the Stack?
The stack is a region of memory used for:
- Storing local variables
- Saving return addresses when calling functions
- Passing function parameters

### Key Properties:
1. **LIFO** - Last In, First Out
2. **Grows DOWNWARD** - From high addresses to low addresses
3. **ESP points to the TOP** - The most recently added item

Our stack setup:
```asm
stack_bottom:
    resb 65536      ; 64KB stack (increased from 16KB for interrupt handlers)
stack_top:
```

## Boot Code Walkthrough

```asm
_start:
    mov esp, stack_top  ; Set up stack pointer
    call kernel_main    ; Jump to Rust kernel

.hang:
    cli                 ; Disable interrupts
    hlt                 ; Halt CPU
    jmp .hang          ; Loop forever if we wake up
```

# What is a Linker Script?

A linker script tells the linker (ld) EXACTLY how to arrange all the pieces of your kernel.

## Line-by-Line Breakdown

### Entry Point
```ld
ENTRY(_start)
```
Tells the linker "execution begins at _start"

### Setting the Base Address
```ld
. = 1M;
```
The dot (.) is the "location counter" - start placing things at 1MB (0x100000)

### The Multiboot Section
```ld
.multiboot ALIGN(4K) : {
    *(.multiboot_header)
}
```
GRUB scans the first 8KB for the Multiboot header, so it MUST come first.

### The Text Section (Code)
```ld
.text ALIGN(4K) : {
    *(.text)
    *(.text.*)
}
```
All executable code goes here.

### The Read-Only Data Section
```ld
.rodata ALIGN(4K) : {
    *(.rodata)
    *(.rodata.*)
}
```
String literals and constants.

### The Data Section
```ld
.data ALIGN(4K) : {
    *(.data)
    *(.data.*)
}
```
Initialized global/static variables.

### The BSS Section
```ld
.bss ALIGN(4K) : {
    *(.bss)
    *(.bss.*)
    *(COMMON)
}
```
Uninitialized data (zero-initialized) - includes our stack.

## Memory Layout After Linking

```
0x00100000 (_kernel_start)
    ↓
    [.multiboot]        ← Multiboot header (MUST be in first 8KB)
    [.text]             ← Code: _start, kernel_main, etc.
    [.rodata]           ← Constants, strings
    [.data]             ← Initialized globals
    [.bss]              ← Stack, uninitialized data
    ↓
0x001XXXXX (_kernel_end)
```

# VGA Text Mode Guide

## What is VGA Text Mode?

VGA (Video Graphics Array) text mode is a simple way to display text on screen. When your computer boots, it starts in VGA text mode by default.

**Key Features:**
- **80 columns × 25 rows** = 2000 characters on screen
- **16 colors** for foreground and background
- **Memory-mapped** at address 0xB8000

## Memory Layout

### VGA Buffer Location
The VGA text buffer is at physical memory address **0xB8000**.

**Each character takes 2 bytes:**
```
[Byte 0]: ASCII character code
[Byte 1]: Color attribute (4 bits background, 4 bits foreground)
```

### Buffer Layout
```
Screen Position (0,0)  → Address 0xB8000
Screen Position (0,1)  → Address 0xB8002
Screen Position (1,0)  → Address 0xB80A0  (80 * 2 = 160 bytes per row)
```

**Formula for position (row, col):**
```
offset = (row * 80 + col) * 2
character_address = 0xB8000 + offset
color_address = 0xB8000 + offset + 1
```

## Color Codes

### Color Byte Format
```
Bit:  7 6 5 4 | 3 2 1 0
      ---------+---------
      Background|Foreground
```

### Color Values
| Value | Color       |
|-------|-------------|
| 0     | Black       |
| 1     | Blue        |
| 2     | Green       |
| 3     | Cyan        |
| 4     | Red         |
| 5     | Magenta     |
| 6     | Brown       |
| 7     | Light Gray  |
| 8     | Dark Gray   |
| 9     | Light Blue  |
| 10    | Light Green |
| 11    | Light Cyan  |
| 12    | Light Red   |
| 13    | Pink        |
| 14    | Yellow      |
| 15    | White       |

# Print Macros Guide

## What We Added

Three macros for easy printing:
1. **`print!(...)`** - Print without newline
2. **`println!(...)`** - Print with newline
3. **`printk!(...)`** - Kernel print (alias for `print!`)

## How It Works

### 1. The Global Writer

```rust
static mut WRITER: Writer = Writer::new();
```
A single, global VGA writer shared across all print calls.

### 2. The Print Function

```rust
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        WRITER.write_fmt(args).unwrap();
    }
}
```
Takes formatted arguments from the macro and writes to WRITER.

### 3. The Macros

```rust
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
```

# Cursor and Scroll Support Guide

## New Features Added

1. **Automatic Scrolling** - Screen scrolls up when you reach the bottom
2. **Hardware Cursor** - Blinking cursor shows where text appears
3. **Cursor Positioning** - Move cursor anywhere on screen
4. **Show/Hide Cursor** - Control cursor visibility

## Feature 1: Automatic Scrolling

When you print past the bottom (row 24), the screen scrolls up automatically.

```rust
fn scroll(&mut self) {
    unsafe {
        // Move each line up by one
        for row in 1..VGA_HEIGHT {
            for col in 0..VGA_WIDTH {
                let src_offset = (row * VGA_WIDTH + col) * 2;
                let dst_offset = ((row - 1) * VGA_WIDTH + col) * 2;
                
                *VGA_BUFFER.add(dst_offset) = *VGA_BUFFER.add(src_offset);
                *VGA_BUFFER.add(dst_offset + 1) = *VGA_BUFFER.add(src_offset + 1);
            }
        }
        
        // Clear the last line
        for col in 0..VGA_WIDTH {
            let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + col) * 2;
            *VGA_BUFFER.add(offset) = b' ';
            *VGA_BUFFER.add(offset + 1) = self.color;
        }
    }
    
    self.row = VGA_HEIGHT - 1;
}
```

## Feature 2: Hardware Cursor

The VGA card has hardware registers to control the cursor via **port I/O**.

**Port 0x3D4** - Control register (which setting to change)
**Port 0x3D5** - Data register (the value to set)

```rust
fn update_cursor(&self) {
    let pos = self.row * VGA_WIDTH + self.column;
    
    unsafe {
        // Send high byte
        outb(VGA_CTRL_PORT, 0x0E);
        outb(VGA_DATA_PORT, (pos >> 8) as u8);
        
        // Send low byte
        outb(VGA_CTRL_PORT, 0x0F);
        outb(VGA_DATA_PORT, pos as u8);
    }
}
```

# Keyboard Input Complete Guide

This is the most complex feature - requires understanding of interrupts, IDT, PIC, scan codes, and exception handling.

## Overview

Keyboard input requires:
1. **Interrupts** - Hardware signals that stop CPU execution
2. **IDT** - Interrupt Descriptor Table (tells CPU where handlers are)
3. **PIC** - Programmable Interrupt Controller (manages interrupts)
4. **Scan codes** - Raw keyboard codes that need translation
5. **ASCII conversion** - Convert scan codes to printable characters
6. **Exception handlers** - Catch errors before they crash the system

## What Are Interrupts?

### The Problem

Your kernel runs in a loop. How does the keyboard tell the kernel "hey, a key was pressed"?

**Bad solution:** Polling - wastes CPU cycles

**Good solution:** Interrupts
- CPU sleeps until interrupt
- Hardware wakes CPU automatically
- Handler runs only when needed

### How Interrupts Work

1. **Key is pressed** → Keyboard hardware sends electrical signal
2. **PIC receives signal** → Routes it to CPU as IRQ1
3. **CPU stops** → Saves current state
4. **CPU looks up handler** → Checks IDT for IRQ1 handler
5. **Handler runs** → Your `keyboard_interrupt_handler()` executes
6. **CPU resumes** → Returns to what it was doing

# Interrupt Handling Deep Dive

## Interrupt Descriptor Table (IDT)

The IDT is a table that tells the CPU: "When interrupt X happens, call function Y"

**Structure:**
```
IDT Entry 0  → Divide by zero handler
IDT Entry 1  → Debug handler
...
IDT Entry 32 → Timer interrupt (IRQ0)
IDT Entry 33 → Keyboard interrupt (IRQ1)  ← We handle this!
...
IDT Entry 255 → Last entry
```

### IDT Entry Structure

```rust
#[repr(C, packed)]
struct IdtEntry {
    offset_low: u16,    // Lower 16 bits of handler address
    selector: u16,      // Code segment selector (0x08 = kernel code)
    zero: u8,           // Always 0
    type_attr: u8,      // 0x8E = present, DPL=0, 32-bit interrupt gate
    offset_high: u16,   // Upper 16 bits of handler address
}
```

**Breaking it down:**
- **offset_low + offset_high:** Combined form the 32-bit address of handler
- **selector:** Code segment (0x08 = kernel code segment set by GRUB)
- **type_attr:** 0x8E = Present, kernel privilege, 32-bit interrupt gate

### Setting an IDT Entry

```rust
fn set_handler(&mut self, handler: unsafe extern "C" fn()) {
    let handler_addr = handler as usize;
    self.offset_low = (handler_addr & 0xFFFF) as u16;
    self.offset_high = ((handler_addr >> 16) & 0xFFFF) as u16;
    self.selector = 0x08;
    self.zero = 0;
    self.type_attr = 0x8E;
}
```

### Loading the IDT

```rust
#[repr(C, packed)]
struct IdtPointer {
    limit: u16,  // Size of IDT - 1
    base: u32,   // Address of IDT
}

asm!(
    "lidt [{}]",
    in(reg) &idt_ptr,
);
```

This tells the CPU where the IDT is located in memory.

## Programmable Interrupt Controller (PIC)

The PIC (8259 chip) manages hardware interrupts.

**Two PICs:**
- **PIC1** (Master) - Handles IRQ0-7
- **PIC2** (Slave) - Handles IRQ8-15

### IRQ Mapping

**Before remapping:**
```
IRQ0 (Timer)    → Interrupt 8   ← CONFLICT! (CPU exception)
IRQ1 (Keyboard) → Interrupt 9   ← CONFLICT!
```

**After remapping:**
```
IRQ0 (Timer)    → Interrupt 32
IRQ1 (Keyboard) → Interrupt 33  ← No conflict!
```

### Why Remap?

CPU reserves interrupts 0-31 for exceptions (divide by zero, page fault, etc.).
Old PIC mapping uses interrupts 8-15.
**Conflict!** We remap to 32-47 to avoid this.

### PIC Remapping Code

```rust
pub fn remap() {
    unsafe {
        // Start initialization
        outb(PIC1_COMMAND, ICW1_INIT);
        outb(PIC2_COMMAND, ICW1_INIT);

        // Set vector offsets
        outb(PIC1_DATA, 32);  // IRQ0-7 → interrupts 32-39
        outb(PIC2_DATA, 40);  // IRQ8-15 → interrupts 40-47

        // Tell PICs about each other
        outb(PIC1_DATA, 4);   // PIC2 at IRQ2
        outb(PIC2_DATA, 2);   // Cascade identity

        // Set 8086 mode
        outb(PIC1_DATA, ICW4_8086);
        outb(PIC2_DATA, ICW4_8086);

        // CRITICAL: Enable ONLY keyboard (IRQ1)
        outb(PIC1_DATA, 0xFD); // 11111101 - IRQ1 enabled
        outb(PIC2_DATA, 0xFF); // All disabled
    }
}
```

**The mask 0xFD is critical:**
- Bit 0 (IRQ0/Timer) = 1 (DISABLED)
- Bit 1 (IRQ1/Keyboard) = 0 (ENABLED)
- Bits 2-7 = 1 (DISABLED)

### End of Interrupt (EOI)

After handling an interrupt, you MUST tell the PIC "I'm done":

```rust
pub fn send_eoi(irq: u8) {
    unsafe {
        if irq >= 8 {
            outb(PIC2_COMMAND, PIC_EOI);
        }
        outb(PIC1_COMMAND, PIC_EOI);
    }
}
```

**Why?** PIC won't send more interrupts until you acknowledge!

## Keyboard Hardware

### PS/2 Keyboard

**Data Port:** 0x60 - Read scan codes from here
**Status Port:** 0x64 - Check if data is available

### Scan Codes

When you press a key:
- **Key press:** Scan code < 128 (e.g., 'A' = 0x1E)
- **Key release:** Scan code + 128 (e.g., 'A' release = 0x9E)

### Scan Code to ASCII

```rust
static SCANCODE_TO_ASCII: [u8; 128] = [
    0,    27,  b'1', b'2', b'3', b'4', b'5', b'6',
    b'7', b'8', b'9', b'0', b'-', b'=', 8,   b'\t',
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i',
    // ... full table
];
```

## Keyboard Interrupt Handler

### The Handler Function

```rust
#[no_mangle]
pub extern "C" fn rust_keyboard_handler() {
    unsafe {
        // 1. Read scan code from keyboard (CRITICAL: clears buffer)
        let scancode = inb(KEYBOARD_DATA_PORT);

        // 2. Send EOI to PIC (CRITICAL: acknowledges interrupt)
        pic::send_eoi(1);

        // 3. Check if it's a key press (not release)
        if scancode < 128 {
            // 4. Convert to ASCII
            let ascii = SCANCODE_TO_ASCII[scancode as usize];
            
            // 5. Print character
            if ascii != 0 {
                crate::print!("{}", ascii as char);
            }
        }
    }
}
```

**Why `#[no_mangle]` and `extern "C"`?**
- Keeps function name as-is (linker can find it)
- Uses C calling convention (required for interrupts)

### Assembly Wrapper

```asm
keyboard_interrupt_handler:
    cli                          ; Disable interrupts during handler
    pusha                        ; Save all registers
    call rust_keyboard_handler   ; Call Rust handler
    popa                         ; Restore all registers
    sti                          ; Re-enable interrupts
    iretd                        ; Return from interrupt
```

**Why assembly wrapper?**
- Saves/restores all CPU registers
- Uses `iretd` to return (normal `ret` won't work)
- Ensures interrupt context is preserved

# Exception Handlers

Exception handlers catch CPU faults BEFORE they become triple faults (which cause reboots).

## Why We Need Them

Without exception handlers:
1. CPU exception occurs (divide by zero, page fault, etc.)
2. No handler → Double fault
3. No double fault handler → Triple fault
4. Triple fault → CPU reset → System reboots

With exception handlers:
1. CPU exception occurs
2. Handler catches it
3. Displays error message
4. System halts gracefully (no reboot!)

## Common Exceptions

| Exception | Number | Cause |
|-----------|--------|-------|
| Divide by Zero | 0 | Division by zero |
| Invalid Opcode | 6 | Bad instruction |
| Double Fault | 8 | Exception during exception handling |
| General Protection Fault | 13 | Invalid memory access, segment violation |
| Page Fault | 14 | Accessing unmapped memory |

## Exception Handler Implementation

### Rust Handlers

```rust
#[no_mangle]
pub extern "C" fn rust_general_protection_fault() {
    unsafe {
        let vga = 0xb8000 as *mut u8;
        let msg = b"EXCEPTION #13: GENERAL PROTECTION FAULT";
        for (i, &byte) in msg.iter().enumerate() {
            let offset = (10 * 80 + i) * 2;
            *vga.add(offset) = byte;
            *vga.add(offset + 1) = 0x4F; // Red background
        }
        loop {}
    }
}
```

### Assembly Wrappers

```asm
general_protection_fault_handler:
    cli
    add esp, 4      ; Remove error code (GPF pushes one)
    pusha
    call rust_general_protection_fault
    popa
    iretd
```

**Why remove error code?**
Some exceptions push an error code onto the stack. We remove it before calling the Rust handler.

### Installing Exception Handlers

```rust
pub fn init() {
    unsafe {
        // Exception handlers (0-31)
        IDT.entries[0].set_handler(divide_by_zero_handler);
        IDT.entries[6].set_handler(invalid_opcode_handler);
        IDT.entries[8].set_handler(double_fault_handler);
        IDT.entries[13].set_handler(general_protection_fault_handler);
        IDT.entries[14].set_handler(page_fault_handler);
        
        // Set default handler for ALL other interrupts
        for i in 1..256 {
            if i != 0 && i != 6 && i != 8 && i != 13 && i != 14 && i != 33 {
                IDT.entries[i].set_handler(default_interrupt_handler);
            }
        }
        
        // Keyboard interrupt (IRQ1 = interrupt 33)
        IDT.entries[33].set_handler(keyboard_interrupt_handler);

        // Load IDT
        let idt_ptr = IdtPointer {
            limit: (core::mem::size_of::<Idt>() - 1) as u16,
            base: &IDT as *const _ as u32,
        };

        asm!("lidt [{}]", in(reg) &idt_ptr);
    }
}
```

**The default handler catches:**
- Timer interrupts (IRQ0)
- Spurious interrupts
- Any other unhandled interrupt

This prevents crashes from unexpected interrupts!

# Debugging Interrupt Storms

## What is an Interrupt Storm?

An interrupt storm occurs when an interrupt fires repeatedly without being properly cleared, causing:
- Handler called thousands of times per second
- Screen flickers
- System appears frozen
- CPU at 100% handling interrupts

## Our Interrupt Storm

**Symptoms:**
- Keys briefly appeared
- Screen flickered
- System seemed to reboot (but was just handler running too fast to see)

**Cause:**
Not always reading port 0x60 to clear the keyboard buffer.

**The Problem Code:**
```rust
// BAD: Checking status first
let status = inb(KEYBOARD_STATUS_PORT);
if (status & 0x01) == 0 {
    pic::send_eoi(1);
    return;  // Buffer not cleared!
}
```

If status says "no data" but buffer still has stale data, IRQ line stays asserted → interrupt fires again immediately → STORM!

**The Fix:**
```rust
// GOOD: Always read to clear buffer
let scancode = inb(KEYBOARD_DATA_PORT);  // ALWAYS clears buffer
pic::send_eoi(1);                         // Then acknowledge
```

**Why this works:**
- Reading port 0x60 ALWAYS clears the keyboard buffer
- This de-asserts the IRQ line
- Even if it's a spurious interrupt, reading clears any pending data
- No more storm!

## Debugging Steps

1. **Minimal handler** - Just read port and send EOI, nothing else
2. **Test incrementally** - Add features one at a time
3. **Check EOI** - ALWAYS send EOI after reading port
4. **Order matters** - Read port BEFORE sending EOI

# Fixing the Reboot Issue

## The Reboot Problem

**Symptoms:**
- Kernel boots
- Shows welcome message
- Counter increments (proving interrupts work)
- System reboots/flickers
- SeaBIOS screen appears
- Cycle repeats

**This means:**
- ✅ Interrupts work
- ✅ Handler executes
- ❌ Something causes triple fault → CPU reset

## Diagnosing the Issue

We added diagnostic output to see what was happening:

```rust
println!("Step 1: Initializing IDT...");
idt::init();

println!("Step 2: Remapping PIC...");
pic::remap();

println!("Step 3: Verifying PIC masks...");
let (mask1, mask2) = pic::read_masks();
println!("  -> PIC1 mask: {:08b}", mask1);
println!("  -> PIC2 mask: {:08b}", mask2);

println!("Step 4: Enabling interrupts...");
idt::enable_interrupts();
```

**What we discovered:**
```
PIC1 mask: 11111101  ← Correct!
PIC2 mask: 11111111  ← Correct!
```

But it STILL rebooted!

## The Root Cause: Timer Interrupt

Even though the mask looked correct, the **timer (IRQ0)** was still firing occasionally.

**What was happening:**
1. Keyboard interrupt (IRQ1) works → counter increments
2. Timer interrupt (IRQ0) fires → no handler!
3. CPU looks for handler at IDT entry 32 → finds nothing valid
4. Exception occurs → no exception handler
5. Double fault → no double fault handler
6. Triple fault → CPU reset → reboot

## The Solution: Comprehensive Exception Handlers

We added handlers for EVERY interrupt:

```rust
// Exception handlers for CPU faults
IDT.entries[0].set_handler(divide_by_zero_handler);
IDT.entries[6].set_handler(invalid_opcode_handler);
IDT.entries[8].set_handler(double_fault_handler);
IDT.entries[13].set_handler(general_protection_fault_handler);
IDT.entries[14].set_handler(page_fault_handler);

// Default handler for ALL other interrupts (including timer)
for i in 1..256 {
    if not_already_set {
        IDT.entries[i].set_handler(default_interrupt_handler);
    }
}

// Keyboard handler
IDT.entries[33].set_handler(keyboard_interrupt_handler);
```

**Now when timer fires:**
1. Timer interrupt (IRQ0) → IDT entry 32
2. CPU finds `default_interrupt_handler`
3. Handler displays "UNHANDLED INTERRUPT!" in red
4. System halts gracefully
5. No triple fault!

## Improving PIC Configuration

We also ensured the PIC mask was correct:

```rust
// CRITICAL: Mask ALL except keyboard
outb(PIC1_DATA, 0xFD);  // 11111101
outb(PIC2_DATA, 0xFF);  // 11111111
```

**Breaking down 0xFD:**
```
Bit 7: 1 (disabled)
Bit 6: 1 (disabled)
Bit 5: 1 (disabled)
Bit 4: 1 (disabled)
Bit 3: 1 (disabled)
Bit 2: 1 (disabled)
Bit 1: 0 (ENABLED - keyboard IRQ1) ← The only one enabled!
Bit 0: 1 (disabled - timer IRQ0)
```

With proper masking AND exception handlers, the system is now rock solid!

## Final Result

![final](/imgs/final.png "Final")

After all fixes:
- ✅ No interrupt storms
- ✅ No reboots or flickers
- ✅ Stable keyboard input
- ✅ Graceful error handling
- ✅ Professional quality

# Final Polishing

## Cleaning Up Warnings

We had 5 Rust warnings and 1 linker warning:

**Rust warnings:**
1. Unused import `crate::print` in kb.rs → Removed
2. Unused variables `mask1`, `mask2` in pic.rs → Removed
3. Unused methods in vga.rs → Added `#[allow(dead_code)]`
4. Unused function `read_masks` → Removed

**Linker warning:**
- RWX segment permissions → Fixed linker script with proper section separation

**Result:** Zero warnings! ✨

## Project Structure

Final clean structure:

```
kfs/
├── Cargo.toml
├── i386-unknown-none.json
├── boot.asm                # Bootloader
├── kb.asm                  # Keyboard interrupt wrapper
├── exceptions.asm          # Exception handler wrappers
├── linker.ld               # Memory layout
├── Makefile                # Build system
└── src/
    ├── lib.rs              # Kernel entry point
    ├── vga.rs              # VGA text mode
    ├── idt.rs              # Interrupt Descriptor Table
    ├── pic.rs              # Programmable Interrupt Controller
    ├── kb.rs               # Keyboard handler
    └── exceptions.rs       # Exception handlers
```

## Key Learnings

### Most Important Lessons

1. **Always read port 0x60** - This clears the keyboard buffer and prevents interrupt storms

2. **Always send EOI** - PIC won't send more interrupts until acknowledged

3. **Exception handlers are critical** - They catch errors before triple faults

4. **PIC masking matters** - Disable unwanted interrupts to prevent crashes

5. **Stack size is important** - Interrupt handlers need adequate stack space (64KB minimum)

6. **Port I/O order matters** - Read keyboard BEFORE sending EOI

7. **Assembly wrappers are necessary** - Interrupts need special handling (pusha/popa/iretd)

8. **Debugging is iterative** - Start minimal, add features incrementally, test thoroughly

### Common Pitfalls Avoided

- ❌ Not clearing keyboard buffer → Interrupt storm
- ❌ Not sending EOI → Interrupts stop working
- ❌ No exception handlers → Triple faults
- ❌ Wrong PIC mask → Unwanted interrupts fire
- ❌ Small stack → Stack overflow in handlers
- ❌ Wrong IDT entries → Jumps to invalid code

### What Makes This Kernel Special

1. **Written in Rust** - Memory-safe systems programming
2. **Full interrupt support** - Hardware-driven keyboard input
3. **Robust error handling** - Exception handlers prevent crashes
4. **Clean architecture** - Modular, well-documented code
5. **Professional quality** - Zero warnings, stable, polished

---

# Sources & Learning Resources

### Essential Reading:
1. **Multiboot Specification**
   - https://www.gnu.org/software/grub/manual/multiboot/multiboot.html

2. **OSDev Wiki - Multiboot**
   - https://wiki.osdev.org/Multiboot

3. **OSDev Wiki - Interrupts**
   - https://wiki.osdev.org/Interrupts

4. **OSDev Wiki - IDT**
   - https://wiki.osdev.org/Interrupt_Descriptor_Table

5. **OSDev Wiki - PIC**
   - https://wiki.osdev.org/PIC

6. **OSDev Wiki - PS/2 Keyboard**
   - https://wiki.osdev.org/PS/2_Keyboard

7. **OSDev Wiki - Exceptions**
   - https://wiki.osdev.org/Exceptions

8. **GNU ld Manual - Linker Scripts**
   - https://sourceware.org/binutils/docs/ld/Scripts.html

### Assembly Learning:
1. **x86 Assembly Guide (Virginia)**
   - https://www.cs.virginia.edu/~evans/cs216/guides/x86.html

2. **NASM Tutorial**
   - https://cs.lmu.edu/~ray/notes/nasmtutorial/

3. **x86 Instruction Reference**
   - https://www.felixcloutier.com/x86/

### Rust OS Development:
1. **Writing an OS in Rust**
   - https://os.phil-opp.com/
   - Excellent Rust kernel tutorial (different approach but great reference)

2. **Rust Embedded Book**
   - https://docs.rust-embedded.org/book/
   - Bare-metal Rust programming

### Deep Dives:
1. **Intel Software Developer Manuals**
   - https://software.intel.com/content/www/us/en/develop/articles/intel-sdm.html
   - The ultimate reference (very dense!)

2. **OSDev Wiki - Main Page**
   - https://wiki.osdev.org/
   - Browse for anything kernel-related

---

## Conclusion

This project represents a complete journey through bare-metal systems programming, from basic bootloader to a fully functional kernel with interrupt-driven I/O and robust error handling. Every feature was implemented from first principles, debugging real hardware issues along the way.

**Key Achievement:** Built a stable, professional-quality kernel with zero warnings, robust error handling, and clean architecture - all without any operating system or standard library support.

**Skills Mastered:**
- Bare-metal programming
- x86 architecture and interrupts
- Hardware interaction (VGA, keyboard, PIC)
- Assembly and Rust integration
- Debugging complex low-level issues
- Build system creation
- Professional code quality

This kernel demonstrates that systems programming in Rust is not only possible but produces safer, cleaner code than traditional C kernels while maintaining the same level of control and performance.

🦀 **Built with Rust** 🦀