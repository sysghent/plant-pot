#![no_std]

pub mod motor_control;

use defmt_rtt as _;
use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 1, 1> = PubSubChannel::new();
