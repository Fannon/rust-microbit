#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use embedded_hal::digital::{OutputPin, PinState};
use hal::{gpio::Level, pac};
use nrf52833_hal as hal;

use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialize RTT
    rtt_init_print!();

    // Example usage of RTT for logging
    rprintln!("RTT is now enabled!");

    let p = pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(p.P0);
    let _col1 = port0.p0_28.into_push_pull_output(Level::Low);
    let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);

    let mut is_on: bool = false;
    loop {
        row1.set_state(PinState::from(is_on)).unwrap();
        for _ in 0..100_000 {
            nop();
        }
        is_on = !is_on;
        rprintln!("PIN pin21: {}", is_on);
    }
}
