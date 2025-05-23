#![no_std]

use embassy_rp::{
    adc::{Adc, Async, Channel},
    bind_interrupts,
    gpio::Output,
};
use embassy_time::{Duration, Ticker};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs {
        ADC_IRQ_FIFO => embassy_rp::adc::InterruptHandler;
    }
);

#[embassy_executor::task]
pub async fn read_adc(
    mut _adc: Adc<'static, Async>,
    mut _humidity_pin: Channel<'static>,
    _led_pin: Output<'static>,
) {
    let mut _ticker: Ticker = Ticker::every(Duration::from_millis(500));

    let adc_threshold: u16 =
        todo!("Find an average ADC value, right after being measured by the ADC unit.");

    loop {
        _ticker.next().await;

        let level: u16 = todo!(
            "Wait until first ADC value has been produced and stored in ADC queue. Then turn on \
             the LED"
        );
        if level < adc_threshold {
            _led_pin.set_high();
        } else {
            _led_pin.set_low();
        }
    }
}
