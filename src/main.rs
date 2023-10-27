#![no_std]
#![no_main]

use core::panic::PanicInfo;

// no mangling on a function name
// extern "C": use the C calling convention for this function
#[no_mangle]
pub extern "C" fn _start() -> !{
    loop {}
}

// we should define our own panic_handler in no_std environment
// panic_handler is called when a panic occurs 
#[panic_handler] 
fn panic(_info: &PanicInfo) -> ! {
    loop {} 
}
