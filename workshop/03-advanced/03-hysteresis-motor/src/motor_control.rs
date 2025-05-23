use embassy_rp::{gpio::Output, pwm::PwmOutput};

use crate::HUMIDITY_PUBSUB_CHANNEL;

const TARGET_HUMIDITY: f32 = 0.3;

enum Toggled {
    On,
    Off,
}

struct Hysteresis {}

impl Hysteresis {
    fn new(_target: f32, _margin: f32) -> Self {
        todo!()
    }

    fn update(&mut self, _measured: f32) -> Toggled {
        todo!()
    }
}

#[embassy_executor::task]
pub async fn run_water_pump(mut pump: Output<'static>, mut pwm: PwmOutput<'static>) {
    let mut config: Hysteresis = unimplemented!();
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let _humidity = humidity_receiver.next_message_pure().await;
        let on: bool = todo!();

        if on {
            pump.set_high();
        } else {
            pump.set_low();
        }
    }
}
