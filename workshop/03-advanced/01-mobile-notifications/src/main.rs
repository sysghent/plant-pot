#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    config::{self},
    gpio::Pull,
};
use mobile_notifications::{
    Irqs, http_notify::notify_http, inputs::measure_humidity, wifi::create_wifi_net_stack,
};

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    spawner
        .spawn(measure_humidity(adc_component, humidity_adc_channel))
        .unwrap();

    let mut embassy_net_stack = create_wifi_net_stack(
        spawner, p.PIO0, p.PIN_23, p.PIN_25, p.PIN_24, p.PIN_29, p.DMA_CH0,
    )
    .await;

    notify_http(&mut embassy_net_stack, "Raspberry Pico W is online.").await;

    loop {
        yield_now().await;
    }
}
