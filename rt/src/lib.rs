#![no_std]

use core::arch::asm;
use core::panic::PanicInfo;

#[panic_handler]//自定义程序奔溃时的行为，因为缺乏运行时的原因这个必须自己定义
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    //为简化程序这里删除了栈初始化的代码


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
    fn NMI();
    // fn HardFault();删除对HardFault的声明，因为不需要在rust代码中调用它
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
    Vector { handler: HardFaultTrampoline },// 改为使用辅助函数，通过它去调用HardFault
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

#[allow(non_snake_case)]
#[no_mangle]
pub fn DefaultExceptionHandler(_ef: *const u32) -> ! {//因为HardFaultTrampoline会传递参数，因此函数签名也要同步修改
    loop {}
}


#[no_mangle]
extern "C" fn HardFaultTrampoline() {
    unsafe{
        asm!(
          "mrs r0, MSP",
          "b HardFault"
        )
    }
}