//! # Serial USB input
//! 
//! In this exercise you will find a way to inject or send commands to the Pico from your laptop over USB. This can be used for debugging the functioning of peripherals or electrical components (such as the pump).
//! 

#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};
use embassy_rp_io::usb::BasicUsbSetup;

use serial_input::Irqs;

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let _led_pin = Output::new(p.PIN_11, Level::Low);

    BasicUsbSetup::new(p.USB, Irqs)
        .receive(
            async |_bytes| {
                todo!("Toggle the led_pin based on the received bytes");
            },
            spawner,
        )
        .await
}



#![no_std]

use defmt_rtt as _;
use embassy_rp::{bind_interrupts, peripherals::USB};
use panic_probe as _;
bind_interrupts!(
    pub struct Irqs {
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;

    }
);
