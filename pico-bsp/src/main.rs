#![no_std]
#![no_main]

use core::fmt::Write;

use panic_halt as _;
use pico_bsp::{humidity::measure_humidity, usb::dequeue_send_usb};
use rp_pico::{
    entry,
    hal::{
        self,
        multicore::{Multicore, Stack},
    },
    pac::{self},
};
static mut MEASURE_STACK: Stack<4096> = Stack::new();

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let mut sio = hal::Sio::new(pac.SIO);

    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);

    // Setup of second task
    let cores = mc.cores();
    let measure_core = &mut cores[1];

    #[allow(static_mut_refs)]
    measure_core
        .spawn(unsafe { &mut MEASURE_STACK.mem }, measure_humidity)
        .unwrap();

    dequeue_send_usb::<64>(|message_buff, humidity| {
        let _ = write!(message_buff, "Humidity: {humidity} %\n\r");
    });
}
