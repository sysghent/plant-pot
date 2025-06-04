#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::{
    adc::{Adc, Async, Channel, Config},
    bind_interrupts,
    config::{self},
    gpio::{Level, Output, Pull},
    peripherals::{PIO0, USB},
    pio::InterruptHandler,
};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, pubsub::PubSubChannel};
use embassy_time::{Duration, Ticker, Timer};
use panic_probe as _;
use plant_pot::voltage_adc::{adc_reading_to_voltage, voltage_to_moisture};

static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
        USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
        PIO0_IRQ_0 => InterruptHandler<PIO0>;
    }
);

#[main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());

    let adc_component = Adc::new(p.ADC, Irqs, Config::default());

    let moisture_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);

    let on_board_pump = Output::new(p.PIN_28, Level::Low);

    spawner
        .spawn(measure_moisture(adc_component, moisture_adc_channel))
        .unwrap();
    run_water_pump(on_board_pump).await
}

pub async fn run_water_pump(mut pump: Output<'static>) -> ! {
    let mut moisture_receiver = HUMIDITY_PUBSUB_CHANNEL.subscriber().unwrap();

    loop {
        let moisture = moisture_receiver.next_message_pure().await;

        if moisture < 0.1 {
            pump.set_high();
        } else {
            pump.set_low();
        }

        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
pub async fn measure_moisture(mut adc: Adc<'static, Async>, mut moisture_pin: Channel<'static>) {
    let publisher = HUMIDITY_PUBSUB_CHANNEL.publisher().unwrap();
    let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;
        let level = adc.read(&mut moisture_pin).await.unwrap();
        let moisture = voltage_to_moisture(adc_reading_to_voltage(level));
        publisher.publish_immediate(moisture);
    }
}
