#![no_std]
#![no_main]
#![feature(core_intrinsics)]// 因为使用了core_intrinsics的原因，必须切换到nightly来构建

use core::intrinsics;
use rt::entry;
//使用rt中暴露出来的宏来调用用户编写的函数，此时用户编写的函数可以用其它名称，例如这里就用了main2
//其实这样做增加了一些复杂性，之前的方法用户只需要编写一个main函数就可以了，其它什么不用管
//而现在则需要了解entry宏
entry!(main2);
fn main2() -> ! {
    //触发 HardFault exception
    intrinsics::abort()
}

#[no_mangle]
pub extern "C" fn HardFault() -> ! {
    //自定义异常处理函数，用QEMU调试时候应该停留在这里
    loop {}
}