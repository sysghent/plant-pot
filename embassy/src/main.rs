//! This example shows how async gpio can be used with a RP2040.
//!
//! The LED on the RP Pico W board is connected differently. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::{debug, info, trace};
use defmt_rtt as _;
use embassy_executor::Executor;
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts, gpio,
    multicore::{Stack, spawn_core1},
    peripherals::USB,
    usb::Driver,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::{Duration, Ticker};
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};
use gpio::{Level, Output, Pull};
use heapless::String;
use panic_probe as _;
use static_cell::StaticCell;
// use static_cell::StaticCell;

static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

bind_interrupts!(
    struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    }
);

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(Default::default());

    let adc = Adc::new(p.ADC, Irqs, Config::default());

    let p26 = Channel::new_pin(p.PIN_26, Pull::None);

    let led = Output::new(p.PIN_25, Level::Low);

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(SENSOR_VAR_STACK) },
        move || {
            debug!("Spawning executor on core 1 ");
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| {
                debug!("Spawning async tasks on core 1");
                spawner.spawn(measure_humidity(adc, p26)).unwrap();
                spawner.spawn(toggle_led(led)).unwrap();
            });
        },
    );

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    debug!("Spawning executor on core 0");
    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        debug!("Spawning async tasks on core 0");
        spawner.spawn(usb_task(usb)).unwrap();
        spawner.spawn(send_humidity(class)).unwrap();
    });
}

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

static mut SENSOR_VAR_STACK: Stack<4096> = Stack::new();

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_16BIT: u32 = u32::pow(2, 16);
    (f32::from(reading_12bit) / STEPS_16BIT as f32) * REFERENCE_VOLTAGE
}

fn voltage_to_humidity(voltage: f32) -> f32 {
    const AIR_V: f32 = 0.178;
    const WATER_V: f32 = 0.0910;
    1.0 - (voltage - WATER_V) / (AIR_V - WATER_V)
}

#[embassy_executor::task]
async fn send_humidity(mut class: CdcAcmClass<'static, StaticUsbDriver>) {
    let mut string = String::<64>::new();
    let mut subscriber = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = subscriber.next_message_pure().await;
        debug!("Received humidity");
        string.clear();
        write!(&mut string, "Humidity: {:.2} %\r\n", humidity).unwrap();
        debug!("Sending humidity over USB: {}", string.as_str());
        class.write_packet(string.as_bytes()).await.unwrap();
    }
}

#[embassy_executor::task]
async fn toggle_led(mut led: Output<'static>) {
    const HUMIDITY_THRESH_PERC: f32 = 0.1;

    let mut subscriber = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();
    loop {
        let humidity = subscriber.next_message_pure().await;

        if humidity < HUMIDITY_THRESH_PERC {
            trace!("Humidity is low, turning on LED");
            led.set_high();
        } else {
            trace!("Humidity is high, turning off LED");
            led.set_low();
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

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 2, 1> =
    PubSubChannel::new();

#[embassy_executor::task]
async fn measure_humidity(mut adc: Adc<'static, Async>, mut humidity_pin: Channel<'static>) {
    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;

        let level = adc.read(&mut humidity_pin).await.unwrap();
        debug!("ADC reading: {}", level);
        let voltage = adc_reading_to_voltage(level);
        debug!("Voltage: {}", voltage);
        let humidity = voltage_to_humidity(voltage);
        debug!("Humidity: {}", humidity);
        publisher.publish_immediate(humidity);
        debug!("Humidity published on internal channel");
    }
}

type StaticUsbDriver = Driver<'static, USB>;
type StaticUsbDevice = UsbDevice<'static, StaticUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: StaticUsbDevice) -> ! {
    debug!("Starting USB communication handling task");
    usb.run().await
}
