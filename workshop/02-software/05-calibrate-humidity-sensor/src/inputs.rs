use embassy_rp::adc::{Adc, Async, Channel};
use embassy_time::{Duration, Ticker};

use crate::HUMIDITY_PUBSUB_CHANNEL;

fn adc_reading_to_voltage(_adc_reading_12bit: u16) -> f32 {
    todo!("Convert ADC reading to voltage");
}

fn voltage_to_humidity(_voltage: f32) -> f32 {
    todo!("Convert voltage to humidity");
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
