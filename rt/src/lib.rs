#![no_std]

use core::panic::PanicInfo;

// CHANGED!
#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    extern "Rust" {
        fn main() -> !;//将控制权交给用户的main函数，因此main必须是发散的
    }

    main()
}