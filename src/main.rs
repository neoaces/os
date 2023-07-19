#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)] 
#![reexport_test_harness_main = "test_main"]
// Requires renaming generated main function

mod vga_buffer;

// static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // Prevents the compiler from changing the name of _start()
pub extern "C" fn _start() -> ! {
    // use core::fmt::Write;
    // // With write! macro for more type support
    // vga_buffer::WRITER.lock().write_string("Hellos"); // 1. Basic
    
    // // 2. write! macro from Volatile<ScreenChar>
    // // * forces each write, will not be optimized out of rust
    // write!(vga_buffer::WRITER.lock(), "{}", 7).unwrap();  
    
    // 3. custom println! macro
    // scoped to entire crate, so no need to import.
    println!("Hello other world!"); // Custom println! macro taken from std lib
    println!("Hello other world!"); // Custom println! macro taken from std lib
    // panic!("Panic message");
    #[cfg(test)]
    test_main();

    loop {}
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

//
// QEMU
//
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        // Refers to the iobase port of the isa-debug-exit of QEMU
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// 
// TESTING SUITE
//
// Define test runner
#[cfg(test)] // Only used when running tests.
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests.", tests.len());
    // "Slice of trait object references of the Fn() trait"
    // - All types that can be run like a function implements Fn()
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}
