#![no_std]
#![no_main]

use ardea_os::{println, serial_println};
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello, this is println! macro from vga_buffer lib");
    serial_println!("Hi, this is serial_println! macro form serial lib");
    // panic!("the disco")

    loop {}
}