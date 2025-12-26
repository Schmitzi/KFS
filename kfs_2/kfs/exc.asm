; exc.asm - Assembly wrappers for exception handlers

section .text

; Exception #0: Divide by zero
global divide_by_zero_handler
extern rust_divide_by_zero
divide_by_zero_handler:
    cli
    pusha
    call rust_divide_by_zero
    popa
    iretd

; Exception #6: Invalid opcode
global invalid_opcode_handler
extern rust_invalid_opcode
invalid_opcode_handler:
    cli
    pusha
    call rust_invalid_opcode
    popa
    iretd

; Exception #8: Double fault (has error code)
global double_fault_handler
extern rust_double_fault
double_fault_handler:
    cli
    add esp, 4      ; Remove error code
    pusha
    call rust_double_fault
    popa
    iretd

; Exception #13: General protection fault (has error code)
global general_protection_fault_handler
extern rust_general_protection_fault
general_protection_fault_handler:
    cli
    add esp, 4      ; Remove error code
    pusha
    call rust_general_protection_fault
    popa
    iretd

; Exception #14: Page fault (has error code)
global page_fault_handler
extern rust_page_fault
page_fault_handler:
    cli
    add esp, 4      ; Remove error code
    pusha
    call rust_page_fault
    popa
    iretd

; Default handler for unhandled interrupts
global default_interrupt_handler
extern rust_default_interrupt
default_interrupt_handler:
    cli
    pusha
    call rust_default_interrupt
    popa
    iretd
