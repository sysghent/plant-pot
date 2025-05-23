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
use tune_duration_motor_pid::{
    Irqs,
    motor_control::run_water_pump,
    usb_input::{UsbSetup, maintain_usb_connection, receive_input},
};

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

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
