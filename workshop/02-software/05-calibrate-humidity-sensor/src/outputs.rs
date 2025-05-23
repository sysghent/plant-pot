use core::fmt::Write;

use embassy_usb::class::cdc_acm::CdcAcmClass;
use heapless::String;
use num_traits::float::FloatCore;

use crate::{HUMIDITY_PUBSUB_CHANNEL, usb_setup::StaticUsbDriver};

#[embassy_executor::task]
pub async fn send_humidity_serial_usb(mut usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>) {
    let mut serial_msg_buf = String::<64>::new();
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;
        let humidity_perc = (humidity * 100.0).floor();
        serial_msg_buf.clear();
        write!(&mut serial_msg_buf, "Humidity: {humidity_perc} %\r\n").unwrap();
        usb_io_handle
            .write_packet(serial_msg_buf.as_bytes())
            .await
            .unwrap();
    }
}
