#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::{board::Board, hal::timer::Timer};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("RTT is now enabled!");

    let mut board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER1);

    let _ = board.display_pins.col1.set_low();
    let mut row1 = board.display_pins.row1;

    loop {
        let _ = row1.set_low();
        rprintln!("LED is OFF");
        timer.delay_ms(1_000);
        let _ = row1.set_high();
        rprintln!("LED is ON");
        timer.delay_ms(1_000);
    }
}
