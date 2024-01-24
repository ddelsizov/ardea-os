#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod interrupts;
pub mod serial;
pub mod vga_buffer;

pub fn init() {
    interrupts::init_idt();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}