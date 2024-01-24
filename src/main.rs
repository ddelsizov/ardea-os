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
    println!("Hello World{}", "!");

    ardea_os::init();

    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
    println!("It did not crash!");
    loop {}
}