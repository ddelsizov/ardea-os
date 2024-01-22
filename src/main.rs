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
    ardea_os::init();
    println!("Heshlou{}; {}", "!", "It did not crash!");
    loop {}
}