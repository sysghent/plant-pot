#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    config::{self},
    gpio::{Level, Output, Pull},
    pwm::{self, Pwm},
};
use hysteresis_motor::{
    Irqs,
    motor_control::run_water_pump,
    usb_input::{UsbSetup, maintain_usb_connection, receive_input},
};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    let pwm = Pwm::new_output_a(p.PWM_SLICE3, p.PIN_22, pwm::Config::default());

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner.spawn(receive_input(usb_io_handle)).unwrap();

    spawner
        .spawn(run_water_pump(on_board_pump, pwm.split().0.unwrap()))
        .unwrap();

    loop {
        yield_now().await;
    }
}
