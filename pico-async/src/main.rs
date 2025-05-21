#![no_std]
#![no_main]

use defmt::{debug, info};
use defmt_rtt as _;
use embassy_executor::Executor;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    gpio::{Level, Output, Pull},
    multicore::{Stack, spawn_core1},
};
use panic_probe as _;
use pico_async::{
    Irqs,
    inputs::measure_humidity,
    outputs::{send_humidity_usb, toggle_led},
    usb_setup::{UsbSetup, usb_task},
};
use static_cell::StaticCell;

static USB_EXECUTOR: StaticCell<Executor> = StaticCell::new();
static ON_BOARD_EXECUTOR: StaticCell<Executor> = StaticCell::new();

static mut SENSOR_VAR_STACK: Stack<4096> = Stack::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(Default::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let on_board_led = Output::new(p.PIN_25, Level::Low);

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(SENSOR_VAR_STACK) },
        move || {
            debug!("Spawning executor on core 1 ");
            let on_board_executor = ON_BOARD_EXECUTOR.init(Executor::new());
            on_board_executor.run(|spawner| {
                debug!("Spawning async tasks on core 1");
                spawner
                    .spawn(measure_humidity(adc_component, humidity_adc_channel))
                    .unwrap();
                spawner.spawn(toggle_led(on_board_led)).unwrap();
            });
        },
    );

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    debug!("Spawning executor on core 0");
    let usb_executor = USB_EXECUTOR.init(Executor::new());
    usb_executor.run(|spawner| {
        debug!("Spawning async tasks on core 0");
        spawner.spawn(usb_task(usb_runtime)).unwrap();
        spawner.spawn(send_humidity_usb(usb_io_handle)).unwrap();
    });
}
