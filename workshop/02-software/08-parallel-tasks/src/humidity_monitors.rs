use embedded_io::Write;

use embassy_executor::Spawner;
use embassy_rp::{gpio::Output, peripherals::USB};
use embassy_rp_io::usb::BasicUsbSetup;

use crate::{HUMIDITY_PUBSUB_CHANNEL, Irqs};

#[embassy_executor::task]
pub async fn toggle_onboard_led(mut led: Output<'static>) {
    const MIN_HUMIDITY: f32 = 0.1;

    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();
    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        if humidity < MIN_HUMIDITY {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}

pub async fn send_humidity_serial_usb(usb: USB, spawner: Spawner) -> ! {
    let usb = BasicUsbSetup::new(usb, Irqs);

    let mut humidity_sensor = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    usb.send(
        async |mut buf| {
            let humidity = humidity_sensor.next_message_pure().await;

            write!(buf, "Humidity: {humidity:.2} %\r\n").unwrap();
        },
        spawner,
    )
    .await
}
