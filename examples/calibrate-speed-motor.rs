#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    bind_interrupts,
    config::{self},
    gpio::{Level, Output},
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
    pwm::{self, Pwm, PwmOutput, SetDutyCycle},
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::Timer;
use panic_probe as _;
use plant_pot::usb::BasicUsbSetup;

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
                // Write a function that parses the bytes  coming in over serial connection into moisture numbers. Use these numbers to change the speed of the water pump dynamiccally at runtime.
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
pub async fn run_water_pump(mut _pump: Output<'static>, mut pwm_output_pin: PwmOutput<'static>) {
    let mut moisture_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let moisture_percentage = moisture_receiver.next_message_pure().await;

        let motor_intensity: f32 = 1.0 * moisture_percentage; // TODO: Replace with actual intensity calculation

        let duty_cycle = (((2 ^ 16) as f32) * motor_intensity) as u16;

        pwm_output_pin.set_duty_cycle(duty_cycle).unwrap();
        Timer::after_secs(1).await;
    }
}
