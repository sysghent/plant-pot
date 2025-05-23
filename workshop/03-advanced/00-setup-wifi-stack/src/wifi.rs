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
    let (interfaces, mut control, runner) = cyw43::new(radio_state, pwr, spi, fw).await;
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

    // TODO: randomise this
    let seed = todo!("Create a random seed within the no_std context");

    info!("Creating network stack.");
    // Init network stack
    let (net_stack, runner) = embassy_net::new(
        interfaces,
        dhcp_config,
        STACK_RESOURCES.init(StackResources::<3>::new()),
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
