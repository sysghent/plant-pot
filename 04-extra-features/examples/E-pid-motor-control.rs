//! # Change pump duration with PID
//!
//! A PID controller is a control loop feedback mechanism (or control loop)
//! widely used in industrial control systems. It continuously calculates an
//! error value as the difference between a desired setpoint and a measured
//! process variable and applies a correction based on proportional, integral,
//! and derivative terms (denoted P, I, and D).
//!
//! Implement a PID controller to control the pump's motor duration. For this,
//! you will need to find values for P, I, and D. Use the calculated correction
//! value to lengthen or shorten the duration of the pump's motor.

#![no_std]
#![no_main]

use async_embassy::usb::BasicUsbSetup;
use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    bind_interrupts,
    config::{self},
    gpio::{Level, Output},
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

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    // info!("Initializing peripherals");
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

struct PidController {
    kp: f32,
    ki: f32,
    kd: f32,
    prev_error: f32,
    integral: f32,
}

impl PidController {
    fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            prev_error: 0.0,
            integral: 0.0,
        }
    }

    fn update(&mut self, setpoint: f32, measured: f32, dt: f32) -> f32 {
        let error = setpoint - measured;
        self.integral += error * dt;
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;
        self.kp * error + self.ki * self.integral + self.kd * derivative
    }
}

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>) {
    const TARGET_HUMIDITY: f32 = 0.3; // Target humidity (30%)
    const DT: f32 = 1.0; // Assume 1s between samples (tune as needed)
    let mut pid = PidController::new(2.0, 0.5, 0.0); // Tune these gains
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;
        let control = pid.update(TARGET_HUMIDITY, humidity, DT);

        // Map PID output to PWM duty cycle (0.0 - 1.0) and duration (seconds)
        let intensity = control.clamp(0.0, 1.0); // Clamp to [0,1]
        let min_duration = 0.5; // Minimum pump run time in seconds
        let max_duration = 3.0; // Maximum pump run time in seconds
        let _duration = min_duration + (max_duration - min_duration) * intensity;

        if intensity > 0.01 {
            let duty = intensity * 100.0; // Map to 0-100%

            todo!("Set duty cycle high to {}%", duty);
        } else {
            todo!("Set duty cycle low");
        }
    }
}
