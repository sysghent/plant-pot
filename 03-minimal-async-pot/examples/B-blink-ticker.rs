//! # Blink light with timers
//!
//! In this exercise you will do the most minimal thing you can do with Embassy:
//! blink an LED with some interval.
//!
//! This is also called the "Hello World" of embedded programming.
//!
//! Notice that Embassy provide a `Duration` type that can be used for all
//! supported micro-controllers. This is a good way to make your code portable
//! between different platforms.
//!
//! ## Hardware
//!
//! You will need:
//!
//! - An external LED, red ones have a forward voltage of 1.8 V and maximum
//!   current of 20 mA.
//! - A resistor of 100 Ohm.
//! - Power supply of Pico with max output of 3.3 V.
//!
//!
//! The desired resistor resistance is:
//!
//! ```txt
//! R = (V_supply - V_forward) / I_max
//!  ```
//!
//! In later exercises you will see how to power the onboard LED (you need to access the wifi module).

#![no_std]
#![no_main]

// use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_time::{Duration, Ticker};
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let mut led_pin = Output::new(p.PIN_27, Level::High);

    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));

    led_pin.set_high();

    loop {
        led_pin.set_high();
        ticker.next().await;
        led_pin.set_low();
        ticker.next().await;
    }
}
