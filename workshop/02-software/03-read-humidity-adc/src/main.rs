#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    config::{self},
    gpio::{Level, Output, Pull},
};
use read_humidity_adc::{Irqs, read_adc};

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let led_pin = Output::new(p.PIN_22, Level::Low);

    spawner
        .spawn(read_adc(adc_component, humidity_adc_channel, led_pin))
        .unwrap();

    loop {
        yield_now().await;
    }
}
