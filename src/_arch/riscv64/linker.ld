OUTPUT_ARCH("riscv")

ENTRY(_start)
SECTIONS
{
    . = 0x80000000;

    .text :
    {
        *(.startup)
        *(.text*)
    }

    .rodata ALIGN(0x1000) :
    {
        PROVIDE(_global_pointer = .);
        *(.rodata*)
    }

    .data ALIGN(0x1000) :
    {
        *(.sdata*)
        *(.data*)
    }

    .bss ALIGN(0x1000) :
    {
        bss_start = .;
        *(.sbss*)
        *(.bss*)
        bss_end = .;
    }
}