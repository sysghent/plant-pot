use embassy_rp::{gpio::Output, pwm::PwmOutput};

const _TARGET_HUMIDITY: f32 = 0.3;

#[embassy_executor::task]
pub async fn run_water_pump(mut _pump: Output<'static>, mut _pwm: PwmOutput<'static>) {
    todo!("Write a loop to read the humidity and turn the pump on or off.");
}
