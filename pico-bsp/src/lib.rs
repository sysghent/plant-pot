#![no_std]

pub mod humidity;
pub mod usb;
use panic_halt as _;

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_12BIT: u16 = u16::pow(2, 12);
    (f32::from(reading_12bit) / f32::from(STEPS_12BIT)) * REFERENCE_VOLTAGE
}
