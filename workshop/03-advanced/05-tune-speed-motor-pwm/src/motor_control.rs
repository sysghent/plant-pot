use embassy_rp::{gpio::Output, pwm::PwmOutput};

use crate::HUMIDITY_PUBSUB_CHANNEL;

const TARGET_HUMIDITY: f32 = 0.3;

#[embassy_executor::task]
pub async fn run_water_pump(mut pump: Output<'static>, mut pwm: PwmOutput<'static>) {
    const TARGET_HUMIDITY: f32 = 0.3; // Target humidity (30%)

    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        let intensity: f32 = todo!("Compute the intensity for PWM to tune the speed of the motor."); // Clamp to [0,1]
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
