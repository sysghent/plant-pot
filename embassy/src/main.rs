//! This example shows how async gpio can be used with a RP2040.
//!
//! The LED on the RP Pico W board is connected differently. See wifi_blinky.rs.

#![no_std]
#![no_main]

// use static_cell::StaticCell;
use async_plant::{
    Irqs, idle,
    monitor_output::{toggle_led, usb_task},
    sensing::{measure_humidity, send_humidity},
    usb::UsbSetup,
};
use defmt::{debug, info};
use defmt_rtt as _;
use embassy_executor::Executor;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    gpio::{Level, Output, Pull},
    multicore::{Stack, spawn_core1},
};
use panic_probe as _;
use static_cell::StaticCell;

static EXECUTOR0: StaticCell<Executor> = StaticCell::new();
static EXECUTOR1: StaticCell<Executor> = StaticCell::new();

static mut SENSOR_VAR_STACK: Stack<4096> = Stack::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    info!("Initializing peripherals");
    let p = embassy_rp::init(Default::default());

    let adc = Adc::new(p.ADC, Irqs, Config::default());

    let p26 = Channel::new_pin(p.PIN_26, Pull::None);

    let led = Output::new(p.PIN_25, Level::Low);

    spawn_core1(
        p.CORE1,
        unsafe { &mut *core::ptr::addr_of_mut!(SENSOR_VAR_STACK) },
        move || {
            debug!("Spawning executor on core 1 ");
            let executor1 = EXECUTOR1.init(Executor::new());
            executor1.run(|spawner| {
                debug!("Spawning async tasks on core 1");
                spawner.spawn(measure_humidity(adc, p26)).unwrap();
                spawner.spawn(toggle_led(led)).unwrap();
                spawner.spawn(idle()).unwrap();
            });
        },
    );

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    debug!("Spawning executor on core 0");
    let executor0 = EXECUTOR0.init(Executor::new());
    executor0.run(|spawner| {
        debug!("Spawning async tasks on core 0");
        spawner.spawn(usb_task(usb_runtime)).unwrap();
        spawner.spawn(send_humidity(usb_io_handle)).unwrap();
        spawner.spawn(idle()).unwrap();
    });
}
