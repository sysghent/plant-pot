#![no_std]

pub mod control;

use defmt_rtt as _;

use panic_probe as _;

use embassy_rp::gpio::Output;

const _TARGET_HUMIDITY: f32 = 0.3;

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>) {
    todo!("Write a loop to read the humidity and turn the pump on or off.");
}
