#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_rp_io::usb::BasicUsbSetup;
use hysteresis_motor::{Irqs, motor_control::run_water_pump};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    spawner.spawn(run_water_pump(on_board_pump)).unwrap();

    BasicUsbSetup::new(p.USB, Irqs)
        .receive(async move |_| {
            todo!("Parse bytes and adjust hysteresis setup that the motor turns on and off at appropriate times.")
        }, spawner)
        .await
}
