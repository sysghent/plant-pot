use embassy_rp::gpio::Output;

use crate::HUMIDITY_PUBSUB_CHANNEL;

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
