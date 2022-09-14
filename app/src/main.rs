#![no_main]
#![no_std]

use core::fmt::Write;
//使用semihosting技术进行输出，因为QEMU直接支持semihosting。而在真机环境则可能需要用到串口等技术
use cortex_m_semihosting::{debug, hio};

use rt::entry;

entry!(main);

fn main() -> ! {
    let mut hstdout = hio::hstdout().unwrap();

    #[export_name = "Hello, world!"]// 将日志信息编码到静态变量A的符号名中，
    static A: u8 = 0;

    // 将地址的值作为usize输出
    let _ = writeln!(hstdout, "{:#x}", &A as *const u8 as usize);

    #[export_name = "Goodbye"]
    static B: u8 = 0;

    let _ = writeln!(hstdout, "{:#x}", &B as *const u8 as usize);

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
} 