#![no_std]

use defmt_rtt as _;
use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;
pub mod inputs;
pub mod net;
pub mod outputs;
pub mod usb_setup;
pub mod wifi;
bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 2, 1> =
    PubSubChannel::new();
