use core::fmt::Write;

use defmt::info;
use embassy_executor::Spawner;
use embassy_net::{DhcpConfig, Runner, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::{
    peripherals::{RADIO_CLK, RNG, TIMG0, WIFI},
    rng::Rng,
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_wifi::{
    EspWifiController, init,
    wifi::{
        ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiStaDevice,
        WifiState,
    },
};
use heapless::String;

macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

const SSID: &str = "?";
const PASSWORD: &str = "?";

pub async fn setup_wifi_dhcp_client(
    spawner: Spawner,
    rng: RNG,
    radio_clk: RADIO_CLK,
    wifi: WIFI,
) -> embassy_net::Stack<'static> {
    esp_alloc::heap_allocator!(80 * 1024);

    let mut rng = Rng::new(rng);

    let esp_wifi_ctrl = &*{
        static STATIC_CELL: static_cell::StaticCell<EspWifiController<'static>> =
            static_cell::StaticCell::new();

        info!("Initialising WIFI controller.");
        STATIC_CELL.uninit().write(
            init(
                TimerGroup::new(unsafe { TIMG0::steal() }).timer0,
                rng,
                radio_clk,
            )
            .unwrap(),
        )
    };

    let (interfaces, mut controller) =
        esp_wifi::wifi::new_with_mode(esp_wifi_ctrl, wifi, WifiStaDevice).unwrap();

    info!("Disabling power saving.");
    controller
        .set_power_saving(esp_wifi::config::PowerSaveMode::None)
        .unwrap();

    info!("Creating DHCP configuration.");
    let dhcp_config = embassy_net::Config::dhcpv4(DhcpConfig::default());

    let seed = (rng.random() as u64) << 32 | rng.random() as u64;

    info!("Creating network stack.");
    // Init network stack
    let (net_stack, runner) = embassy_net::new(
        interfaces,
        dhcp_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        seed,
    );

    info!("Spawning WIFI helper tasks.");
    spawner.spawn(connection(controller)).unwrap();
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

    println!("Waiting to get IP address...");
    loop {
        if let Some(config) = net_stack.config_v4() {
            println!("Got IP: {}", config.address);
            break;
        }
        Timer::after(Duration::from_millis(500)).await;
    }
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    println!("start connection task");
    println!("Device capabilities: {:?}", controller.capabilities());
    loop {
        if esp_wifi::wifi::wifi_state() == WifiState::StaConnected {
            // wait until we're no longer connected
            controller.wait_for_event(WifiEvent::StaDisconnected).await;
            Timer::after(Duration::from_millis(5000)).await
        }
        if !matches!(controller.is_started(), Ok(true)) {
            let mut ssid = String::<32>::new();
            ssid.write_str(SSID).unwrap();
            let mut password = String::<64>::new();
            password.write_str(PASSWORD).unwrap();
            let client_config = Configuration::Client(ClientConfiguration {
                ssid,
                password,
                ..Default::default()
            });
            controller.set_configuration(&client_config).unwrap();
            println!("Starting wifi");
            controller.start_async().await.unwrap();
            println!("Wifi started!");
        }
        println!("About to connect...");

        match controller.connect_async().await {
            Ok(_) => println!("Wifi connected!"),
            Err(e) => {
                println!("Failed to connect to wifi: {e:?}");
                Timer::after(Duration::from_millis(5000)).await
            }
        }
    }
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static, WifiStaDevice>>) {
    runner.run().await
}
