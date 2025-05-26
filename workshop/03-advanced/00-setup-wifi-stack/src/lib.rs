#![no_std]

pub mod wifi;

use defmt_rtt as _;
use embassy_rp::{bind_interrupts, peripherals::PIO0, pio::InterruptHandler};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);
