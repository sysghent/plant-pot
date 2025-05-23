#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use serial_input::usb_input::{UsbSetup, maintain_usb_connection, receive_input};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let led_pin = Output::new(p.PIN_11, Level::Low);

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner
        .spawn(receive_input(usb_io_handle, led_pin))
        .unwrap();

    loop {
        yield_now().await;
    }
}
