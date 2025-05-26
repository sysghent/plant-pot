use core::fmt::Write;

use embassy_usb::class::cdc_acm::CdcAcmClass;
use heapless::String;

use crate::usb_setup::StaticUsbDriver;

#[embassy_executor::task]
pub async fn spam_serial_monitor(mut _usb_io_handle: CdcAcmClass<'static, StaticUsbDriver>) {
    let mut serial_msg_buf = String::<64>::new();
    let counter = 0;
    loop {
        serial_msg_buf.clear();
        write!(&mut serial_msg_buf, "Counter: {counter} %\r\n").unwrap();
        todo!(
            "Implement sending the counter over the serial USB connection and try reading it in a \
             serial monitor on your laptop."
        );
        counter += 1;
    }
}
