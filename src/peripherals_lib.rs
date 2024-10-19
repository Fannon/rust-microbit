#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialize RTT
    rtt_init_print!();

    // Example usage of RTT for logging
    rprintln!("RTT is now enabled!");

    let p = Peripherals::take().unwrap();

    p.P0.pin_cnf[21].write(|w| w.dir().output());
    p.P0.pin_cnf[28].write(|w| w.dir().output());

    let mut is_on: bool = false;
    loop {
        p.P0.out.write(|w| w.pin21().bit(is_on));
        for _ in 0..400_000 {
            nop();
        }
        is_on = !is_on;
        rprintln!("PIN pin21: {}", is_on);
    }
}
