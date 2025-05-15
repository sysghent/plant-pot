#![no_std]
#![no_main]

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use rp_pico::hal;

use hal::entry;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access
use hal::pac;

use embedded_hal::digital::OutputPin;
use rp_pico::hal::adc::AdcPin;

const SERIAL_MESSAGE_LEN: usize = 80;

fn adc_reading_to_voltage(reading_12bit: u16) -> f32 {
    const REFERENCE_VOLTAGE: f32 = 3.3;
    const STEPS_12BIT: u16 = u16::pow(2, 12);
    (f32::from(reading_12bit) / f32::from(STEPS_12BIT)) * REFERENCE_VOLTAGE
}

const HUMIDITY_THRESH_PERC: f32 = 0.1;

fn voltage_to_humidity(voltage: f32) -> f32 {
    const AIR_V: f32 = 2.77;
    const WATER_V: f32 = 1.4;
    -(voltage - WATER_V) / (AIR_V - WATER_V)
}

use rp_pico::hal::clocks::ClocksManager;
use rp_pico::hal::fugit::MicrosDurationU32;
use rp_pico::hal::multicore::Multicore;
use rp_pico::hal::multicore::Stack;
use rp_pico::hal::sio::SioFifo;
use rp_pico::pac::Peripherals;
// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

// Used to demonstrate writing formatted strings
use core::fmt::Write;
// use core::time::Duration;
fn measure_humidity() {
    let mut pac = unsafe { pac::Peripherals::steal() };

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut adc = hal::Adc::new(pac.ADC, &mut pac.RESETS);

    let mut humidity_pin = AdcPin::new(pins.gpio26).unwrap();
    let mut adc_queue = adc.build_fifo().set_channel(&mut humidity_pin).start();
    let mut on_board_led_pin = pins.gpio25.into_push_pull_output();

    let mut sio_queue = sio.fifo;
    loop {
        if adc_queue.len() > 0 {
            let digital_value: u16 = adc_queue.read();
            let sensor_volt = adc_reading_to_voltage(digital_value);
            let humidity = voltage_to_humidity(sensor_volt);
            if sio_queue.is_write_ready() {
                #[allow(clippy::cast_sign_loss)]
                #[allow(clippy::cast_possible_truncation)]
                sio_queue.write_blocking((humidity * 100.0) as u32);
            }
            if humidity < HUMIDITY_THRESH_PERC {
                on_board_led_pin.set_high().unwrap();
            } else {
                on_board_led_pin.set_low().unwrap();
            }
        }
    }
}

static mut MEASURE_STACK: Stack<4096> = Stack::new();

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut sio = hal::Sio::new(pac.SIO);
    let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);

    // Setup of second task
    let cores = mc.cores();
    let measure_core = &mut cores[1];
    #[allow(static_mut_refs)]
    measure_core
        .spawn(unsafe { &mut MEASURE_STACK.mem }, measure_humidity)
        .unwrap();

    let mut queue = sio.fifo;

    output_regularly(
        clocks,
        &mut queue,
        |humidity, message| {
            message.clear();
            write!(message, "Humidity: {} %\n\r", humidity).unwrap()
        },
        MicrosDurationU32::millis(500),
    )
}

pub fn output_regularly(
    clocks: ClocksManager,
    queue: &mut SioFifo,
    f: impl Fn(u32, &mut heapless::String<SERIAL_MESSAGE_LEN>),
    interval: MicrosDurationU32,
) -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);
    // Setup of serial connection
    let usb_bus = UsbBusAllocator::new(hal::usb::UsbBus::new(
        pac.USBCTRL_REGS,
        pac.USBCTRL_DPRAM,
        clocks.usb_clock,
        true,
        &mut pac.RESETS,
    ));

    // Set up the USB Communications Class Device driver
    let mut serial = SerialPort::new(&usb_bus);

    // Create a USB device with a fake VID and PID
    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .strings(&[StringDescriptors::default()
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")])
        .unwrap()
        .device_class(2) // from: https://www.usb.org/defined-class-codes
        .build();

    let mut message: heapless::String<SERIAL_MESSAGE_LEN> = heapless::String::new();

    let mut last_emission = None;

    serial.write("Before loop".as_bytes()).unwrap();
    loop {
        let _ = usb_dev.poll(&mut [&mut serial]);

        serial.write("After loop".as_bytes()).unwrap();
        if last_emission.is_none()
            || last_emission.is_some_and(|last| {
                timer.get_counter().checked_duration_since(last).unwrap() > interval
            })
        {
            if let Some(humidity) = queue.read() {
                message.clear();

                f(humidity, &mut message);
                // let _ = write!(message, "Humidity: {humidity} %\n\r");
                let _ = serial.write(message.as_bytes());
                last_emission = Some(timer.get_counter());
            }
        }
    }
}
