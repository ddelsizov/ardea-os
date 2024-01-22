#![no_std]
#![no_main]

use ardea_os::{println /*, serial_println */ };
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Heshlou{}", "!");

    ardea_os::init();
    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    println!("It did not crash!");
    loop {}
}