//! # Add a water pump
//! 
//! In this exercise you will add a water pump to your plant pot.
//! 
//! At first we will test the pump using serial input from your development laptop.



#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_rp_io::usb::BasicUsbSetup;
use water_pump::{Irqs, run_water_pump};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    spawner.spawn(run_water_pump(on_board_pump)).unwrap();

    BasicUsbSetup::new(p.USB, Irqs)
        .receive(
            async |_bytes| todo!("Write a parser that handles incoming USB data"),
            spawner,
        )
        .await
}


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
