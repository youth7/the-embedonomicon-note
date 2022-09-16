#![no_main]
#![no_std]

use cortex_m_semihosting::{
    debug,
    hio::{self, HostStream}//0.5.0之后改为使用HostStream结构体，原文中是使用HStdout
};

use log::{log, Log};
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

    let _ = log!(logger, "Hello, world!");

    let _ = log!(logger, "Goodbye");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
