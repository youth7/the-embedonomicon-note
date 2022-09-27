#![no_main]//告诉编译器不要使用main函数作为程序的入口，因为main对运行时有要求
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]//自定义程序奔溃时的行为，因为缺乏运行时的原因这个必须自己定义
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 42;
    //永不退出的发散函数
    loop {}
}
//说明这个函数需要编译到名称为.vector_table.reset_vector的这个节中，这个节在后面会被引用到
#[link_section = ".vector_table.reset_vector"]
//告诉编译器不要用Rust的命名规则为Reset重命名，保留原来的名称就好
#[no_mangle]
//RESET_VECTOR就是vector table中的第二个元素，指向了异常处理函数Reset
//不太明白为何要多用一个变量RESET_VECTOR而不是直接使用Reset函数
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;