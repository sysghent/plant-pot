//! # Calibrate humidity sensor
//!
//! In this exercise you need to find a way to transform raw ADC values into a
//! humidity value.
//!
//! The ADC values are n-bit. Find how many bits are used for the ADC on the
//! Pico. Then find the minimum and maximum for a cup of water and a air.
//!
//! Use this to make a formula that transforms the ADC value into a humidity
//! value.
//!
//! This step is similar to the `map` function in ArduinoIDE. It is just a
//! linear transformation.

#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts,
    config::{self},
    gpio::Pull,
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::{Duration, Ticker};
use embedded_io::Write;
use num_traits::float::FloatCore;
use panic_probe as _;

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    spawner
        .spawn(measure_humidity(adc_component, humidity_adc_channel))
        .unwrap();

    send_humidity_serial_usb(p.USB, spawner).await
}

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

#[must_use]
pub fn adc_reading_to_voltage(_adc_reading_12bit: u16) -> f32 {
    todo!("Convert ADC reading to voltage");
}

#[must_use]
pub fn voltage_to_humidity(_voltage: f32) -> f32 {
    todo!("Convert voltage to humidity");
}

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

#[embassy_executor::task]
pub async fn measure_humidity(mut adc: Adc<'static, Async>, mut humidity_pin: Channel<'static>) {
    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;
        let level = adc.read(&mut humidity_pin).await.unwrap();
        let voltage = adc_reading_to_voltage(level);
        let humidity = voltage_to_humidity(voltage);
        publisher.publish_immediate(humidity);
    }
}

pub async fn send_humidity_serial_usb(usb: USB, spawner: Spawner) -> ! {
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    BasicUsbSetup::new(usb, Irqs)
        .send(
            async |mut buf| {
                let humidity = humidity_receiver.next_message_pure().await;
                let humidity_perc = (humidity * 100.0).floor();
                write!(buf, "Humidity: {humidity_perc} %\r\n").unwrap();
            },
            spawner,
        )
        .await
}
