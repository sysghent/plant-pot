//! # Hysteresis motor control
//!
//! You can switch the pump's motor on and off with a hysteresis controller.
//! This is a simple control loop that turns the pump on when the humidity is
//! below a certain threshold and turns it off when the humidity is above a
//! certain threshold. This is a simple way to control the pump without using a
//! PID controller.

#![no_std]
#![no_main]

use async_embassy::usb::BasicUsbSetup;
use defmt::info;
use defmt_rtt as _;
use embassy_rp::{
    bind_interrupts,
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 1, 1> = PubSubChannel::new();

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    config::{self},
    gpio::{Level, Output},
};

#[main]
async fn main(spawner: Spawner) -> ! {
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

const _TARGET_HUMIDITY: f32 = 0.3;

struct Hysteresis {}

impl Hysteresis {
    fn new(_target: f32, _margin: f32) -> Self {
        todo!("Implement hysteresis constructor.");
    }

    fn update(&mut self, _measured: f32) -> bool {
        todo!("Implement hysteresis update logic.");
    }
}

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>) {
    let mut _config: Hysteresis = todo!("Implement hysteresis.");
    let mut humidity_receiver = PUBSUB_CHANNEL.subscriber().unwrap();
    info!("Starting water pump control loop");
    loop {
        let _humidity = humidity_receiver.next_message_pure().await;
        let _on: bool = todo!(
            "Implement logic that turns the pump on or off based on the target humidity and the \
             hysteresis margin."
        );

        if _on {
            _pump.set_high();
        } else {
            _pump.set_low();
        }
    }
}
