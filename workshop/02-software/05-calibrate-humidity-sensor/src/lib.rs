#![no_std]

pub mod calc;

use calc::{adc_reading_to_voltage, voltage_to_humidity};
use embassy_rp::adc::{Adc, Async, Channel};
use embassy_time::{Duration, Ticker};

use embassy_executor::Spawner;
use embassy_rp::peripherals::USB;
use embassy_rp_io::usb::BasicUsbSetup;
use embedded_io::Write;

use num_traits::float::FloatCore;

use defmt_rtt as _;
use embassy_rp::{bind_interrupts, peripherals::PIO0, pio::InterruptHandler};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

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
