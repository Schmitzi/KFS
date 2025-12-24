# KFS_1 - Kernel From Scratch

A bare-metal i386 kernel written in Rust, bootable with GRUB.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Building](#building)
- [Running](#running)
- [Implementation Details](#implementation-details)
- [Learning Resources](#learning-resources)

---

## Introduction

This project implements a minimal kernel from scratch without any existing software, API, or operating system dependencies. The kernel is written in Rust (with Assembly for the bootloader) and boots via GRUB on i386 architecture.

The goal is to understand low-level systems programming, bootloaders, VGA text mode, and the fundamentals of kernel development.

---

## Features

### Mandatory Features ✅
- **GRUB Bootable** - Multiboot-compliant kernel
- **Assembly Bootloader** - Handles Multiboot header and initialization
- **VGA Text Mode** - Display "42" on screen
- **Custom Linker Script** - Links ASM and Rust code together
- **Makefile** - Automates build process for multiple languages

### Bonus Features ✅
- **Color Support** - 16 VGA colors for foreground/background
- **Print Macros** - `print!()`, `println!()`, `printk!()` for easy output
- **Scroll Support** - Automatic scrolling when screen fills
- **Cursor Control** - Hardware cursor positioning and visibility

### Future Features 🚧
- Keyboard input (interrupts and scan codes)
- Multiple virtual screens with keyboard shortcuts

---

## Project Structure

```
kfs/
├── Cargo.toml              # Rust project configuration
├── Cargo.lock              
├── i386-unknown-none.json  # Custom bare-metal target
├── boot.asm                # Assembly bootloader (Multiboot)
├── linker.ld               # Linker script
├── Makefile                # Build automation
├── src/
│   ├── lib.rs              # Kernel entry point
│   └── vga.rs              # VGA text mode driver
└── target/                 # Build artifacts
```

---

## Prerequisites

- **Rust nightly toolchain**
  ```bash
  rustup toolchain install nightly
  rustup component add rust-src --toolchain nightly
  ```

- **NASM assembler**
  ```bash
  sudo pacman -S nasm  # Arch Linux
  ```

- **QEMU** (for testing)
  ```bash
  sudo pacman -S qemu-system-i386
  ```

- **GRUB tools** (for ISO creation)
  ```bash
  sudo pacman -S grub xorriso mtools
  ```

---

## Building

### Quick Build
```bash
make
```

This will:
1. Assemble `boot.asm` → `boot.o`
2. Build Rust kernel → `libkfs.a`
3. Link everything → `kernel.bin`

### Clean Build
```bash
make clean  # Remove build artifacts
make fclean # Deep clean (removes target/)
make re     # Rebuild from scratch
```

---

## Running

### Test in QEMU
```bash
make run
```

### Create Bootable ISO
```bash
make iso
```

### Run ISO in QEMU
```bash
make run-iso
```

---

## Implementation Details

### Architecture

**Target:** i386 (32-bit x86)
- Custom target specification (`i386-unknown-none.json`)
- No standard library (`#![no_std]`)
- No runtime (`#![no_main]`)

### Boot Process

1. **GRUB** loads kernel at 1MB (0x100000)
2. **boot.asm** sets up stack and calls `kernel_main`
3. **Rust kernel** initializes VGA and displays output
4. **Infinite loop** - kernel runs forever

### Memory Layout (linker.ld)

```
0x00100000 (1MB)  : Multiboot header
                  : .text (code)
                  : .rodata (constants)
                  : .data (initialized data)
                  : .bss (stack, uninitialized data)
```

### VGA Text Mode

- **Address:** 0xB8000 (memory-mapped I/O)
- **Format:** 80 columns × 25 rows
- **Character:** 2 bytes (ASCII + color attribute)
- **Cursor:** Hardware cursor via ports 0x3D4/0x3D5

### Key Components

**boot.asm:**
- Multiboot header (magic: 0x1BADB002)
- Stack setup (16KB)
- Calls `kernel_main()`

**lib.rs:**
- Kernel entry point
- Panic handler
- Demo output

**vga.rs:**
- VGA text buffer at 0xB8000
- Color support (16 colors)
- Print macros (`print!`, `println!`, `printk!`)
- Scrolling (software-based memory copying)
- Cursor control (hardware registers via port I/O)

---

## Language Choice

**Why Rust?**
- Memory safety without garbage collection
- Zero-cost abstractions
- Pattern matching and type safety
- No undefined behavior (in safe code)
- Great for systems programming

**Challenges:**
- No standard library (`no_std`)
- Manual panic handler
- Custom target specification
- Translating C examples to Rust

---

## Compilation Flags

### Rust (via Cargo.toml)
```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"

[lib]
crate-type = ["staticlib"]
```

### Assembly (NASM)
```bash
nasm -f elf32 boot.asm -o boot.o
```

### Linker (ld)
```bash
ld -m elf_i386 -n -T linker.ld -o kernel.bin boot.o libkfs.a
```

---

## Learning Resources

### Essential Reading

**Kernel Development:**
- [OSDev Wiki](https://wiki.osdev.org/) - Comprehensive OS development guide
- [OSDev Bare Bones](https://wiki.osdev.org/Bare_Bones) - Minimal kernel tutorial
- [OSDev Multiboot](https://wiki.osdev.org/Multiboot) - GRUB bootloader spec

**Rust OS Development:**
- [Writing an OS in Rust](https://os.phil-opp.com/) - Excellent Rust kernel tutorial
- [Rust Embedded Book](https://docs.rust-embedded.org/book/) - Bare-metal Rust

**Specifications:**
- [Multiboot Specification](https://www.gnu.org/software/grub/manual/multiboot/multiboot.html) - Official GRUB spec
- [GNU Linker Manual](https://sourceware.org/binutils/docs/ld/Scripts.html) - Linker scripts

### Assembly & x86

- [x86 Assembly Guide](https://www.cs.virginia.edu/~evans/cs216/guides/x86.html) - Beginner-friendly
- [NASM Tutorial](https://cs.lmu.edu/~ray/notes/nasmtutorial/) - NASM syntax
- [x86 Instruction Reference](https://www.felixcloutier.com/x86/) - Look up instructions

### VGA Programming

- [OSDev VGA Text Mode](https://wiki.osdev.org/Text_mode) - VGA text mode basics
- [OSDev Text Mode Cursor](https://wiki.osdev.org/Text_Mode_Cursor) - Hardware cursor

---

## What You'll Learn

By completing this project, you'll understand:

1. **Boot Process** - How computers boot from power-on to kernel
2. **Multiboot** - Bootloader standards and GRUB
3. **Bare-Metal Programming** - Code without an OS
4. **Memory Management** - Physical memory layout, linker scripts
5. **Hardware I/O** - Memory-mapped I/O, port I/O
6. **VGA Programming** - Text mode, colors, cursor control
7. **x86 Assembly** - Bootloader, stack setup, calling conventions
8. **Rust Systems Programming** - `no_std`, panic handlers, inline assembly

---

## Acknowledgments

- **42 School** for the project specification
- **OSDev Community** for excellent documentation
- **Philipp Oppermann** for "Writing an OS in Rust" blog series
- **Rust Community** for bare-metal tooling

---

## License

Educational project - feel free to learn from and build upon this code.

---

**Built with 🦀 Rust and ❤️**