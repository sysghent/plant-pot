use embassy_executor::Spawner;
const MAX_BUFFER_SIZE: usize = 512;

use esp_hal::{
    Async,
    peripherals::USB_DEVICE,
    usb_serial_jtag::{UsbSerialJtag, UsbSerialJtagRx, UsbSerialJtagTx},
};
use num_traits::float::FloatCore;

use crate::HUMIDITY_PUBSUB_CHANNEL;

pub struct UsbJtagSetup {
    pub rx: UsbSerialJtagRx<'static, Async>,
    pub tx: UsbSerialJtagTx<'static, Async>,
}

impl UsbJtagSetup {
    pub fn new(usb_device: USB_DEVICE) -> Self {
        let (rx, tx) = UsbSerialJtag::new(usb_device).into_async().split();

        Self { rx, tx }
    }

    pub fn spawn(self, spawner: Spawner) {
        let UsbJtagSetup { rx, tx } = self;
        spawner.spawn(reader(rx)).unwrap();
        spawner.spawn(writer(tx)).unwrap();
    }
}

#[embassy_executor::task]
async fn writer(mut tx: UsbSerialJtagTx<'static, Async>) {
    use core::fmt::Write;
    embedded_io_async::Write::write_all(
        &mut tx,
        b"Hello async USB Serial JTAG. Type something.\r\n",
    )
    .await
    .unwrap();

    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        let humidity_perc = (humidity * 100.0).floor();
        write!(&mut tx, "Humidity: {} %\r\n", humidity_perc).unwrap();
        embedded_io_async::Write::flush(&mut tx).await.unwrap();
    }
}

#[embassy_executor::task]
async fn reader(mut rx: UsbSerialJtagRx<'static, Async>) {
    let mut rbuf = [0u8; MAX_BUFFER_SIZE];
    loop {
        let r = embedded_io_async::Read::read(&mut rx, &mut rbuf).await;
        match r {
            Ok(len) => {
                let mut string_buffer: heapless::Vec<_, MAX_BUFFER_SIZE> = heapless::Vec::new();
                string_buffer.extend_from_slice(&rbuf[..len]).unwrap();

                // TODO: do something with `string_buffer`
            }
            #[allow(unreachable_patterns)]
            Err(e) => esp_println::println!("RX Error: {:?}", e),
        }
    }
}
