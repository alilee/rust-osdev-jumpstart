ENTRY(_start)
SECTIONS
{
    /DISCARD/ :
    {
        *(.ARM.exidx*)
    }

    . = 0x40080000;

    .text :
    {
        *(.startup)
        *(.text*)
    }

    .rodata ALIGN(0x1000) :
    {
        *(.rodata*)
    }

    .data ALIGN(0x1000) :
    {
        *(.data*)
    }

    .bss ALIGN(0x1000) :
    {
        bss_base = .;
        *(.bss*)
        bss_end = .;
    }
}
