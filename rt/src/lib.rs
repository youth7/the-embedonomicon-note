#![no_std]

use core::panic::PanicInfo;
use core::ptr;

#[panic_handler]//自定义程序奔溃时的行为，因为缺乏运行时的原因这个必须自己定义
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {

    // 为何这里需要extern块修饰呢？因为这些符号都是由链接脚本直接定义的，需要直接从ELF文件中读取。
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
//不太明白为何要多用一个变量RESET_VECTOR而不是直接使用Reset函数
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