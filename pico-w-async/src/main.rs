#![no_std]
#![no_main]

use defmt::{debug, info};
use defmt_rtt as _;
use embassy_executor::{Executor, Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    gpio::{Level, Output, Pull},
    multicore::{Stack, spawn_core1},
};
use panic_probe as _;
use pico_w_async::{
    Irqs,
    inputs::measure_humidity,
    net::notify_http,
    outputs::{send_humidity_serial_usb, toggle_onboard_led},
    usb_setup::{UsbSetup, maintain_usb_connection},
    wifi::create_wifi_net_stack,
};
use static_cell::StaticCell;

static CORE1_ASYNC_EXECUTOR: StaticCell<Executor> = StaticCell::new();

static mut CORE1_VAR_STACK: Stack<4096> = Stack::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(Default::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let on_board_led = Output::new(p.PIN_27, Level::Low);

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(CORE1_VAR_STACK) },
        move || {
            debug!("Spawning executor on core 1 ");
            let on_board_executor = CORE1_ASYNC_EXECUTOR.init(Executor::new());
            on_board_executor.run(|spawner| {
                debug!("Spawning async tasks on core 1");
                spawner
                    .spawn(measure_humidity(adc_component, humidity_adc_channel))
                    .unwrap();
                spawner.spawn(toggle_onboard_led(on_board_led)).unwrap();
            });
        },
    );

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    let mut embassy_net_stack = create_wifi_net_stack(
        spawner, p.PIO0, p.PIN_23, p.PIN_25, p.PIN_24, p.PIN_29, p.DMA_CH0,
    )
    .await;

    notify_http(&mut embassy_net_stack, "Raspberry Pico W is online.").await;

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner
        .spawn(send_humidity_serial_usb(usb_io_handle))
        .unwrap();

    loop {
        yield_now().await;
    }
}
