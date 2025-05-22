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
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[embassy_executor::task]
async fn cyw43_task(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

const SSID: &str = "?";
const PASSWORD: &str = "?";

pub async fn create_wifi_net_stack(
    spawner: Spawner,
    pio: PIO0,
    pwr_pin_23: PIN_23,
    cs_pin_25: PIN_25,
    dio_pin_24: PIN_24,
    clk_pin_29: PIN_29,
    dma_ch_0: DMA_CH0,
) -> embassy_net::Stack<'static> {
    let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");
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

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (interfaces, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    spawner.spawn(cyw43_task(runner)).unwrap();

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    info!("Disabling power saving.");
    control
        .set_power_management(cyw43::PowerManagementMode::None)
        .await;

    info!("Creating DHCP configuration.");
    let dhcp_config = embassy_net::Config::dhcpv4(DhcpConfig::default());

    let seed = 0_u64;

    info!("Creating network stack.");
    // Init network stack
    let (net_stack, runner) = embassy_net::new(
        interfaces,
        dhcp_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
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

    info!("Waiting to get IP address...");
    loop {
        if let Some(config) = net_stack.config_v4() {
            info!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn connection(mut controller: Control<'static>) {
    loop {
        match controller
            .join(SSID, JoinOptions::new(PASSWORD.as_bytes()))
            .await
        {
            Ok(_) => break,
            Err(err) => {
                info!("join failed with status={}", err.status);
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, NetDriver<'static>>) {
    runner.run().await
}
