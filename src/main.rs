#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sigmaos::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
use alloc::boxed::Box;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use sigmaos::println;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use sigmaos::allocator;
    use sigmaos::memory::{self, BootInfoFrameAllocator};

    println!("Hello World{}", "!");
    sigmaos::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    #[cfg(test)]
    test_main();

    sigmaos::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    sigmaos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sigmaos::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
