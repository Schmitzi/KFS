; boot.asm - Multiboot bootloader for i386 kernel

; ==============================================================================
; MULTIBOOT HEADER
; ==============================================================================

section .multiboot_header
align 4

MULTIBOOT_MAGIC     equ 0x1BADB002      ; Magic number for GRUB
MULTIBOOT_FLAGS     equ 0x00000003      ; Align modules + memory map
MULTIBOOT_CHECKSUM  equ -(MULTIBOOT_MAGIC + MULTIBOOT_FLAGS)

dd MULTIBOOT_MAGIC
dd MULTIBOOT_FLAGS
dd MULTIBOOT_CHECKSUM

; ==============================================================================
; STACK
; ==============================================================================

section .bss
align 16

stack_bottom:
    resb 65536       ; 64KB stack
stack_top:

; ==============================================================================
; BOOT CODE
; ==============================================================================

section .text
global _start
extern kernel_main

_start:
    mov esp, stack_top  ; Set up stack pointer
    call kernel_main    ; Jump to Rust kernel

.hang:
    cli                 ; Disable interrupts
    hlt                 ; Halt CPU
    jmp .hang           ; Loop forever if we wake up
