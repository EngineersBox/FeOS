#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fe_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fe_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    fe_os::init();

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }
    stack_overflow();

    #[cfg(test)]
    test_main();

    println!("Execution proceeded");
    loop {}
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    fe_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
