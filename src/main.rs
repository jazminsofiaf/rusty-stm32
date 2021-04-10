//to compile: xargo build --target=thumbv7m-none-eabi --release

#![no_main] // <- no default main
#![no_std] // <- no standart

extern crate cortex_m;

#[macro_use(entry)]
extern crate cortex_m_rt as rt;

// makes `panic!` print messages to the host stderr using semihosting
extern crate panic_semihosting;

// the program entry point is ...
entry!(main);

// never ending function
fn main() -> ! {
    loop {
    }
}
