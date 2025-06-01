//! # Calibrate capacitive sensor

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts,
    config::{self},
    gpio::{Level, Output, Pull},
};
use embassy_time::{Duration, Instant, Ticker, Timer};
use panic_probe as _;

const ADC_THRESHOLD: u16 = 1000; // Lower threshold for discharge detection
const DISCHARGE_DURATION_WATER: Duration = Duration::from_micros(100); // Max loop count to avoid infinite loop

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
    }
);

#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc = Adc::new(p.ADC, Irqs, Config::default());
    let adc_channel = Channel::new_pin(p.PIN_26, Pull::None); // Analogue voltage input on a pin that supports ADC (see pinout diagram)

    let led_pin = Output::new(p.PIN_22, Level::Low);
    let cap_pin = Output::new(p.PIN_21, Level::Low); // GPIO21 to charge/discharge the capacitor

    capacitive_sensor(adc, adc_channel, led_pin, cap_pin).await
}

#[allow(clippy::cast_possible_truncation)]
pub async fn capacitive_sensor(
    mut adc: Adc<'static, Async>,
    mut adc_channel: Channel<'static>,
    mut led_pin: Output<'static>,
    mut cap_pin: Output<'static>,
) -> ! {
    let mut ticker = Ticker::every(Duration::from_millis(500));

    loop {
        ticker.next().await;

        // Discharge the capacitor
        cap_pin.set_low();
        Timer::after_millis(5).await;

        // Charge the capacitor
        cap_pin.set_high();
        Timer::after_millis(2).await;

        // Set pin low to start discharge (simulate high-Z)
        cap_pin.set_low();

        let start = Instant::now();

        loop {
            let level = adc.read(&mut adc_channel).await.unwrap();
            if level < ADC_THRESHOLD {
                break;
            }
        }

        let discharge_dur = start.elapsed();

        // Map cycles to percentage (simple linear mapping, tune as needed)
        let moisture_ratio = moisture(discharge_dur);

        // Output value and LED
        if moisture_ratio > 0.5 {
            led_pin.set_high();
        } else {
            led_pin.set_low();
        }
    }
}

/// Dry environment -> smaller capacitance -> smaller time to discharge
/// Wet environment -> larger capacitance -> longer time to discharge
fn moisture(discharge_time: Duration) -> f32 {
    if discharge_time <= DISCHARGE_DURATION_WATER {
        (DISCHARGE_DURATION_WATER - discharge_time).as_ticks() as f32
            / (DISCHARGE_DURATION_WATER.as_ticks() as f32)
    } else {
        1.0
    }
}
