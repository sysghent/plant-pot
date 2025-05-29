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

use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts,
    config::{self},
    gpio::{Level, Output, Pull},
};
use embassy_time::{Duration, Ticker};
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let led_pin = Output::new(p.PIN_22, Level::Low);

    read_adc(adc_component, humidity_adc_channel, led_pin).await
}

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
    }
);

pub async fn read_adc(
    mut _adc: Adc<'static, Async>,
    mut _humidity_pin: Channel<'static>,
    _led_pin: Output<'static>,
) -> ! {
    let mut _ticker: Ticker = Ticker::every(Duration::from_millis(500));

    let _adc_threshold: u16 =
        todo!("Find an average ADC value, right after being measured by the ADC unit.");

    loop {
        _ticker.next().await;

        let _level: u16 = todo!(
            "Wait until first ADC value has been produced and stored in ADC queue. Then turn on \
             the LED"
        );
        if _level < _adc_threshold {
            _led_pin.set_high();
        } else {
            _led_pin.set_low();
        }
    }
}
