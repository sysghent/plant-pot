#![no_std]
#![no_main]

use core::fmt::Write;
use core::str;

use embassy_executor::{Spawner, main};
use embassy_rp::bind_interrupts;
use embassy_rp::config::Config;
use embassy_rp::peripherals::PIO0;
use embassy_rp::pio::InterruptHandler;

use heapless::String;
use plant_pot::http_notify::notify_http;
use plant_pot::wifi::{EasyWifi, WifiStack};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(Config::default());

    let wifi_conf = EasyWifi {
        pio: p.PIO0,
        pwr_pin_23: p.PIN_23,
        cs_pin_25: p.PIN_25,
        dio_pin_24: p.PIN_24,
        clk_pin_29: p.PIN_29,
        dma_ch_0: p.DMA_CH0,
    };
    let WifiStack { mut net_stack, .. } = wifi_conf.setup_wifi_stack(spawner, Irqs).await;

    let mut message = String::<64>::new();
    let mut counter = 0;
    loop {
        write!(&mut message, "Iteration: {counter}").unwrap();
        notify_http(&mut net_stack, &message).await;
        counter += 1;
    }
}
