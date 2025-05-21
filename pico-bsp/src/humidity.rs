use embedded_hal::digital::OutputPin;
use panic_halt as _;
use rp_pico::{
    hal::{self, adc::AdcPin},
    pac,
};

use crate::adc_reading_to_voltage;

fn voltage_to_humidity(voltage: f32) -> f32 {
    const AIR_V: f32 = 2.77;
    const WATER_V: f32 = 1.4;
    -(voltage - WATER_V) / (AIR_V - WATER_V)
}

const HUMIDITY_THRESH_PERC: f32 = 0.1;

pub fn measure_humidity() {
    let mut pac = unsafe { pac::Peripherals::steal() };

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);

    let mut humidity_pin = AdcPin::new(pins.gpio26).unwrap();
    let mut adc_queue = adc.build_fifo().set_channel(&mut humidity_pin).start();
    let mut on_board_led_pin = pins.led.into_push_pull_output();

    let mut sio_queue = sio.fifo;
    loop {
        if adc_queue.len() > 0 {
            let digital_value: u16 = adc_queue.read();
            let sensor_volt = adc_reading_to_voltage(digital_value);
            let humidity = voltage_to_humidity(sensor_volt);
            if sio_queue.is_write_ready() {
                #[allow(clippy::cast_sign_loss)]
                #[allow(clippy::cast_possible_truncation)]
                sio_queue.write_blocking((humidity * 100.0) as u32);
            }
            if humidity < HUMIDITY_THRESH_PERC {
                on_board_led_pin.set_high().unwrap();
            } else {
                on_board_led_pin.set_low().unwrap();
            }
        }
    }
}
