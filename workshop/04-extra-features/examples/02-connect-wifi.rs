//! # Setup the WiFi stack of Embassy
//! 
//! Creating a WiFi network stack is a little bit too complicated, so you just have to write a small function to randomize the seed used for authentication WiFi. (It is not required to be random, but probably a good idea.)
//! 
//! The core of this code has been taken from the [example code](https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/blinky_wifi_pico_plus_2.rs) of the Embassy project.


#![no_std]
#![no_main]

use cortex_m_rt as _;
use defmt::info;
use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::config::{self};
use setup_wifi_stack::wifi::create_wifi_net_stack;

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(config::Config::default());

    let mut _embassy_net_stack = create_wifi_net_stack(
        spawner, p.PIO0, p.PIN_23, p.PIN_25, p.PIN_24, p.PIN_29, p.DMA_CH0,
    )
    .await;

    loop {
        yield_now().await;
    }
}

#![no_std]

pub mod wifi;

use defmt_rtt as _;
use embassy_rp::{bind_interrupts, peripherals::PIO0, pio::InterruptHandler};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);


use cyw43::{Control, JoinOptions, NetDriver};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::info;
use embassy_executor::Spawner;
use embassy_net::{DhcpConfig, Runner, StackResources};
use embassy_rp::{
    gpio::{Level, Output},
    peripherals::{DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0},
    pio::Pio,
};
use embassy_time::{Duration, Timer};
use static_cell::StaticCell;

use crate::Irqs;

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

pub async fn create_wifi_net_stack(
    spawner: Spawner,
    pio: PIO0,
    pwr_pin_23: PIN_23,
    cs_pin_25: PIN_25,
    dio_pin_24: PIN_24,
    clk_pin_29: PIN_29,
    dma_ch_0: DMA_CH0,
) -> embassy_net::Stack<'static> {
    static RADIO_STATE: StaticCell<cyw43::State> = StaticCell::new();
    static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    let fw = include_bytes!("../../../../wifi-firmware/43439A0.bin");
    let clm = include_bytes!("../../../../wifi-firmware/43439A0_clm.bin");
    let pwr = Output::new(pwr_pin_23, Level::Low);
    let cs = Output::new(cs_pin_25, Level::High);
    let mut pio = Pio::new(pio, Irqs);

    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        dio_pin_24,
        clk_pin_29,
        dma_ch_0,
    );

    let radio_state = RADIO_STATE.init(cyw43::State::new());
    let (_interfaces, mut control, runner) = cyw43::new(radio_state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;

    info!("Disabling power saving.");
    control
        .set_power_management(cyw43::PowerManagementMode::None)
        .await;

    info!("Creating DHCP configuration.");
    let _dhcp_config = embassy_net::Config::dhcpv4(DhcpConfig::default());

    // TODO: randomise this
    let _seed = todo!(
        "Create a random seed within the no_std context using a hardware random number generator \
         of the Pico."
    );

    info!("Creating network stack.");
    // Init network stack
    let (net_stack, runner) = embassy_net::new(
        _interfaces,
        _dhcp_config,
        STACK_RESOURCES.init(StackResources::<3>::new()),
        _seed,
    );

    info!("Spawning WIFI helper tasks.");
    spawner.spawn(connection(control)).unwrap();
    spawner.spawn(net_task(runner)).unwrap();

    wait_until_ip_acquired(net_stack).await;

    net_stack
}

async fn wait_until_ip_acquired(net_stack: embassy_net::Stack<'_>) {
    loop {
        if net_stack.is_link_up() {
            break;
        }
        info!("Waiting until link is up...");
        Timer::after(Duration::from_millis(500)).await;
    }

    loop {
        if let Some(config) = net_stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
        info!("Waiting to get IP address...");
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn connection(mut wifi_controller: Control<'static>) {
    loop {
        match wifi_controller
            .join(SSID, JoinOptions::new(PASSWORD.as_bytes()))
            .await
        {
            Ok(()) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, NetDriver<'static>>) {
    runner.run().await;
}
