use defmt::trace;
use esp_hal::gpio::Output;

use crate::HUMIDITY_PUBSUB_CHANNEL;

#[embassy_executor::task]
pub async fn toggle_led(mut led: Output<'static>) {
    const MIN_HUMIDITY: f32 = 0.1;

    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();
    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        if humidity < MIN_HUMIDITY {
            trace!("Humidity is low, turning on LED");
            led.set_high();
        } else {
            trace!("Humidity is high, turning off LED");
            led.set_low();
        }
    }
}
