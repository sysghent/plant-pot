use embassy_rp::gpio::Output;

use crate::PUBSUB_CHANNEL;

const _TARGET_HUMIDITY: f32 = 0.3;

struct Hysteresis {}

impl Hysteresis {
    fn new(_target: f32, _margin: f32) -> Self {
        todo!()
    }

    fn update(&mut self, _measured: f32) -> bool {
        todo!()
    }
}

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>) {
    let mut _config: Hysteresis = todo!("Implement hysteresis.");
    let mut humidity_receiver = PUBSUB_CHANNEL.subscriber().unwrap();

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
