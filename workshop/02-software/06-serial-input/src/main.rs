#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_rp_io::usb::BasicUsbSetup;

use serial_input::Irqs;

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let _led_pin = Output::new(p.PIN_11, Level::Low);

    BasicUsbSetup::new(p.USB, Irqs)
        .receive(
            async |_bytes| {
                todo!("Toggle the led_pin based on the received bytes");
            },
            spawner,
        )
        .await
}
