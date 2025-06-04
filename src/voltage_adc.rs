#[must_use]
pub fn adc_reading_to_voltage(raw_adc_output: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_16BIT: u32 = u32::pow(2, 16);
    (f32::from(raw_adc_output) / STEPS_16BIT as f32) * REFERENCE_VOLTAGE
}

#[must_use]
pub fn voltage_to_moisture(voltage: f32) -> f32 {
    const AIR_V: f32 = 0.178;
    const WATER_V: f32 = 0.0910;
    1.0 - (voltage - WATER_V) / (AIR_V - WATER_V)
}
