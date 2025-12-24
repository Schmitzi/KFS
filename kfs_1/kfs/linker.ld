/* linker.ld - Linker script for i386 kernel */

/* Entry point - where execution starts */
ENTRY(_start)

SECTIONS
{
    /* Kernel is loaded at 1MB by GRUB */
    . = 1M;

    /* Start of kernel in memory */
    _kernel_start = .;

    /* Multiboot header MUST come first (GRUB looks in first 8KB) */
    .multiboot : ALIGN(4K)
    {
        *(.multiboot_header)
    }

    /* Read-only data (code) */
    .text : ALIGN(4K)
    {
        *(.text)
    }

    /* Read-only data (constants, strings, etc) */
    .rodata : ALIGN(4K)
    {
        *(.rodata)
    }

    /* Initialized data */
    .data : ALIGN(4K)
    {
        *(.data)
    }

    /* Uninitialized data (BSS = Block Started by Symbol) */
    .bss : ALIGN(4K)
    {
        *(.bss)
    }

    /* End of kernel in memory */
    _kernel_end = .;
}
