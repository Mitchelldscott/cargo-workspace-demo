
MEMORY
{
  /* Flash memory */
  FLASH (rx)  : ORIGIN = 0x08000000, LENGTH = 1M

  /* SRAM */
  RAM (rwx)   : ORIGIN = 0x20000000, LENGTH = 128K

  /* CCM RAM (Core Coupled Memory, fast RAM) */
  CCMRAM (rwx) : ORIGIN = 0x10000000, LENGTH = 64K
}

/* Stack and heap sizes */
_stack_size = 0x400;  /* 1KB stack as in IAR (0x400 bytes) */
_heap_size  = 0x200;  /* 512 bytes heap */

/* Define stack start symbol (_estack) at top of RAM */
_estack = ORIGIN(RAM) + LENGTH(RAM);

/* You can define symbols for heap start/end if needed */
_heap_start = _estack - _stack_size - _heap_size;
_heap_end   = _heap_start + _heap_size;

/* Entry point */
ENTRY(Reset_Handler)

SECTIONS
{
  .isr_vector :
  {
    KEEP(*(.isr_vector))
  } > FLASH

  .text :
  {
    *(.text*)
    *(.rodata*)
    KEEP(*(.eh_frame))
  } > FLASH

  .data : AT(ADDR(.text) + SIZEOF(.text))
  {
    _sdata = .;
    *(.data*)
    _edata = .;
  } > RAM

  .bss :
  {
    _sbss = .;
    *(.bss*)
    *(COMMON)
    _ebss = .;
  } > RAM

  /* Optionally place CCMRAM sections if used */
  .ccmram (NOLOAD) :
  {
    *(.ccmram*)
  } > CCMRAM
}