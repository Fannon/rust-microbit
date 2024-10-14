#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    // Initialize RTT
    rtt_init_print!();

    // Example usage of RTT for logging
    rprintln!("RTT is now enabled!");
    loop {
        rprintln!("Loop...");
        for _ in 0..100_000 {
            nop();
        }
    }
}
