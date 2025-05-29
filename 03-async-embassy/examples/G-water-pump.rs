//! # Add a water pump
//!
//! In this exercise you will add a water pump to your plant pot.
//!
//! At first we will test the pump using serial input from your development
//! laptop.

#![no_std]
#![no_main]

use async_embassy::usb::BasicUsbSetup;
use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    bind_interrupts,
    config::{self},
    gpio::{Level, Output},
    peripherals::USB,
};
use panic_probe as _;

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

bind_interrupts!(
    pub struct Irqs {
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    }
);

const _TARGET_HUMIDITY: f32 = 0.3;

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>) {
    todo!("Write a loop to read the humidity and turn the pump on or off.");
}
