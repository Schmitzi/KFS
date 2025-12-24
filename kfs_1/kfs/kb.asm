; keyboard_int.asm - Assembly wrapper for keyboard interrupt handler

section .text

global keyboard_interrupt_handler
extern rust_keyboard_handler

; Interrupt handler that uses IRET
keyboard_interrupt_handler:
    pusha                        ; Save all general-purpose registers
    call rust_keyboard_handler   ; Call Rust handler
    popa                         ; Restore all general-purpose registers
    iretd                        ; Return from interrupt (32-bit)