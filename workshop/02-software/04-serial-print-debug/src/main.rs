#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::config::{self};
use serial_print_debug::{
    outputs::spam_serial_monitor,
    usb_setup::{UsbSetup, maintain_usb_connection},
};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner.spawn(spam_serial_monitor(usb_io_handle)).unwrap();

    loop {
        yield_now().await;
    }
}
