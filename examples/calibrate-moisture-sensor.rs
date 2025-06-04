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
    peripherals::USB,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::{Duration, Ticker};
use embedded_io::Write;
use num_traits::float::FloatCore;
use panic_probe as _;
use plant_pot::usb::BasicUsbSetup;

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let moisture_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    spawner
        .spawn(measure_moisture(adc_component, moisture_adc_channel))
        .unwrap();

    send_moisture_serial_usb(p.USB, spawner).await
}

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    }
);

#[must_use]
pub fn adc_reading_to_voltage(_adc_pin_input: u16) -> f32 {
    todo!("Convert ADC reading to voltage");
}

#[must_use]
pub fn voltage_to_moisture(_voltage: f32) -> f32 {
    todo!("Convert voltage to moisture");
}

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

#[embassy_executor::task]
pub async fn measure_moisture(mut adc: Adc<'static, Async>, mut moisture_pin: Channel<'static>) {
    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;
        let level = adc.read(&mut moisture_pin).await.unwrap();
        let voltage = adc_reading_to_voltage(level);
        let moisture = voltage_to_moisture(voltage);
        publisher.publish_immediate(moisture);
    }
}

pub async fn send_moisture_serial_usb(usb: USB, spawner: Spawner) -> ! {
    let mut moisture_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    BasicUsbSetup::new(usb, Irqs)
        .send(
            async |mut buf| {
                let moisture = moisture_receiver.next_message_pure().await;
                let moisture_perc = (moisture * 100.0).floor();
                write!(buf, "Humidity: {moisture_perc} %\r\n").unwrap();
            },
            spawner,
        )
        .await
}
