//! # Read humidity with ADC
//!
//! In this exercise you will read an output value of the ADC. You will use this
//! value as input to compute the humidity later on.
//!
//! For now, you just have to know that the ADC unit measures analogue values
//! and converts them into digital values. It does this kind of independently of
//! the CPU and pushes digital values into a FIFO queue / buffer. The CPU can
//! then read the buffer and use the digital values.
//!
//! We will only use the latest ADC value in the FIFO buffer.
//!
//! _*Important*: Before you can use the ADC in Rust, you have to manually
//! initialize the clocks._

#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts,
    config::{self},
    gpio::{Level, Output, Pull},
};
use embassy_time::{Duration, Ticker, Timer};
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc = Adc::new(p.ADC, Irqs, Config::default());
    let adc_channel = Channel::new_pin(p.PIN_26, Pull::None); // ADC0

    let led_pin = Output::new(p.PIN_22, Level::Low);
    let cap_pin = Output::new(p.PIN_21, Level::Low); // GPIO21 to charge/discharge the capacitor

    capacitive_sensor(adc, adc_channel, led_pin, cap_pin).await
}

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
    }
);

#[allow(clippy::cast_possible_truncation)]
pub async fn capacitive_sensor(
    mut adc: Adc<'static, Async>,
    mut adc_channel: Channel<'static>,
    mut led_pin: Output<'static>,
    mut cap_pin: Output<'static>,
) -> ! {
    let mut ticker = Ticker::every(Duration::from_millis(500));
    let adc_threshold: u16 = 1000; // Lower threshold for discharge detection
    let max_cycles: u32 = 10_000; // Max loop count to avoid infinite loop

    loop {
        ticker.next().await;

        // 1. Discharge the capacitor
        cap_pin.set_low();
        Timer::after_millis(5).await;

        // 2. Charge the capacitor
        cap_pin.set_high();
        Timer::after_millis(2).await;

        // 3. Set pin low to start discharge (simulate high-Z)
        cap_pin.set_low();
        // 4. Measure discharge time
        let mut cycles = 0u32;
        loop {
            let level = adc.read(&mut adc_channel).await.unwrap();
            if level < adc_threshold || cycles >= max_cycles {
                break;
            }
            cycles += 1;
        }

        // 5. Map cycles to percentage (simple linear mapping, tune as needed)
        let percent = if cycles >= max_cycles {
            0
        } else if cycles > 1000 {
            100
        } else {
            (cycles * 100 / 1000) as u8
        };

        // 6. Output value and LED
        info!("Capacitive value: {}% (cycles: {})", percent, cycles);
        if percent > 50 {
            led_pin.set_high();
        } else {
            led_pin.set_low();
        }
    }
}
