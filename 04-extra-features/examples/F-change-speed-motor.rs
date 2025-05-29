//! # Tune motor speed with PWM
//!
//! Change the speed of the pump's motor is not strictly necessary. However, it
//! is common in other projects.
//!
//! In this project you will find a way to slow down the motor using pulse width
//! modulation (PWM). This is a technique used to control the amount of power
//! delivered to an electrical device by varying the width of the pulses in a
//! pulse train.
//!
//! The average power delivered to the load is proportional to the duty cycle of
//! the pulse train.
//!
//! See <https://pico.implrust.com/led/pwm-rp2350.html>
//!
//! ## `PioPWM`
//!
//! The Pico board also has multiple PIO peripherals. This is a programmable
//! input/output peripheral that can be used to implement custom protocols and
//! control devices.
//!
//! Creating a PWM output with the PIO peripheral requires more work, but may be
//! more performant than using simpler ways to drive PWM outputs.
//!
//! See <https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/pio_pwm.rs>

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
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
    pwm::{self, Pwm, PwmOutput},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use panic_probe as _;

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

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>, mut _pwm: PwmOutput<'static>) {
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let _humidity = humidity_receiver.next_message_pure().await;

        let _intensity: f32 =
            todo!("Compute the intensity for PWM to tune the speed of the motor."); // Clamp to [0,1]

        todo!("Pump with the given intensity using pulse width modulation (PWM).");
    }
}
