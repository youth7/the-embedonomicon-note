#![no_main]
#![no_std]

use cortex_m::interrupt;
use cortex_m_semihosting::{
    debug,
    hio::{self, HostStream}//0.5.0之后改为使用HostStream结构体，原文中是使用HStdout
};

use log::{global_logger, log, GlobalLog};
use rt::entry;

struct Logger;

global_logger!(Logger);//将Logger注册为全局logger，这样在使用log!宏的时候就不再需要提供logger对象

entry!(main);

fn main() -> ! {
    log!("Hello, world!");//更为简洁的日志API，不需要主动提供logger对象
    log!("Goodbye");
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}

//全局logger的实现
impl GlobalLog for Logger {
    fn log(&self, address: u8) {
        //interrupt::free作用是在一个无中断的上下文环境中执行函数，这是访问static mut类型变量的要求
        //因为HSTDOUT是静态变量，只有做到这样才能保证内存安全。
        //这种机制就是所谓的临界区（critical section）
        interrupt::free(|_| unsafe {
            static mut HSTDOUT: Option<HostStream> = None;

            // 延迟初始化
            if HSTDOUT.is_none() {
                HSTDOUT = Some(hio::hstdout()?);
            }
            let hstdout = HSTDOUT.as_mut().unwrap();
            hstdout.write_all(&[address])
        })
        .ok(); // 调用ok()意味着忽略错误并返回Option
    }
}