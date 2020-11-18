MEMORY
{
   ps7_ddr_0 : ORIGIN = 0x30000000, LENGTH = 0xFF00000
   ps7_qspi_linear_0 : ORIGIN = 0xFC000000, LENGTH = 0x1000000
   ps7_ram_0 : ORIGIN = 0x0, LENGTH = 0x30000
   ps7_ram_1 : ORIGIN = 0xFFFF0000, LENGTH = 0xFE00
}

ENTRY(_start)

SECTIONS 
{
    .text.init :
    {
        *(.text.init)
    } > ps7_ddr_0
    .text : 
    {
        *(.text .text.*)
    } > ps7_ddr_0
    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*)
    } > ps7_ddr_0
    .data : ALIGN(4)
    {
        *(.data .data.*)
    } > ps7_ddr_0
    . = ALIGN(4K);
    . += 8K;
    _stack = .;

    .shstrtab : 
    {
        *(.shstrtab)
    }



    /DISCARD/ :
    {
        *(.ARM.*)
    }

}