use core::fmt::Write;

use defmt::{debug, trace};
use embassy_rp::gpio::Output;
use embassy_usb::class::cdc_acm::CdcAcmClass;
use heapless::String;
use num_traits::float::FloatCore;

use crate::{HUMIDITY_PUBSUB_CHANNEL, usb_setup::StaticUsbDriver};

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

#[embassy_executor::task]
pub async fn send_humidity_usb(mut usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>) {
    let mut serial_msg_buf = String::<64>::new();
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;
        let humidity_perc = (humidity * 100.0).floor();
        debug!("Received humidity");
        serial_msg_buf.clear();
        write!(&mut serial_msg_buf, "Humidity: {} %\r\n", humidity_perc).unwrap();
        debug!("Sending humidity over USB: {}", serial_msg_buf.as_str());
        usb_io_handle
            .write_packet(serial_msg_buf.as_bytes())
            .await
            .unwrap();
    }
}
