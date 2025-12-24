/* linker.ld - Linker script for i386 kernel (warnings fixed) */

ENTRY(_start)

SECTIONS
{
    /* Kernel is loaded at 1MB by GRUB */
    . = 1M;

    _kernel_start = .;

    /* Multiboot header MUST come first (GRUB looks in first 8KB) */
    .multiboot ALIGN(4K) : {
        *(.multiboot_header)
    }

    /* Read-only code section */
    .text ALIGN(4K) : {
        *(.text)
        *(.text.*)
    }

    /* Read-only data section */
    .rodata ALIGN(4K) : {
        *(.rodata)
        *(.rodata.*)
    }

    /* Read-write data section */
    .data ALIGN(4K) : {
        *(.data)
        *(.data.*)
    }

    /* Uninitialized data section */
    .bss ALIGN(4K) : {
        *(.bss)
        *(.bss.*)
        *(COMMON)
    }

    _kernel_end = .;

    /* Discard debug and comment sections */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
        *(.note*)
    }
}
