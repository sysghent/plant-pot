//! This example shows how async gpio can be used with a RP2040.
//!
//! The LED on the RP Pico W board is connected differently. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::fmt::Write;

use defmt::{info, unwrap};
use embassy_executor::{Executor, Spawner};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config, Mode},
    bind_interrupts, gpio,
    multicore::{Stack, spawn_core1},
    peripherals::USB,
    usb::Driver,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_usb::{
    UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
    driver::EndpointError,
};
use gpio::{Input, Level, Output, Pull};
use heapless::String;
use panic_halt as _;
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
    let p = embassy_rp::init(Default::default());

    let adc = Adc::new(p.ADC, Irqs, Config::default());

    let p26 = Channel::new_pin(p.PIN_26, Pull::None);

    let led = Output::new(p.PIN_25, Level::Low);

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(MEASURE_STACK) },
        move || {
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| {
                spawner.spawn(measure_humidity(adc, p26)).unwrap();
                spawner.spawn(toggle_led(led)).unwrap();
            });
        },
    );

    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let config = {
        let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
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

        let builder = embassy_usb::Builder::new(
            driver,
            config,
            CONFIG_DESCRIPTOR.init([0; 256]),
            BOS_DESCRIPTOR.init([0; 256]),
            &mut [], // no msos descriptors
            CONTROL_BUF.init([0; 64]),
        );
        builder
    };

    // Create classes on the builder.
    let mut class = {
        static STATE: StaticCell<State> = StaticCell::new();
        let state = STATE.init(State::new());
        CdcAcmClass::new(&mut builder, state, 64)
    };

    // Build the builder.
    let usb = builder.build();

    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        spawner.spawn(usb_task(usb)).unwrap();
        spawner.spawn(send_humidity(class)).unwrap();
    });
}

static mut MEASURE_STACK: Stack<4096> = Stack::new();

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_12BIT: u16 = u16::pow(2, 12);
    (f32::from(reading_12bit) / f32::from(STEPS_12BIT)) * REFERENCE_VOLTAGE
}

fn voltage_to_humidity(voltage: f32) -> f32 {
    const AIR_V: f32 = 2.77;
    const WATER_V: f32 = 1.4;
    -(voltage - WATER_V) / (AIR_V - WATER_V)
}

#[embassy_executor::task]
async fn send_humidity(mut class: CdcAcmClass<'static, MyUsbDriver>) {
    let mut string = String::<64>::new();
    let mut subscriber = CHANNEL.subscriber().unwrap();
    loop {
        let humidity = subscriber.next_message_pure().await;
        string.clear();
        write!(&mut string, "Humidity: {:.2} %\r\n", humidity).unwrap();
        class.write_packet(string.as_bytes()).await.unwrap();
    }
}

#[embassy_executor::task]
async fn toggle_led(mut led: Output<'static>) {
    const HUMIDITY_THRESH_PERC: f32 = 0.1;

    let mut subscriber = CHANNEL.subscriber().unwrap();
    loop {
        let humidity = subscriber.next_message_pure().await;

        if humidity < HUMIDITY_THRESH_PERC {
            led.set_high();
        } else {
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

static CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 2, 1> = PubSubChannel::new();

#[embassy_executor::task]
async fn measure_humidity(mut adc: Adc<'static, Async>, mut humidity_pin: Channel<'static>) {
    let publisher = CHANNEL.publisher().unwrap();
    loop {
        let level = adc.read(&mut humidity_pin).await.unwrap();
        let humidity = voltage_to_humidity(adc_reading_to_voltage(level));
        publisher.publish(humidity).await;
    }
}

type MyUsbDriver = Driver<'static, USB>;
type MyUsbDevice = UsbDevice<'static, MyUsbDriver>;

#[embassy_executor::task]
async fn usb_task(mut usb: MyUsbDevice) -> ! {
    usb.run().await
}
