#![no_std]

pub mod inputs;
pub mod wifi;

use defmt_rtt as _;
use embassy_rp::{bind_interrupts, peripherals::PIO0, pio::InterruptHandler};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();
