#![no_std]

use defmt_rtt as _;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_halt as _;

pub mod inputs;
pub mod outputs;
pub mod usb_setup;

pub static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 2, 1> =
    PubSubChannel::new();
