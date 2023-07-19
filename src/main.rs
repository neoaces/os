#![no_std]
#![no_main]

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Prevents the compiler from changing the name of _start()
pub extern "C" fn _start() -> ! {
    vga_buffer::print_ex("HEllo world");
    
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


