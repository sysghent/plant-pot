use defmt::debug;
use embassy_time::{Duration, Ticker};
use esp_hal::{
    analog::adc::{Adc, AdcConfig},
    gpio::GpioPin,
    peripherals::ADC1,
};

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
pub async fn measure_humidity(adc_peripheral: ADC1, analogue_humidity_pin: GpioPin<0>) {
    let mut adc_config = AdcConfig::default();

    let mut adc_ready_pin = adc_config.enable_pin(
        analogue_humidity_pin,
        esp_hal::analog::adc::Attenuation::_11dB,
    );

    let mut adc_unit = Adc::new(adc_peripheral, adc_config);

    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut sensor_ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        sensor_ticker.next().await;

        let digital_level = adc_unit.read_oneshot(&mut adc_ready_pin).unwrap();
        debug!("ADC reading: {}", digital_level);
        let voltage = adc_reading_to_voltage(digital_level);
        debug!("Voltage: {}", voltage);
        let humidity = voltage_to_humidity(voltage);
        debug!("Humidity: {}", humidity);
        publisher.publish_immediate(humidity);
        debug!("Humidity published on internal channel");
    }
}
