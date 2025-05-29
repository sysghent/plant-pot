#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_rp_io::usb::BasicUsbSetup;
use tune_duration_motor_pid::{Irqs, motor_control::run_water_pump};

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    spawner.spawn(run_water_pump(on_board_pump)).unwrap();

    BasicUsbSetup::new(p.USB, Irqs)
        .receive(
            async move |_| {
                todo!(
                    "Parse bytes and adjust hysteresis setup that the motor turns on and off at \
                     appropriate times."
                )
            },
            spawner,
        )
        .await
}
