use embassy_rp::adc::{Adc, Async, Channel};
use embassy_time::{Duration, Ticker};

use crate::HUMIDITY_PUBSUB_CHANNEL;

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
