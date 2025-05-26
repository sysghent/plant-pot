#![no_std]
#![no_main]

use calibrate_humidity_sensor::{Irqs, measure_humidity, send_humidity_serial_usb};
use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
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

    send_humidity_serial_usb(p.USB, spawner).await
}
