//! # Parallelism on a multicore microcontroller
//! 
//! The Raspberry Pi Pico 2 W has two cores. This allows to run multiple tasks in parallel. The Embassy framework provides a way to run tasks on different cores. This is done by assigning tasks to different executors. Each executor runs on a different core.
//! 
//! In this exercise you have to try to run two tasks in parallel, not just concurrently, but simultaneously on two different cores.
//! 
//! _**Remark**: Notice that both tasks on both cores are blocking (non-asynchronous). In other words, we don't actually need the Embassy framework to run these tasks. However, there are no threads available on the Pico and we need the 'spawn' made specifically for this micro-controller architecture: 'cortex-m'._
//! 

#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Executor, Spawner, main};
use embassy_rp::{
    adc::{Adc, Channel, Config},
    config::{self},
    gpio::{Level, Output, Pull},
    multicore::Stack,
};
use parallel_tasks::{
    Irqs,
    humidity_monitors::{send_humidity_serial_usb, toggle_onboard_led},
    measure_humidity::measure_humidity,
};
use static_cell::StaticCell;

static CORE1_ASYNC_EXECUTOR: StaticCell<Executor> = StaticCell::new();

static mut CORE1_VAR_STACK: Stack<4096> = Stack::new();


bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();


#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let on_board_led = Output::new(p.PIN_27, Level::Low);

    let _stack = unsafe { &mut *core::ptr::addr_of_mut!(CORE1_VAR_STACK) };

    let _second_core_task = || {
        let on_board_executor = CORE1_ASYNC_EXECUTOR.init(Executor::new());
        on_board_executor.run(|spawner| {
            spawner
                .spawn(measure_humidity(adc_component, humidity_adc_channel))
                .unwrap();
            spawner.spawn(toggle_onboard_led(on_board_led)).unwrap();
        });
    };

    todo!("Use spawn_core1 to run the second task on the second core 'core1' (in a blocking way).");

    send_humidity_serial_usb(p.USB, _spawner).await;
}



use embassy_rp::adc::{Adc, Async, Channel};
use embassy_time::{Duration, Ticker};

use crate::HUMIDITY_PUBSUB_CHANNEL;

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_16BIT: u32 = u32::pow(2, 16);
    (f32::from(reading_12bit) / STEPS_16BIT as f32) * REFERENCE_VOLTAGE
}

fn voltage_to_humidity(voltage: f32) -> f32 {
    const AIR_V: f32 = 0.178;
    const WATER_V: f32 = 0.0910;
    1.0 - (voltage - WATER_V) / (AIR_V - WATER_V)
}

#[embassy_executor::task]
pub async fn measure_humidity(mut adc: Adc<'static, Async>, mut humidity_pin: Channel<'static>) {
    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;

        let level = adc.read(&mut humidity_pin).await.unwrap();
        let voltage = adc_reading_to_voltage(level);
        let humidity = voltage_to_humidity(voltage);
        publisher.publish_immediate(humidity);
    }
}




use embedded_io::Write;

use embassy_executor::Spawner;
use embassy_rp::{gpio::Output, peripherals::USB};
use embassy_rp_io::usb::BasicUsbSetup;

use crate::{HUMIDITY_PUBSUB_CHANNEL, Irqs};

#[embassy_executor::task]
pub async fn toggle_onboard_led(mut led: Output<'static>) {
    const MIN_HUMIDITY: f32 = 0.1;

    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();
    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        if humidity < MIN_HUMIDITY {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}

pub async fn send_humidity_serial_usb(usb: USB, spawner: Spawner) -> ! {
    let usb = BasicUsbSetup::new(usb, Irqs);

    let mut humidity_sensor = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    usb.send(
        async |mut buf| {
            let humidity = humidity_sensor.next_message_pure().await;

            write!(buf, "Humidity: {humidity:.2} %\r\n").unwrap();
        },
        spawner,
    )
    .await
}
