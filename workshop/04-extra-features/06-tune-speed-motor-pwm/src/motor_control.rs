use embassy_rp::{gpio::Output, pwm::PwmOutput};

use crate::HUMIDITY_PUBSUB_CHANNEL;

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
