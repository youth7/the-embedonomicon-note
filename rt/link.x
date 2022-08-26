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
EXTERN(EXCEPTIONS); 

SECTIONS
{
  .vector_table ORIGIN(FLASH) :
  {
    /* vector table第一项：ISP */
    LONG(ORIGIN(RAM) + LENGTH(RAM));

    /* vector table第二项 */
    KEEP(*(.vector_table.reset_vector));

    KEEP(*(.vector_table.exceptions)); /* 将剩余的14个异常处理函数保存到flash中，加上上面已有的两项刚好16项 */
  } > FLASH

  /* 为符号提供默认值，只有当用户未提供自定义的异常处理程序时候才会生效，注意被提供默认值的项都是在lib.rs中声明过的外部函数 */
  PROVIDE(NMI = DefaultExceptionHandler);
  PROVIDE(HardFault = DefaultExceptionHandler);
  PROVIDE(MemManage = DefaultExceptionHandler);
  PROVIDE(BusFault = DefaultExceptionHandler);
  PROVIDE(UsageFault = DefaultExceptionHandler);
  PROVIDE(SVCall = DefaultExceptionHandler);
  PROVIDE(PendSV = DefaultExceptionHandler);
  PROVIDE(SysTick = DefaultExceptionHandler);

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

  .data : AT(ADDR(.rodata) + SIZEOF(.rodata))  /*指定.data的LMA，紧贴着.rodata*/
  {
    _sdata = .;/* 将.data的起始地址保存到_sdata中 */
    *(.data .data.*);
    _edata = .;/* 将.data的结束地址保存到_edata中 */
  } > RAM

  _sidata = LOADADDR(.data);/*将.data的LMA与某个符号关联起来*/


}