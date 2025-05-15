// The macro for our start-up function

#![no_std]
#![no_main]

use rp_pico::{
    hal::{clocks::ClocksManager, usb::UsbBus},
    pac::Peripherals,
};

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// A shorter alias for the Peripheral Access Crate, which provides low-level
// register access

// A shorter alias for the Hardware Abstraction Layer, which provides
// higher-level drivers.
use rp_pico::hal;

// USB Device support
use usb_device::{class_prelude::*, prelude::*};

// USB Communications Class Device support
use usbd_serial::SerialPort;

pub fn reply_continuous(mut pac: Peripherals, clocks: ClocksManager, f: impl Fn(&mut [u8])) {
    // Set up the USB driver
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
    loop {
        if usb_dev.poll(&mut [&mut serial]) {
            reply(&mut serial, &f);
        }
    }
}

/// When `usb_dev.poll` use this to reply
pub fn reply(serial: &mut SerialPort<'_, UsbBus>, f: &impl Fn(&mut [u8])) {
    // Check for new data
    let mut buf = [0u8; 64];
    match serial.read(&mut buf) {
        Err(_e) => (),
        Ok(0) => (),
        Ok(count) => {
            // Convert to upper case
            f(&mut buf[..count]); // Send back to the host
            let mut wr_ptr = &buf[..count];
            while !wr_ptr.is_empty() {
                match serial.write(wr_ptr) {
                    Ok(len) => {
                        wr_ptr = &wr_ptr[len..];
                    }
                    // On error, just drop unwritten data.
                    // One possible error is Err(WouldBlock), meaning the USB
                    // write buffer is full.
                    Err(_) => (),
                }
            }
        }
    }
}
