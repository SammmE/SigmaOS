#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sigmaos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use sigmaos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    sigmaos::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
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
