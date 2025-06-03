use cyw43::{Control, JoinOptions, NetDriver};
use cyw43_pio::{PioSpi, RM2_CLOCK_DIVIDER};
use defmt::info;
use embassy_executor::Spawner;
use embassy_net::{DhcpConfig, Runner, StackResources};
use embassy_rp::{
    clocks::RoscRng,
    gpio::{Level, Output},
    interrupt::typelevel::Binding,
    peripherals::{self, DMA_CH0, PIN_23, PIN_24, PIN_25, PIN_29, PIO0},
    pio::{Instance, InterruptHandler, Pio},
};
use embassy_time::{Duration, Timer};
use rand_core::RngCore;
use static_cell::StaticCell;
const SSID: &str = env!("SSID");
const PASSWORD: &str = env!("PASSWORD");

/// Some low-level components for setting up Wifi.
pub struct EasyWifi {
    pub pio: PIO0,
    pub pwr_pin_23: PIN_23,
    pub cs_pin_25: PIN_25,
    pub dio_pin_24: PIN_24,
    pub clk_pin_29: PIN_29,
    pub dma_ch_0: DMA_CH0,
}

impl EasyWifi {
    pub async fn setup_wifi_controller(
        self,
        spawner: Spawner,
        irqs: impl Binding<<peripherals::PIO0 as Instance>::Interrupt, InterruptHandler<PIO0>>,
    ) -> Control<'static> {
        static RADIO_STATE: StaticCell<cyw43::State> = StaticCell::new();
        static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
        let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
        let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

        let EasyWifi {
            pio,
            pwr_pin_23,
            cs_pin_25,
            dio_pin_24,
            clk_pin_29,
            dma_ch_0,
        } = self;
        let pwr = Output::new(pwr_pin_23, Level::Low);
        let cs = Output::new(cs_pin_25, Level::High);
        let mut pio = Pio::new(pio, irqs);

        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            RM2_CLOCK_DIVIDER,
            pio.irq0,
            cs,
            dio_pin_24,
            clk_pin_29,
            dma_ch_0,
        );

        let radio_state = RADIO_STATE.init(cyw43::State::new());
        let (_, mut control, wifi_runner) = cyw43::new(radio_state, pwr, spi, fw).await;
        spawner.spawn(run_cyw43(wifi_runner)).unwrap();

        control.init(clm).await;

        info!("Disabling power saving.");
        control
            .set_power_management(cyw43::PowerManagementMode::None)
            .await;

        control
    }

    pub async fn setup_wifi_stack(
        self,
        spawner: Spawner,
        irqs: impl Binding<<peripherals::PIO0 as Instance>::Interrupt, InterruptHandler<PIO0>>,
    ) -> WifiStack {
        static RADIO_STATE: StaticCell<cyw43::State> = StaticCell::new();
        static STACK_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
        let fw = include_bytes!("../cyw43-firmware/43439A0.bin");
        let clm = include_bytes!("../cyw43-firmware/43439A0_clm.bin");

        let EasyWifi {
            pio,
            pwr_pin_23,
            cs_pin_25,
            dio_pin_24,
            clk_pin_29,
            dma_ch_0,
        } = self;
        let pwr = Output::new(pwr_pin_23, Level::Low);
        let cs = Output::new(cs_pin_25, Level::High);
        let mut pio = Pio::new(pio, irqs);

        let spi = PioSpi::new(
            &mut pio.common,
            pio.sm0,
            RM2_CLOCK_DIVIDER,
            pio.irq0,
            cs,
            dio_pin_24,
            clk_pin_29,
            dma_ch_0,
        );

        let radio_state = RADIO_STATE.init(cyw43::State::new());
        let (device_driver, mut control, wifi_runner) = cyw43::new(radio_state, pwr, spi, fw).await;
        spawner.spawn(run_cyw43(wifi_runner)).unwrap();

        control.init(clm).await;

        info!("Disabling power saving.");
        control
            .set_power_management(cyw43::PowerManagementMode::None)
            .await;

        info!("Creating DHCP configuration.");
        let dhcp_config = embassy_net::Config::dhcpv4(DhcpConfig::default());

        let seed = RoscRng.next_u64();

        info!("Creating network stack.");
        let (net_stack, net_stack_runner) = embassy_net::new(
            device_driver,
            dhcp_config,
            STACK_RESOURCES.init(StackResources::<3>::new()),
            seed,
        );

        info!("Spawning WiFi helper tasks.");
        join_authenticate(&mut control).await;
        spawner.spawn(run_net_stack(net_stack_runner)).unwrap();

        wait_until_ip_acquired(net_stack).await;

        WifiStack {
            net_stack,
            wifi_controller: control,
        }
    }
}

pub struct WifiStack {
    pub net_stack: embassy_net::Stack<'static>,
    pub wifi_controller: Control<'static>,
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

#[embassy_executor::task]
async fn run_cyw43(
    runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}
