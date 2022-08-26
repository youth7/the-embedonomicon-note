#![no_std]

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]//自定义程序奔溃时的行为，因为缺乏运行时的原因这个必须自己定义
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    // 为何这里需要extern块修饰呢？
    extern "C" {
        static mut _sbss: u8;
        static mut _ebss: u8;

        static mut _sdata: u8;
        static mut _edata: u8;
        static _sidata: u8;
    }

    //初始化.bss只需要将对应区域全部置为0即可
    let count = &_ebss as *const u8 as usize - &_sbss as *const u8 as usize;
    ptr::write_bytes(&mut _sbss as *mut u8, 0, count);

    //初始化.data则需要从ROM复制
    let count = &_edata as *const u8 as usize - &_sdata as *const u8 as usize;
    ptr::copy_nonoverlapping(&_sidata as *const u8, &mut _sdata as *mut u8, count);


    extern "Rust" {
        fn main() -> !;//将控制权交给用户的main函数，因此main必须是发散的
    }

    main()
}
//说明这个函数需要编译到名称为.vector_table.reset_vector的这个节中，这个节在后面会被引用到
#[link_section = ".vector_table.reset_vector"]
//告诉编译器不要用Rust的命名规则为Reset重命名，保留原来的名称就好
#[no_mangle]
//RESET_VECTOR就是vector table中的第二个元素，指向了异常处理函数Reset
//一开始不太明白为何要多用一个变量RESET_VECTOR而不是直接使用Reset函数，后来发现Reset函数是被编译到.text节中的，
//这样后续要继续引用Reset的地址会比较麻烦，用RESET_VECTOR来保存Reset的地址并放到.vector_table.reset_vector中有利于在链接脚本中引用Reset的地址
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;


#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]//__main导出为main，则rt中最终链接的是当前文件内的main函数，而不是用户的main函数
        pub unsafe fn __main() -> ! {
            // $path就是用户传入的函数，对它进行类型检验后调用，此时用户写的函数的名称可以自定义了，不一定就是要用main
            let f: fn() -> ! = $path;

            f()
        }
    }
}


pub union Vector {
    // 一个Vector就是vector table中的一项，根据arm的文档，每一项要么是一个异常处理函数，要么是预留（值为0）
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    //声明会用到的外部函数，因为有可能是用户提供的所以必须用extern，不明白为何是C规范而不是Rust规范，
    //注意这里只是声明并没有提供具体实现，实现有两种，一种是使用默认的DefaultExceptionHandler；一种是用户提供
    fn NMI();
    fn HardFault();
    fn MemManage();
    fn BusFault();
    fn UsageFault();
    fn SVCall();
    fn PendSV();
    fn SysTick();
}

#[link_section = ".vector_table.exceptions"]// 将异常处理函数保存到节.vector_table.exceptions中
#[no_mangle]
pub static EXCEPTIONS: [Vector; 14] = [//定义vector table中剩余的14项
    Vector { handler: NMI },
    Vector { handler: HardFault },
    Vector { handler: MemManage },
    Vector { handler: BusFault },
    Vector { handler: UsageFault},
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: SVCall },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: PendSV },
    Vector { handler: SysTick },
];

#[no_mangle]
pub extern "C" fn DefaultExceptionHandler() {// 定义一个默认的异常处理函数
    loop {}
}