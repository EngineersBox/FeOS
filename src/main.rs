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

    let ptr = 0xdeadbeaf as *mut u32;
    unsafe { *ptr = 42; }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    fe_os::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    fe_os::hlt_loop();
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
