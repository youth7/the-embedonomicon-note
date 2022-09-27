SECTIONS
{
  .log 0 (INFO) : {
    *(.log.error);              /*前面部分放置error级别日志*/
    __log_warning_start__ = .;  /*将当前地址值与符号__log_warning_start__关联起来，意味着剩下地址存的都是警告级别的日志*/
    *(.log.warning);            /*剩下部分放置warning级别日志*/
  }
}