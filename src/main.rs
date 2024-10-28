#![no_std]
#![no_main]

use panic_semihosting as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

mod peripheral;
use peripheral::Peripherals;
use peripheral::PinMode::*;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let port_a = &mut peripherals.port_a;
    port_a.set_mode([
        GPOut,
        GPOut,
        GPIn,
        Analogue,
        GPIn,
        Analogue,
        GPIn,
        AltFunc,
        AltFunc,
        GPIn,
        GPIn,
        GPOut,
        GPOut,
        GPIn,
        Analogue,
        GPIn,
    ]);

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);
    loop {}
}
