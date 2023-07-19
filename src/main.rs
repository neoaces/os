#![no_std]
#![no_main]

mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Prevents the compiler from changing the name of _start()
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello!").unwrap();

    // With write! macro for more type support
    write!(vga_buffer::WRITER.lock(), "{}", 7).unwrap();
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


