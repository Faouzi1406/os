#![no_main]
#![no_std]
use core::panic::PanicInfo;
mod vga_buffer;



#[no_mangle]
pub extern "C" fn _start() -> !{
    vga_buffer::print_something();
    loop {
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
    }
}


