//! # Mobile notifications
//!
//! This advanced exercise shows how to send notifications to a mobile device
//! from your embedded project. Begin by implementing code to trigger a
//! notification when a specific event occurs.
//!

#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_net::{
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use embassy_rp::{
    bind_interrupts,
    config::{self},
    peripherals::PIO0,
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::{Duration, Timer};
use extra_features::wifi::{BasicWiFi, NetStackControl};
use heapless::String;
use panic_probe as _;
use reqwless::client::HttpClient;

bind_interrupts!(
    pub struct Irqs {
          PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let wifi_peripherals = BasicWiFi {
        pio: p.PIO0,
        pwr_pin_23: p.PIN_23,
        cs_pin_25: p.PIN_25,
        dio_pin_24: p.PIN_24,
        clk_pin_29: p.PIN_29,
        dma_ch_0: p.DMA_CH0,
    };
    let NetStackControl {
        net_stack: mut embassy_net_stack,
        wifi_controller: mut control,
    } = wifi_peripherals.start(spawner, Irqs).await;

    notify_http(&mut embassy_net_stack, "Raspberry Pico W is online.").await;

    loop {
        Timer::after(Duration::from_secs(5)).await;

        control.gpio_set(0, true).await;
        notify_http(&mut embassy_net_stack, "Raspberry Pico W is still online.").await;

        control.gpio_set(0, false).await;
    }
}

pub async fn notify_http(net_stack: &mut embassy_net::Stack<'_>, message: &str) {
    let state = TcpClientState::<1, 4096, 4096>::new();
    let tcp_client = TcpClient::new(*net_stack, &state);
    let dns_socket = DnsSocket::new(*net_stack);
    let mut _client = HttpClient::new(&tcp_client, &dns_socket);

    let mut body = String::<64>::new();
    body.write_str(message).unwrap();
    let mut _buffer = [0u8; 64];

    todo!("Create an HTTP request to ntfy.sh with the message in the body.");
}
