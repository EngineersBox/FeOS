#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(fe_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use fe_os::println;
use bootloader::{BootInfo, entry_point};

use alloc::boxed::Box;

extern crate alloc;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use fe_os::memory::{self, BootInfoFrameAllocator};
    use fe_os::allocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    fe_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialisation failed");

    let x = Box::new(42);

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
