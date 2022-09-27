#![no_std]

pub trait Log {
    type Error;
    fn log(&mut self, address: u8) -> Result<(), Self::Error>;
}

// 输出error等级的日志
#[macro_export]
macro_rules! error {
    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log.error"] // 放置到.log.error这个节
        static SYMBOL: u8 = 0;
        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)//最终都是调用log函数，只是放置的地方不一样
    }};
}

// 输出warn等级的日志
#[macro_export]
macro_rules! warn {
    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log.warning"] // 放置到.log.warning这个节
        static SYMBOL: u8 = 0;
        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)//最终都是调用log函数，只是放置的地方不一样
    }};
}