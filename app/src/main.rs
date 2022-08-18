#![no_std]
#![no_main]

use rt::entry;
//使用rt中暴露出来的宏来调用用户编写的函数，此时用户编写的函数可以用其它名称，例如这里就用了main2
//其实这样做增加了一些复杂性，之前的方法用户只需要编写一个main函数就可以了，其它什么不用管
//而现在则需要了解entry宏
entry!(main2);
static RODATA: &[u8] = b"Hello, world!";
static mut BSS: u8 = 0;
static mut DATA: u16 = 1;
fn main2() -> ! {
    let _x = RODATA;
    let _y = unsafe { &BSS };
    let _z = unsafe { &DATA };

    loop {}
}