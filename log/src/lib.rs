#![no_std]

//关于Sync、Send的语义参考：https://www.zhihu.com/question/303273488/answer/2345814358
pub trait GlobalLog: Sync {
    /**
    声明一个trait，全局的单例日志对象必须实现它，需要注意以下几点：
    1，log方法只需要&self，不消耗所有权，因为它用的是单例的全局共享对象。
    2，这里并没有像下面的Log trait那样定义一个关联的错误类型，这是为了简化细节。
       将错误处理委托给用户，而不是由库指定错误处理规则并强制用户实现。
    **/
    fn log(&self, address: u8);
}

pub trait Log {
    type Error;
    fn log(&mut self, address: u8) -> Result<(), Self::Error>;
}

#[macro_export]
macro_rules! log {
    //该宏接受两种传参，第一种传参不需要提供日志对象，第二种传参需要提供日志对象

    //第一种传参方式，此时会使用一个名为"LOGGER"的全局对象进行日志输出，它是一个定义在某处的全局对象
    ($string:expr) => {
        unsafe {
            extern "Rust" {
                //关于$crate请见：
                //https://zjp-cn.github.io/tlborm/decl-macros/minutiae/hygiene.html?highlight=%24crate#unhygientic
                //我们并不知道LOGGER的具体类型，但要求它必须实现了这里必须实现了GlobalLog，所以必须用trait object
                static LOGGER: &'static dyn $crate::GlobalLog;
            }

            #[export_name = $string]
            #[link_section = ".log"]
            static SYMBOL: u8 = 0;

            $crate::GlobalLog::log(LOGGER, &SYMBOL as *const u8 as usize as u8)
        }
    };

    //第二种传参方式，需要用户自己提供日志对象进行输出，这是上一章的方式
    ($logger:expr, $string:expr) => {{
        #[export_name = $string]
        #[link_section = ".log"]
        static SYMBOL: u8 = 0;

        $crate::Log::log(&mut $logger, &SYMBOL as *const u8 as usize as u8)
    }};
}

//提供一个宏，让用户注册单例的全局日志对象，并将符号名称定为"LOGGER"，这样正好和上面对应
#[macro_export]
macro_rules! global_logger {
    ($logger:expr) => {
        #[no_mangle]
        pub static LOGGER: &dyn $crate::GlobalLog = &$logger;
    };
}
