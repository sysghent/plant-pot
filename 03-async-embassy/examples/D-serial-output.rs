//! In this exercise you will learn how to send binary data over a serial
//! connection that runs on an USB-connection with your laptop.

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    bind_interrupts,
    config::{self},
    peripherals::USB,
    usb::Driver,
};
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
};
use heapless::String;
use panic_probe as _;
use static_cell::StaticCell;
#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spam_serial_monitor(usb_io_handle).await
}

bind_interrupts!(
    pub struct Irqs {
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    }
);

pub async fn spam_serial_monitor(mut _usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>) -> ! {
    let mut serial_msg_buf = String::<64>::new();
    let counter = 0;
    loop {
        serial_msg_buf.clear();
        write!(&mut serial_msg_buf, "Counter: {counter} %\r\n").unwrap();
        todo!(
            "Implement sending the counter over the serial USB connection and try reading it in a \
             serial monitor on your laptop."
        );
        counter += 1;
    }
}

pub type StaticUsbDriver = Driver<'static, USB>;
pub type StaticUsbDevice = UsbDevice<'static, StaticUsbDriver>;

pub struct UsbSetup {
    pub usb_runtime: StaticUsbDevice,
    pub usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>,
}

impl UsbSetup {
    pub fn new(usb_pin: USB) -> Self {
        // Create the driver, from the HAL.
        let usb_driver = Driver::new(usb_pin, Irqs);

        // Create embassy-usb Config
        let usb_config = {
            let mut usb_config: embassy_usb::Config<'static> =
                embassy_usb::Config::new(0xc0de, 0xcafe);
            usb_config.manufacturer = Some("Raspberry");
            usb_config.product = Some("Pi Pico (flashed with Embassy)");
            usb_config.serial_number = Some("00000000");
            usb_config.max_power = 100;
            usb_config.max_packet_size_0 = 64;
            usb_config
        };

        // Create embassy-usb DeviceBuilder using the driver and config.
        // It needs some buffers for building the descriptors.
        let mut usb_runtime_builder = {
            static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
            static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
            static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

            embassy_usb::Builder::new(
                usb_driver,
                usb_config,
                CONFIG_DESCRIPTOR.init([0; 256]),
                BOS_DESCRIPTOR.init([0; 256]),
                &mut [], // no msos descriptors
                CONTROL_BUF.init([0; 64]),
            )
        };

        // Create classes on the builder.
        let usb_io_handle = {
            static USB_STATE: StaticCell<State> = StaticCell::new();
            let usb_state_ref = USB_STATE.init(State::new());
            CdcAcmClass::new(&mut usb_runtime_builder, usb_state_ref, 64)
        };

        // Build the builder.
        let usb_runtime = usb_runtime_builder.build();
        Self {
            usb_runtime,
            usb_io_handle,
        }
    }
}

#[embassy_executor::task]
pub async fn maintain_usb_connection(mut usb_runtime: StaticUsbDevice) -> ! {
    usb_runtime.run().await
}
