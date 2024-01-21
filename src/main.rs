#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Heshlou agen -> Write_str into VGA Buffer\n").unwrap();
    write!(vga_buffer::WRITER.lock(), "Writer into buffer via macro -> some garbage: {} {} {}", 42, 3.14, 127).unwrap();
    loop {}
}