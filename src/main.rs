#![no_std]
#![no_main]

mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Prevents the compiler from changing the name of _start()
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    // With write! macro for more type support
    write!(vga_buffer::WRITER.lock(), "{}", 7).unwrap();

    println!("Hello other world!"); // Custom println! macro taken from std lib
    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


