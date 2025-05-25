OUTPUT_ARCH("riscv")
ENTRY(_start)

SECTIONS

{
  . = 0x80000000;

  .text : {
    *(.text .text.*)
    . = ALIGN(0x1000);
    PROVIDE(etext = .);
  }

  .rodata : {
    *(.srodata .srodata.*)
    . = ALIGN(16);
    *(.rodata .rodata.*)
    . = ALIGN(16);
  }

  .data : {
    *(.sdata .sdata.*)
    . = ALIGN(16);
    *(.data .data.*)
    . = ALIGN(16);
  }

  .bss : {
    *(.sbss .sbss.*)
    . = ALIGN(16);
    *(.bss .bss.*)
    . = ALIGN(16);
  }

  PROVIDE(end = .);
}
