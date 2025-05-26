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
use embassy_rp_io::wifi::BasicWiFi;
use mobile_notifications::{Irqs, http_notify::notify_http, measure_humidity::keep_measuring};

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    spawner
        .spawn(keep_measuring(adc_component, humidity_adc_channel))
        .unwrap();

    let wifi_peripherals = BasicWiFi {
        pio: p.PIO0,
        pwr_pin_23: p.PIN_23,
        cs_pin_25: p.PIN_25,
        dio_pin_24: p.PIN_24,
        clk_pin_29: p.PIN_29,
        dma_ch_0: p.DMA_CH0,
    };
    let mut embassy_net_stack = wifi_peripherals.start(spawner, Irqs).await;

    notify_http(&mut embassy_net_stack, "Raspberry Pico W is online.").await;

    loop {
        yield_now().await;
    }
}
