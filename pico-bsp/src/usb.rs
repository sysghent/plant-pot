use heapless::String;
use panic_halt as _;
use rp_pico::{
    hal::{self, Sio, fugit::MicrosDurationU32},
    pac::{self},
};
use usb_device::{class_prelude::*, prelude::*};
use usbd_serial::SerialPort;

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

pub fn dequeue_send_usb<const N: usize>(write_message: impl Fn(&mut String<N>, u32)) -> ! {
    let mut pac = unsafe { pac::Peripherals::steal() };
    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let timer = hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

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

    let mut message: heapless::String<N> = heapless::String::new();

    let mut last_emission = None;

    let mut sio = Sio::new(pac.SIO);

    loop {
        let _ = usb_dev.poll(&mut [&mut serial]);

        if last_emission.is_none()
            || last_emission.is_some_and(|last| {
                timer.get_counter().checked_duration_since(last).unwrap()
                    > MicrosDurationU32::millis(500u32)
            })
        {
            if let Some(humidity) = sio.fifo.read() {
                message.clear();
                write_message(&mut message, humidity);
                let _ = serial.write(message.as_bytes());
                last_emission = Some(timer.get_counter());
            }
        }
    }
}
