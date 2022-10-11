/* Memory layout of the LM3S6965 microcontroller */
/* 1K = 1 KiBi = 1024 bytes */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 256K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* The entry point is the reset handler */
ENTRY(Reset);

EXTERN(RESET_VECTOR);

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* First entry: initial Stack Pointer value */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* Second entry: reset vector */
    KEEP(*(.vector_table.reset_vector));
  } > FLASH

  .text :
  {
    *(.text .text.*);
  } > FLASH

  /DISCARD/ :
  {
    *(.ARM.exidx .ARM.exidx.*);
  }

  /* 新增三个用于保存数据的section */
  .rodata :
  {
    *(.rodata .rodata.*);
  } > FLASH

  .bss :
  {
    _sbss = .; /* 将.bss的起始地址保存到_sbss中 */
    *(.bss .bss.*);
    _ebss = .;/* 将.bss的结束地址保存到_ebss中 */
  } > RAM

/*
用AT命令指定.data的LMA，让这个段紧贴着.rodata段，但这里并没有指定VMA
这里非常重要，理解AT命令请看这里的例子：https://ftp.gnu.org/old-gnu/Manuals/ld-2.9.1/html_chapter/ld_3.html#SEC16
务必理解这里通过什么样的方式指定了.data的VMA和LMA（链接脚本并没有指定VMA）
*/
  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))  
  {
    _sdata = .;/* 将.data的起始地址保存到_sdata中,注意：这里相当于保存了该节的VMA！ */
    *(.data .data.*);
    _edata = .;/* 将.data的结束地址保存到_edata中 */
  } > RAM

  _sidata = LOADADDR(.data);/*将.data的LMA与某个符号关联起来*/
}