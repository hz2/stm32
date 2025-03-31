// indicates this program will not link to the standard library/crate,
// instead will link to its subset `core` which is a no_std crate
#![no_std]
// indicates this program will not use the interface of the standard `main`
// interface that rust programs use
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;

// attribute provided by the `cortex-m-rt` crate that indicates this is the entry point
// of the program. This is where the program starts executing.
#[entry]
fn main() -> ! {
    // -> ! indicates a divergent function, meaning this will be the only
    // process that runs on the hardware, and it will never return to the caller
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
