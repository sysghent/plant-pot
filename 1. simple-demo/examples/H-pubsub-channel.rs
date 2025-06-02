//! In this exercise you will combine measuring the humidity of the soil with
//! driving the water pump. On way to accomplish this is by using a channel.
//!
//! TODO: Show how the SIO FIFO queue can be used to send messages between
//! cores (seems unavailable in Embassy).

#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::{Executor, Spawner, main};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts,
    config::{self},
    gpio::{Level, Output, Pull},
    multicore::Stack,
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::{Duration, Ticker, Timer};
use panic_probe as _;
use static_cell::StaticCell;

static CORE1_ASYNC_EXECUTOR: StaticCell<Executor> = StaticCell::new();

static mut CORE1_VAR_STACK: Stack<4096> = Stack::new();

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let humidity_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    spawner
        .spawn(measure_humidity(adc_component, humidity_adc_channel))
        .unwrap();
    run_water_pump(on_board_pump).await
}

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

pub async fn run_water_pump(mut pump: Output<'static>) -> ! {
    let mut humidity_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let humidity = humidity_receiver.next_message_pure().await;

        if humidity < 0.1 {
            pump.set_high();
        } else {
            pump.set_low();
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
pub async fn measure_humidity(mut adc: Adc<'static, Async>, mut humidity_pin: Channel<'static>) {
    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;

        let level = adc.read(&mut humidity_pin).await.unwrap();

        let humidity = voltage_to_humidity(adc_reading_to_voltage(level));

        publisher.publish_immediate(humidity);
    }
}
