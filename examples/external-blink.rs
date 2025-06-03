#![no_std]
#![no_main]

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

    let mut led_pin = Output::new(p.PIN_16, Level::High);

    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));

    loop {
        led_pin.set_high();
        ticker.next().await;
        led_pin.set_low();
        ticker.next().await;
    }
}
