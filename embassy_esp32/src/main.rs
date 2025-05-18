#![no_std]
#![no_main]

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use esp_hal::{
    gpio::{Level, Output},
    timer::timg::TimerGroup,
};
use esp32_async_plant::{inputs::measure_humidity, outputs::toggle_led, usb_setup::UsbJtagSetup};

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals of ESP32");
    let p = esp_hal::init(Default::default());

    let timer_unit = TimerGroup::new(p.TIMG0);

    // Necessary to be able to use delays and timers.
    esp_hal_embassy::init(timer_unit.timer0);

    let on_board_led = Output::new(p.GPIO2, Level::Low);

    spawner.spawn(toggle_led(on_board_led)).unwrap();

    UsbJtagSetup::new(p.USB_DEVICE).start_usb_comm(spawner);

    measure_humidity(p.ADC1, p.GPIO0).await
}
