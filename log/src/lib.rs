#![no_std]

pub trait Log {
    type Error;

    fn log(&mut self, address: u8) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $string:expr) => {{//用户调用宏的时候参数包括2个：一个log Trait实例；一个日志字符串
        #[export_name = $string]
        #[link_section = ".log"]
        static SYMBOL: u8 = 0;// 每条日志字符串都有一个对应的静态变量

        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
        // 由用户提供具体的输出实现，但是对于本教程来说，个人认为应该由库提供实现才对，这样用户就无需关注这方面的细节
    }};
}