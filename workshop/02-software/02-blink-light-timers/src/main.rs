#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_time::Ticker;
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let _led_pin = Output::new(p.PIN_22, Level::Low);

    let mut _ticker: Ticker =
        todo!("Create a ticker with a 1 second interval that toggles the led_pin");
}
