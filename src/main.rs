#![no_std]
#![no_main]
use core::panic::PanicInfo;

mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Some test thingy")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", yeah these are numbers: {} {}",
        42,
        1.337
    )
    .unwrap();

    loop {}
}
