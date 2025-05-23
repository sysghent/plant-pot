use core::fmt::Write;

use embassy_usb::class::cdc_acm::CdcAcmClass;
use heapless::String;

use crate::usb_setup::StaticUsbDriver;

#[embassy_executor::task]
pub async fn spam_serial_monitor(mut usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>) {
    let mut serial_msg_buf = String::<64>::new();
    let mut counter = 0;
    loop {
        serial_msg_buf.clear();
        write!(&mut serial_msg_buf, "Counter: {counter} %\r\n").unwrap();
        usb_io_handle
            .write_packet(serial_msg_buf.as_bytes())
            .await
            .unwrap();
        counter += 1;
    }
}
