#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::config::{self};
// Provides a panic handler.
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let _p = embassy_rp::init(config::Config::default());

    todo!("Turn on the onboard LED");
}
