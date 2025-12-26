; kb.asm - Assembly wrapper with explicit interrupt control

section .text

global kb_pic_handler
extern kbhandler

kb_pic_handler:
    cli                          ; Explicitly disable interrupts
    pusha                        ; Save all general-purpose registers
    call kbhandler               ; Call Rust handler
    popa                         ; Restore all general-purpose registers
    sti                          ; Re-enable interrupts
    iretd                        ; Return from interrupt (32-bit)
