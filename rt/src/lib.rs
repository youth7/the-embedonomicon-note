#![no_std]

use core::panic::PanicInfo;

#[panic_handler]//自定义程序奔溃时的行为，因为缺乏运行时的原因这个必须自己定义
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
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