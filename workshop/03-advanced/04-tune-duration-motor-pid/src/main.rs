#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use tune_duration_motor_pid::{
    motor_control::run_water_pump,
    usb_input::{UsbSetup, maintain_usb_connection, receive_input},
};

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner.spawn(receive_input(usb_io_handle)).unwrap();

    spawner.spawn(run_water_pump(on_board_pump)).unwrap();

    loop {
        yield_now().await;
    }
}
