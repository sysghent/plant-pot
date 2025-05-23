#![no_std]
#![no_main]

use calibrate_humidity_sensor::{
    Irqs,
    inputs::measure_humidity,
    outputs::send_humidity_serial_usb,
    usb_setup::{UsbSetup, maintain_usb_connection},
};
use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    config::{self},
    gpio::Pull,
};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    spawner
        .spawn(measure_humidity(adc_component, humidity_adc_channel))
        .unwrap();

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner
        .spawn(send_humidity_serial_usb(usb_io_handle))
        .unwrap();

    loop {
        yield_now().await;
    }
}
