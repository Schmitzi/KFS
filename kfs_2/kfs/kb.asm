; keyboard_int.asm - Assembly wrapper with explicit interrupt control

section .text

global keyboard_interrupt_handler
extern rust_keyboard_handler

keyboard_interrupt_handler:
    cli                          ; Explicitly disable interrupts
    pusha                        ; Save all general-purpose registers
    call rust_keyboard_handler   ; Call Rust handler
    popa                         ; Restore all general-purpose registers
    sti                          ; Re-enable interrupts
    iretd                        ; Return from interrupt  ; Return from interrupt (32-bit)
