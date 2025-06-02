#![no_std]
#![no_main]

use core::str;

use cyw43::{Control, JoinOptions, NetDriver};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};
use defmt::*;
use embassy_executor::{Spawner, main};
use embassy_net::{DhcpConfig, Runner, StackResources};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIO0, USB};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::usb::Driver;
use embassy_time::{Duration, Timer};

use plant_pot::http_notify::{self, notify_http};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]

async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Hello World!");

    let p = embassy_rp::init(Default::default());

    spawner
        .spawn(logger_task(Driver::new(p.USB, Irqs)))
        .unwrap();

    let fw = include_bytes!("../wifi-cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../wifi-cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(
        &mut pio.common,
        pio.sm0,
        DEFAULT_CLOCK_DIVIDER,
        pio.irq0,
        cs,
        p.PIN_24,
        p.PIN_29,
        p.DMA_CH0,
    );

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (device_driver, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    info!("Creating DHCP configuration.");
    let dhcp_config = embassy_net::Config::dhcpv4(DhcpConfig::default());

    // TODO: randomise this
    let seed = 0_u64;

    static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
    info!("Creating network stack.");
    // Init network stack
    let (mut net_stack, net_stack_runner) = embassy_net::new(
        device_driver,
        dhcp_config,
        STACK_RESOURCES.init(StackResources::<3>::new()),
        seed,
    );

    spawner.spawn(run_net_stack(net_stack_runner)).unwrap();

    Timer::after_millis(100).await;

    join_authenticate(&mut control).await;

    Timer::after_millis(100).await;

    wait_until_ip_acquired(net_stack).await;

    loop {
        notify_http(&mut net_stack, "Hey").await;
    }
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

async fn join_authenticate(wifi_controller: &mut Control<'static>) {
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
async fn run_net_stack(mut runner: Runner<'static, NetDriver<'static>>) {
    runner.run().await;
}
