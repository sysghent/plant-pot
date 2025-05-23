use embassy_rp::{gpio::Output, pwm::PwmOutput};
use embassy_time::{Duration, Timer};

use crate::HUMIDITY_PUBSUB_CHANNEL;

#[embassy_executor::task]
pub async fn run_water_pump(mut pump: Output<'static>, mut pwm: PwmOutput<'static>) {
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        if humidity < 0.1 {
            pump.set_high();
        } else {
            pump.set_low();
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}
