#![no_main]
#![no_std]

use cortex_m_semihosting::{
    debug,
    hio::{self, HostStream}//0.5.0之后改为使用HostStream结构体，原文中是使用HStdout
};

use log::{error, warn, Log};
use rt::entry;

struct Logger {
    hstdout: HostStream,
}

impl Log for Logger {
    type Error = ();
    fn log(&mut self, address: u8) -> Result<(), ()> {
        self.hstdout.write_all(&[address])
    }
}

entry!(main);

fn main() -> ! {
    let hstdout = hio::hstdout().unwrap();
    let mut logger = Logger { hstdout };
    let _ = warn!(logger, "Hello, world!");
    let _ = error!(logger, "Goodbye");
    let _ = error!(logger, "你好呀");
    let _ = warn!(logger, "是的师父！");
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}