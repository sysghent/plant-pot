//! # Blink light with timers
//! 
//! In this exercise you will do the most minimal thing you can do with Embassy: blink an LED with some interval.
//! 
//! This is also called the "Hello World" of embedded programming.
//! 
//! Notice that Embassy provide a `Duration` type that can be used for all supported micro-controllers. This is a good way to make your code portable between different platforms.
//! 
//! Underneath the `async` front-end, Embassy creates a hardware timer. If you prefer to instead use a hardware timer directly, you can drop Embassy and use the `rp235x-hal` crate directly. This is a lower-level approach that requires more code to achieve the same result.
//! 


//! red 1.8 V

//! max digital out 3.3 V

//! typ. current 15 mA

//! R = V/ I = 1.5 V / 0.015 A = 100 Ohm

#![no_std]
#![no_main]

use cortex_m_rt as _;
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

    let mut led_pin = Output::new(p, Level::Low);

    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));

    led_pin.set_high();

    loop {
        led_pin.set_high();
        ticker.next().await;
        led_pin.set_low();
        ticker.next().await;
    }
}
