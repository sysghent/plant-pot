#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
    pwm::{self, Pwm},
};
use embassy_rp_io::usb::BasicUsbSetup;
use tune_speed_motor_pwm::{Irqs, motor_control::run_water_pump};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    let pwm = Pwm::new_output_a(p.PWM_SLICE3, p.PIN_22, pwm::Config::default());

    spawner
        .spawn(run_water_pump(on_board_pump, pwm.split().0.unwrap()))
        .unwrap();

    BasicUsbSetup::new(p.USB, Irqs)
        .receive(
            async move |_bytes| {
                todo!(
                    "Write a function that parses the bytes and adjusts the speed of the motor \
                     accordingly with PWM."
                )
            },
            spawner,
        )
        .await
}
