#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Executor, Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    adc::{Adc, Channel, Config},
    config::{self},
    gpio::{Level, Output, Pull},
    multicore::{Stack, spawn_core1},
};
use parallel_tasks::{
    Irqs,
    humidity_monitors::{send_humidity_serial_usb, toggle_onboard_led},
    measure_humidity::measure_humidity,
    usb_setup::{UsbSetup, maintain_usb_connection},
};
use static_cell::StaticCell;

static CORE1_ASYNC_EXECUTOR: StaticCell<Executor> = StaticCell::new();

static mut CORE1_VAR_STACK: Stack<4096> = Stack::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let on_board_led = Output::new(p.PIN_27, Level::Low);

    let stack = unsafe { &mut *core::ptr::addr_of_mut!(CORE1_VAR_STACK) };

    let second_core_task = || {
        let on_board_executor = CORE1_ASYNC_EXECUTOR.init(Executor::new());
        on_board_executor.run(|spawner| {
            spawner
                .spawn(measure_humidity(adc_component, humidity_adc_channel))
                .unwrap();
            spawner.spawn(toggle_onboard_led(on_board_led)).unwrap();
        });
    };

    todo!("Use spawn_core1 to run the second task on the second core 'core1' (in a blocking way).");

    let UsbSetup {
        usb_runtime,
        usb_io_handle,
    } = UsbSetup::new(p.USB);

    spawner.spawn(maintain_usb_connection(usb_runtime)).unwrap();
    spawner
        .spawn(send_humidity_serial_usb(usb_io_handle))
        .unwrap();

    loop {
        yield_now().await;
    }
}
