#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output},
    timer::systimer::SystemTimer,
};
use esp32c6_async::{
    inputs::measure_humidity, net::notify, outputs::toggle_led, usb_setup::UsbJtagSetup,
    wifi::setup_wifi_dhcp_client,
};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    info!("Initializing peripherals of ESP32");
    let p = esp_hal::init(config);

    info!("Initializing timer");

    let systimer = SystemTimer::new(p.SYSTIMER);
    esp_hal_embassy::init(systimer.alarm0);

    let mut net_stack = setup_wifi_dhcp_client(spawner, p.RNG, p.RADIO_CLK, p.WIFI).await;

    notify(&mut net_stack, "Setup the wifi").await;

    info!("Set up of onboard LED");
    let on_board_led = Output::new(p.GPIO2, Level::Low);

    spawner.spawn(toggle_led(on_board_led)).unwrap();

    UsbJtagSetup::new(p.USB_DEVICE).start_usb_comm(spawner);

    info!("Set up of humidity sensor");
    measure_humidity(p.ADC1, p.GPIO0).await
}
