#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
    pwm::{self, Pwm},
};
use water_pump::{
    control::run_water_pump,
    usb_input::{UsbSetup, maintain_usb_connection, receive_input},
};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    let pwm = Pwm::new_output_a(p.PWM_SLICE3, p.PIN_22, pwm::Config::default());

    spawner
        .spawn(run_water_pump(on_board_pump, pwm.split().0.unwrap()))
        .unwrap();

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner.spawn(receive_input(usb_io_handle)).unwrap();

    loop {
        yield_now().await;
    }
}
