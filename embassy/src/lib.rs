#![no_std]
use defmt_rtt as _;
use embassy_rp::{bind_interrupts, peripherals::USB};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;
pub mod monitor_output;
pub mod sensing;
pub mod usb;

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 2, 1> =
    PubSubChannel::new();

#[embassy_executor::task]
pub async fn idle() {
    loop {
        embassy_futures::yield_now().await;
    }
}
