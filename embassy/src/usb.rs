use embassy_rp::{peripherals::USB, usb::Driver};
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};
use panic_probe as _;
use static_cell::StaticCell;

use crate::Irqs;

pub type StaticUsbDriver = Driver<'static, USB>;
pub type StaticUsbDevice = UsbDevice<'static, StaticUsbDriver>;

pub struct UsbSetup {
    pub usb_runtime: StaticUsbDevice,
    pub usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>,
}

impl UsbSetup {
    pub fn new(usb_pin: USB) -> Self {
        // Create the driver, from the HAL.
        let driver = Driver::new(usb_pin, Irqs);

        // Create embassy-usb Config
        let config = {
            let mut config: embassy_usb::Config<'static> = embassy_usb::Config::new(0xc0de, 0xcafe);
            config.manufacturer = Some("Embassy");
            config.product = Some("USB-serial example");
            config.serial_number = Some("12345678");
            config.max_power = 100;
            config.max_packet_size_0 = 64;
            config
        };

        // Create embassy-usb DeviceBuilder using the driver and config.
        // It needs some buffers for building the descriptors.
        let mut builder = {
            static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
            static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
            static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

            embassy_usb::Builder::new(
                driver,
                config,
                CONFIG_DESCRIPTOR.init([0; 256]),
                BOS_DESCRIPTOR.init([0; 256]),
                &mut [], // no msos descriptors
                CONTROL_BUF.init([0; 64]),
            )
        };

        // Create classes on the builder.
        let usb_io_handle = {
            static STATE: StaticCell<State> = StaticCell::new();
            let state = STATE.init(State::new());
            CdcAcmClass::new(&mut builder, state, 64)
        };

        // Build the builder.
        let usb_runtime = builder.build();
        Self {
            usb_runtime,
            usb_io_handle,
        }
    }
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}
