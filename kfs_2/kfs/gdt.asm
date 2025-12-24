; gdt.asm
global gdt_flush

gdt_flush:
    mov eax, [esp+4]  ; Get GDT pointer from argument
    lgdt [eax]        ; Load GDT
    
    mov ax, 0x10      ; 0x10 is offset to kernel data segment
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    
    jmp 0x08:.flush   ; 0x08 is offset to kernel code segment
.flush:
    ret