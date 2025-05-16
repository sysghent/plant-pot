use defmt::{debug, trace};
use embassy_rp::gpio::Output;

use crate::{HUMIDITY_PUBSUB_CHANNEL, usb::StaticUsbDevice};

#[embassy_executor::task]
pub async fn usb_task(mut usb: StaticUsbDevice) -> ! {
    debug!("Starting USB communication handling task");
    usb.run().await
}

#[embassy_executor::task]
pub async fn toggle_led(mut led: Output<'static>) {
    const HUMIDITY_THRESH_PERC: f32 = 0.1;

    let mut subscriber = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();
    loop {
        let humidity = subscriber.next_message_pure().await;

        if humidity < HUMIDITY_THRESH_PERC {
            trace!("Humidity is low, turning on LED");
            led.set_high();
        } else {
            trace!("Humidity is high, turning off LED");
            led.set_low();
        }
    }
}
