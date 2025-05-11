#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use rp_pico::hal;

use hal::entry;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use rp_pico::hal::adc::AdcPin;

use embedded_hal_0_2::adc::OneShot;
/// External high-speed crystal on the Raspberry Pi Pico board is 12 MHz. Adjust
/// if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_12BIT: u16 = 4096;
    (f32::from(reading_12bit) / f32::from(STEPS_12BIT)) * REFERENCE_VOLTAGE
}

const HUMIDITY_THRESH_PERC: f32 = 0.1;

fn voltage_to_humidity(voltage: f32) -> f32 {
    const AIR_V: f32 = 2.77;
    const WATER_V: f32 = 1.4;
    -(voltage - WATER_V) / (AIR_V - WATER_V)
}

#[entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);

    let mut humidity_pin = AdcPin::new(pins.gpio26).unwrap();

    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        let digital_value: u16 = adc.read(&mut humidity_pin).unwrap();
        let sensor_volt = adc_reading_to_voltage(digital_value);
        let humidity = voltage_to_humidity(sensor_volt);

        if humidity < HUMIDITY_THRESH_PERC {
            led_pin.set_high().unwrap();
        } else {
            led_pin.set_low().unwrap();
        }

        // TODO: Maybe this delay can be removed.
        timer.delay_ms(500);
    }
}
