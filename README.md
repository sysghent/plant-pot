# Workshop: make a smart plant pot

Notes for a workshop organised by Hugo & Willem in Ghent on 4th of June 2025. Register on [Mobilizon](https://mobilizon.be/events/3babf471-434d-431c-972c-b0bbae57b64c).

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the moisture of the earth in the pot is too dry.

## Overview

### Prerequisites

Please bring:

- **Laptop**
- **Micro-USB cable**
- **Water container** (sponge, or a dry vs. wet plant pot)

### Provided

You can borrow from us (or bring your own):

- [Raspberry Pico 2 W](https://datasheets.rapberrypi.com/picow/pico-2-w-datasheet.pdf): ~ 10 €
- Analogue capacitive moisture sensor: ~ 5 €
- JST SH 1mm Pitch 3 Pin to Male (not yet delivered): 2.50 €
- 3V submersible water pump ~ 5 €
- Breadboard: ~ 5 €
- Jumper wires ~ 1 €
- LED ~ 0.5 €
- MOSFET transistor ~ 0.5 €
- resistor ~ 0.5 €
- soldering iron

You can buy the hardware that you used in the workshop at the end of the workshop.

### Homework

After the workshop, you should be able to continue and finalise the project at home. You will need to:

- Buy a plant: 10 €
- Provide a waterproof case for the electronics: 2 €
- Provide battery power: 4 €

_Remark: If you didn't have the chance to debug during the workshop, you can also buy a cable for debugging "JST SH 1mm pitch 3 pin to male jumper". See the last section of this file about debugging._

## Preparations

Clone this repository first on your laptop:

```bash
git clone https://github.com/sysghent/plant-pot.git
```

This will allow you to easily run the example code in this repository and tweak it.

Next, you need to install Rust and add some exceptions to your `udev` rules to be able to flash the Raspberry Pico 2 W without root privileges.

1. Install [`rustup`](https://www.rust-lang.org/tools/install).

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    On certain operating systems, the `rustup` package is already available in the package manager. For example, on Debian-based systems, you can install it with:

    ```bash
    sudo apt install rustup
    ```

2. Verify that `cargo` and `rustc` are available in your shell's `PATH`:

    ```bash
    cargo --version
    rustc --version
    ```

3. Install compiler components for local development and cross-compilation for the Pico target:

    ```bash
    rustup install stable --profile default
    rustup target add thumbv8m.main-none-eabihf
    ```

4. Install `picotool` to flash the Raspberry Pico (in BOOTSEL mode).

    ```bash
    sudo apt install picotool
    picotool info
    ```

5. Update your `udev` rules to allow flashing without root privileges:

    ```bash
    sudo curl --output /etc/udev/rules.d/99-picotool.rules "https://github.com/raspberrypi/picotool/tree/master/udev/99-picotool.rules"
    ```

    Then reload the udev rules:

    ```bash
    sudo udevadm control --reload-rules
    sudo udevadm trigger
   ```

_Note: The Pico 2 W also has RISC-V cores, but for the moment they are less commonly used than the ARM cores. In case you want to use them and cross-compile for them, you will need to install the RISC-V Rust compiler toolchain and replace Cortex code by RiscV code. This workshop will focus on the ARM cores._

## Editors / IDE / development environment

Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) language server to get code completion, type hints, and other features in your editor.

```bash
rustup component add rust-analyzer
```

Please make sure you have a good editor or IDE installed. If you are a beginner and you don't have experience with programming, you can use:

- [Visual Studio Code](https://code.visualstudio.com/)
- [RustRover](https://www.jetbrains.com/rust/)

If you are more advanced and prefer not to touch your mouse, you can use:

- [Neovim](https://neovim.io/)
- [Helix](https://helix-editor.com/)

_Remark: If you really feel uncomfortable with Rust or the editors above, you can also use MicroPython on the Raspberry Pico 2 W. This is a Python interpreter that runs on the Pico and allows you to send Python code to the Pico for interpretation in real-time. Read the [official documentation](https://docs.micropython.org/en/latest/rp2/quickref.html) for more information on how to get started with `micropython` on the Raspberry Pi Pico. You can use the [thonny](https://thonny.org/) editor for Micropython. However, this workshop will focus on Rust._

## Blinking an LED

Look at the official Pico 2 W "pinout" SVG diagram provided in the [./cheatsheets](./cheatsheets) directory. It shows the different pins on the Pico 2 W and their functions.

_**Warning**: Disconnect the USB cable from the probe to prevent hardware damage, until you have completed the wiring._

Pick a pin where the voltage is high (or can be set high), close to a ground pin. If you put an LED in between, it should light because of the current flowing from the high to the ground pin.

Unplug your Pico and plug it back in while holding the "BOOTSEL" button. This will attach the Pico as a mass storage device.

Run `picotool info` to verify that the Pico is connected and ready to be flashed. Run the following command to run a basic "Hello World" program on the Pico:

```bash
cargo run --example on-board-blink
```

The mass storage device should now be automatically unmounted and the Pico should reboot into the program that you just flashed.

You should see the on-board LED blinking.

Before you dip your toes into the rest of the code in this workshop, it might helpful to read about the basics of the Rust programming language. A good starting point is the [Rust book](https://doc.rust-lang.org/book/).

**Exercise**: Make sure that you understand the concepts of `struct`s, `impl` blocks, and `async` functions in Rust.

To check if your code compiles and satisfies basic style conventions, you can run:

```bash
cargo clippy --examples
```

The Embassy framework, used in the project, provides an asynchronous executor that can be used to run many asynchronous tasks concurrently. Embassy tasks are run cooperatively: we assume they will give up (yield) control voluntarily to other tasks.

The Embassy project provides plugin crates for different support micro-controllers such as the Raspberry Pi Pico 2 W: [embassy-rp](https://crates.io/crates/embassy-rp). This crate is already added as a dependency to this project with the right configuration.

Let's take a closer look at the main loop in the `examples/external-blink.rs` file:

```rust
let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
loop {
    led_pin.set_high();
    ticker.next().await;
    led_pin.set_low();
    ticker.next().await;
}
```

You can see a few `await`-keywords in this sample. Just like in C#, JavaScript, Python or other languages, the `await` keyword is used to wait for an asynchronous operation to complete. In this case, it waits for the next tick of the `Ticker`.

The `Ticker` type is a primitive provided by Embassy and can be compared to Tokio's `Interval`. Embassy is an asynchronous framework for embedded systems (like micro-controllers). It allows users to **run software without an operating system**.

You might be thinking:

> "Do we really need an async framework for this?"

Below the surface, Embassy uses hardware timers and interrupts to put the CPU of the Pico to sleep while it is _waiting_. This is more efficient than constantly looping and running an if-then check, as is often done in Python (think: `while True:`).

The flavour of Embassy that is used in this workshop is located in a crate [`embassy-rp`](https://docs.embassy.dev/embassy-rp/git/rp235xb/index.html). This crate contains useful abstractions that correspond to hardware components on the Raspberry Pico 2 W, such as GPIO pins, ADC channels, and timers.

Let's take a closer look at the `Ticker` type:

```rust
let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
```

The `Duration` type (used on the previous sample) is a generic type offered by Embassy. This means you could easily port the blinker example to other chip architectures (if they support Embassy).

> **Exercise**:  Learn about the `Duration` type in Embassy. Modify the code in the main loop to make the LED blink faster or slower.

More advanced books that contain large sections about asynchronous programming are:

- For a general introduction: [Programming Rust](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)
- Written more like a step-by-step tutorial: [Asynchronous programming in Rust](https://www.packtpub.com/en-mt/product/asynchronous-programming-in-rust-9781805128137).

## A minimal Embassy program

Maybe it is useful to start with a minimal Embassy program that does not do anything, but can be used as a template for future programs.

```rust
#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::config::Config;
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_rp::init(config::Config::default());
}
```

As you can see, there are two strange lines on top of the file.

- `#![no_std]` means that the program does not use the standard library. Embedded systems are too small for the standard library.
- `#![no_main]` means that the program does not have a typical `main` function as on normal operating systems. Instead, calling and creating the `main` function is completely handled by the Embassy framework.

Then there are two `use x as _;` lines. These crates don't expose functions or public modules to be used, but they contain set-up code that should be included at least one in your embedded program.

- The `panic_probe` crate provides a panic handler that is compatible with Embassy.  Panics are **fatal errors**. Every embedded program needs a panic handler, because traditional panics would unwind or abort and yield control back to the operating system. This operating system is absent, so we have to tell the compiler how to handle panics. Usually, this means going in an infinite loop.

- The `defmt_rtt` is not useful for the moment, but once you have configured a hardware debugger, it will allow you to log messages to the debugger console. This is useful for debugging your program.

The `spawner` argument allows users to spawn asynchronous tasks. Keep in mind, however, that each task should be non-generic and completely specified at compile time. This is because the Embassy framework does not support dynamic task creation at runtime.

## USB serial input / output

For this section, you can detach all attached jumper wires and components from the Raspberry Pico, except for the USB.

Preparation for serial IO:

1. Add yourself to the `dialout` group to be able to access the serial port without root privileges:

    ```bash
    sudo usermod -aG dialout $USER
    ```

    Log out and log back in to apply the changes.

2. Run the serial echo example:

    ```bash
    cargo run --example serial-echo
    ```

3. Install the `tio` tool to be able to read the serial output of the Raspberry Pico:

    ```bash
    sudo apt install tio
    ```

4. List serial devices with `tio` (if you receive a "permission denied" error, you may need to re-login or reboot first). Look for the section _by-id_, which is more stable than the _by-path_ section:

    ```bash
    tio --list
    ```

5. Connect to the Pico from your laptop using a virtual serial connection that runs over USB (your id may be different):

    ```bash
    tio /dev/serial/by-id/usb-c0de_USB-serial_example-if00
    ```

6. If you are not able to connect, you can try different parameters for the serial connection or a different device path.

    ```bash
    tio -s 1 -d 8 -p none -b 9600 /dev/ttyACM1
    ```

From now on, you can send bytes to the Pico and also receive bytes from the Pico. All keyboard commands for `tio` are listed in [GitHub](https://github.com/tio/tio#32-key-commands). The most important one is `ctrl-t q` to quit the serial monitor.

_Remark: It is possible I made mistakes in the implementation of the USB serial wrapper. If you find any, you can have a look at the [example code from Embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/usb_serial.rs) that I used and compare._

The serial-over-usb functionality is placed inside the Rust library of this repository, in the file [`src/usb.rs`](src/usb.rs):

```rust
loop {
    match usb_io_handle.read_packet(&mut serial_in_buf).await {
        Ok(n) => {
            process(&serial_in_buf[0..n], &mut serial_out_buf).await;
            let _ = usb_io_handle.write_packet(&serial_out_buf).await;
            serial_in_buf.fill(0);
            serial_out_buf.fill(0);
        }
        Err(_) => todo!("Handle USB read error"),
    }
}
```

This asynchronous function takes a handle to the USB port and reads data from it. It then writes the same data back to the USB port, effectively echoing it back to the sender.

> **Exercise**: Implement a program that runs on your Pico and reverses every line sent from your laptop.

## Measuring moisture

From now on, you need to have a moisture sensor connected to the Raspberry Pico. The moisture sensor has three pins: VCC, GND, and the signal pin.

- Connect the VCC pin to the 3.3V output pin on the Pico,
- the GND pin to a ground pin on the Pico,
- and the signal pin to one of the ADC-capable pins on the Pico. This is the part that provides information to the Pico about how much moisture is in the soil.

A microcontroller is not continuously powered, but turns millions of times per second (CPU clock cycles). This means that we cannot really have a continuous measurement of the moisture in the soil. Instead, it is a discrete measurement that we can take at regular intervals.

We have to use the ADC (Analog-to-Digital Converter) of the Raspberry Pico to measure the moisture in the soil. The ADC converts the analog signal from the moisture sensor into a digital value that can be processed by the microcontroller.

> **Exercise**: Find all the pins on the Pico that can be used as ADC inputs.

Next:

> **Exercise**: How many bits are used by the ADC on the Raspberry Pico? How many different values can it measure? Is this standard across all microcontrollers?

Now you can flash an example program on your Pico. Remember from last time that you have to hold the "BOOTSEL" button while plugging in the Pico to mount it as a mass storage device.

```bash
cargo run --example calibrate-moisture-sensor
```

I created a simple program that can measure the moisture in the soil using ADC. The main loop has this code:

```rust
let level = adc.read(&mut moisture_pin).await.unwrap();
let voltage = adc_reading_to_voltage(level);
let moisture = voltage_to_moisture(voltage);
```

Notice that we need a handle to the ADC component and also a pin configured as an ADC channel.

Now you probably want some output from the moisture sensor over USB. So open up a serial monitor on your laptop, while the Pico is running.

> **Exercise**: Fill in the `todo!` macro-calls inside the bodies of the conversion functions `adc_reading_to_voltage` and `voltage_to_moisture`. Hint: these functions are similar to the `map` function in ArduinoIDE.

Maybe more difficult:

> **Exercise**: In which situations would need to sample multiple ADC values in on read?

## Water pump

After reverse-engineering the parameters for the moisture sensor, we can now use the data to control a water pump.

The water pump is a small 3V submersible pump that can be controlled by a GPIO pin (which may be wired to a transistor) on the Raspberry Pico.  The transistor acts as a switch that can be controlled by the GPIO pin on the Raspberry Pico.

_**Import**: For protecting the Pico you should put a diode in the circuit with the pump, to prevent current from flowing back into the Pico when the pump is turned off. This is called a flyback diode._

_Remark: The transistor allows the water pump to be powered by a higher voltage source, such as a battery or an external power supply. However, in this project we don't need that._

You can see the pump in action by running the example:

```bash
cargo run --example water-pump
```

Notice, in the [source code](examples/water-pump.rs), that we are now using a static channel:

```rust
static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();
```

The generic parameter `<_,_,1,3,1>` part means that the channel can cash one value, has a maximum of three subscribers, and one publisher. The `CriticalSectionRawMutex` is used to ensure that the channel can be accessed safely from multiple tasks.

This is certainly more verbose than Tokio's channels, but in an embedded context, you probably don't want to create many subscribers and publishers at runtime. Instead, you want to create them at compile time, so that the code is more predictable and deterministic.

Static variables are like global variables. They should be initialised before the actual program runs. Since they "always" have a value, they can be used to communicate between different tasks in the program.

It is important to know that mutating static (global) variables is not allowed by default in Rust. This is because it may cause race conditions between different tasks mutating the static variable in parallel.

A channel usually comes in two sides: an input and an output. Let's have a look at the sending part of an [Embassy channel](https://docs.embassy.dev/embassy-sync/git/default/index.html):

```rust
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
```

The sending task is an async task, because we are not interested in measurements that are closer than 500 milliseconds apart. The `Ticker` is used to wait for the next measurement interval and allow competing async tasks to progress work.

The `publisher` is used to send the moisture value to the channel. The `publish_immediate` method is used to send the value immediately, and drop any old values not yet consumed by the receiving task.

## Pulse Width Modulation (PWM)

The speed of the motor should always be at its maximum. Instead, you would try to turn up the motor speed gradually. By default GPIO pins output the highest voltage (3.3V) when set to high.

PWM can tweak the output voltage of a GPIO pin by rapidly switching it on and off. The ratio of the time the pin is high to the time it is low is called the **duty cycle**. A higher duty cycle means a higher average voltage.

You try it out by running the example:

```bash
cargo run --example calibrate-speed-water-pump
```

The first exercise will allow you to manually set the speed of the water pump by typing a number in the serial monitor.

> **Exercise**: Write a function that parses the bytes  coming in over serial connection into moisture numbers.

Next, you should try to adjust the speed of the water pump based on the received intensity numbers.

- Listen for new numbers coming in over the serial connection.
- Parse the numbers and convert them to a speed value.
- Send the speed value through the sender of a `PubSubChannel` to another task.
- Receive the speed value in the task that controls the water pump.
- Compute the duty cycle based on the speed value and set the PWM output accordingly.

> **Exercise**: Use the incoming numbers over serial USB to change the speed of the water pump dynamiccally at runtime.

The Pico board also has multiple PIO peripherals. This is a programmable input/output peripheral that can be used to implement custom protocols and control devices.

Creating a PWM output with the PIO peripheral requires more work, but may be more performant than using simpler ways to drive PWM outputs. See <https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/pio_pwm.rs>

## HTTP notifications

The setup of HTTP communication in Rust is more difficult than in MicroPython. On the other hand, it is more powerful and flexible.

1. Make an account on [Ntfy](https://docs.ntfy.sh).

2. Install the mobile Ntfy app on your phone (optional) or use another platform to receive notifications.

3. Try publishing a notification from you command line using `curl`:

    ```bash
    curl -X POST https://ntfy.sh/sysghent -d "$USER will water the plants!"
    ```

Instead of using `curl` you can also use your Pico to send notifications.

Now, you should configure your Wifi's authentication details in the [.cargo/config.toml](.cargo/config.toml) file of this repository.

```toml
[env]
PASSWORD = "?"      # WiFi password
SSID = "?"          # WiFi SSID
```

After filling in the secrets (don't commit them to GitHub), you can try out a program that will send notifications regularly to the Ntfy service. If you subscribe to the associated channel / topic, you can receive them on your phone or laptop.

```bash
cargo run --example http-notifications
```

> **Exercise**: Make the messages emitted to `ntfy` by the Pico prettier or more informative (e. g. containing some numerical data).

## Levels of abstraction in embedded Rust

Writing programs for microcontrollers can be done at different levels of abstraction.

### Low level

The lowest level of abstraction for software running on a microcontroller, is the MCU. The MCU enables access to the core processor. See [Cortex-M](https://crates.io/crates/cortex-m).

On top of the MCU, there always is a "peripheral access crate" (the PAC). This crate contains code generated from SVD files provided by the board manifacturer. See the [RP-PAC](https://crates.io/crates/rp235x-pac)

The Embassy framework builts on top of the PAC to provide a more intuitive / convenient API for accessing the hardware.

### Medium level

In case you feel like the Embassy framework does not allow you do certaint things, you can fall-back to a more convential level of abstraction, without async/await.

The "hardware access layer" (HAL) is a more convenient way to access the hardware of the microcontroller. It provides a higher level of abstraction than the PAC, but still allows you to access the hardware directly.

See [rp235x-hal](https://crates.io/crates/rp235x-hal) and [examples](https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples).

_Remark: If you want to be able to **kill async tasks**, you should not use Embassy, but instead use [RTIC](https://github.com/rtic-rs/rtic) which allow pre-emptive killing of running tasks. You can also assign priorities to different tasks, which may be required for sensitive applications. However, it is not yet stable._

### High level

Normally, for commonly used micro-controllers, there should at least be one good board support package (also called BSP). These so-called packages are actually creates that have a very generic API, but less customisable. For example, in the case of the Microbit controller, the BSP is called [microbit](https://crates.io/crates/microbit) and it allows you draw visual shapes on the on-board LED array.

For the Raspberry Pico 2 W, `embassy` (and the plugin `embassy-rp`) come the closest to a real BSP.

### More reading material

Hands-on embedded Rust books:

- There is a book for beginners in embedded Rust:  [Rust Discovery Embedded book](https://docs.rust-embedded.org/discovery-mb2/). It assumes you have bought a Microbit v2 (20 euros).
- There is also a book about embedded Rust using an STM32 chip: [Embedded Rust book](https://docs.rust-embedded.org/book/).
- Another book about Rust and the Pico 2 [Pico Pico](https://pico.implrust.com)

## Using small microcontrollers

In case you work with smaller micro-controllers than the Pico, you can follow the tips no [Min-sized Rust](https://github.com/johnthagen/min-sized-rust) to be able to fit a Rust binary on the chip.

The most important tip is probably compiling with `--release` and using the `opt-level = "s"` option in the `Cargo.toml` file.:

```toml
[profile.release]
opt-level = "s"
```

This will optimize the binary size, but it will also make the code run slower.

## Using a hardware debugger (if hardware available)

Most popular microcontrollers that are used for educational purposes, there is already some hardware debugging support (also called a **hardware debug probe**) on the board itself: such as the [Microbit](https://microbit.org/) or the [ESP32](https://www.espressif.com/en/products/socs/esp32).

Having this debug probe allows you to debug the code running on the target Pico using GDB or other debugging tools through the debug probe.

A debug probe comes in the form of a small secundary chip that can be used to debug the main microcontroller on the board.

The Pico family of microcontrollers does not have this feature built-in. However, it is possible to turn a spare Raspberry Pico into a hardware debugging probe for another Pico.

If you have the right calbes, sockets and pins, you can turn a spare Pico into a real hardware debugging probe for your current Pico:

- One side is [JST-SH](https://www.kiwi-electronics.com/en/jst-sh-1mm-pitch-3-pin-to-male-headers-cable-100mm-long-19930)
- The other side has male pins that can be plugged in a breadboard
- Your Pico has a socket that can fit the JST-SH

### Flashing a `picoprobe`

The Raspberry foundation provides images for the Pico's that can be flashed to turn a Pico into a hardware debugging mode.

1. Download the latest `debugprobe_on_pico2.uf2` flash image from the official [`picprobe`](https://github.com/raspberrypi/debugprobe/releases) releases.

2. Attach the Pico to your laptop while holding the white BOOTSEL mode.

3. Drop the downloaded `uf2` file on the mass storage drive emulated by the Pico. Wait for a fraction of a second while the Pico unmounts.

Now you have successfully made a cheap hardware debugging probe.

We still need to wire this homemade probe to the target Pico that we want to debug.

- Assume **D** is the homemade debug probe (a Pico).
- Assume **T** is the target Pico.

### SWD wiring

Right now, there is no cabling yet between the debug probe and the target Pico. The cables should be connected such that **D** can detect **T** over the SWD debugging protocol.

1. Find the cable that has a tiny white connector on one side (JST PH 3-pin) and three male jumber cables on the other side.

2. Plug the white connector of the JST cable into the SWD socket of **D**.

Place **T** and **D** in parallel  with USB ports facing upwards (to prevent confusion).

The three male header pins should be connected to **D** as follows:

- **T** left (yellow) <-> **D** pin n. 5
- **T** middle (black) <-> **D** pin n. 3
- **T** right (orange) <-> **D** pin n. 4

Instead of pin number, you can als use the pin names:

- **T** SWCLK <-> **D** GP3
- **T** SWDIO <-> **D** GP2
- **T** GND <-> **D** GND

Provide power to **T** with only one USB cable by forwarding it from the power of **D**:

- **T** GND pin n. 38 <-> **D** pin n. 38 (connect ground)
- **T** VSYS pin n. 39 <-> **D** pin n. 39 (connect power supply)

_Remark: You can also connect **T** to **D** for UART communication. Exercise: find out if this is really necessary and let me know._

### Development setup

1. Install `cargo-embed`, which is included in the `probe-rs` tools suite.

    ```bash
    cargo install probe-rs-tools
    ```

2. Verify that `cargo-embed` is available in your shell's `PATH` (cargo-[CMD] can be called with `cargo [CMD]`):

    ```bash
    cargo embed --version
    ```

3. Add `udev` rules for `probe-rs` as described in [probe-rs documentation](https://probe.rs/docs/getting-started/probe-setup/). Follow the same steps as for `picotool`.

    ```bash
    sudo curl --output /etc/udev/rules.d/69-probe-rs.rules  "https://probe.rs/files/69-probe-rs.rules"
    sudo udevadm control --reload-rules
    sudo udevadm trigger
    ```

### Flashing the target

Adjust the `Embed.toml` file in the root of this repository if necessary. This file configures the behaviour of the `cargo embed` command when run on your laptop.

For example, if the configuration contains the following, a GDB debug session server will be started and the loaded program will be reset to the first instruction.

```toml
[default.gdb]
enabled = true

[default.reset]
halt_afterwards = true
```

You can now flash the target Pico 2 W with one of the examples without holding the white BOOTSEL button while plugging in. You can choose between the following commands:

Flash with the `cargo embed` command, which will also open an RTT console:

```bash
cargo embed --example on-board-blink
```

This will compile and then flash the binary using the SWD connection between the debug probe and the target device. This may be a little bit slower than using the previous BOOTSEL approach, but it gives better debugging output and we don't have to re-plug the USB cable.

Configure the `cargo embed` command as the default Cargo runner in [`.cargo/config.toml`](.cargo/config.toml), so you can run your embedded binary on the Pico directly with `cargo run`:

```toml
[target.thumbv8m.main-none-eabi]
runner = "cargo embed --chip RP235x"
```

Now you can flash with `cargo run`, in exactly the same way as done previously (there were some issues with this command):

```bash
cargo run --example on-board-blink
```

## Using RTT for logging

RTT (Real-Time Transfer) is a logging protocol that can be used on top of an SWD connection.

Import the `defmt` crate to add log statemens throughout your code. Just import macro's such as `defmt::info!`, `defmt::error!`, and `defmt::warn!` to log messages, similar to how you would use `log::info!` in standard Rust.

Then you have to enable a "transport" for `defmt` which is usally `RTT`, implemented by linking your code with the `defmt-rtt` crate. `defmt-rtt` is the analogue of the

1. Add `defmt` and `defmt-rtt` as dependency to your `Cargo.toml` file. Also enable the `defmt` features for all existing dependencies that have it.
2. Import the `defmt-rtt` module in your binary or library:

    ```rust
    use defmt_rtt as _;
    ```

    This may seem useless but it allows the setup of some data that is necessary to link the binary against the `defmt-rtt` crate.

3. Add a compiler flag under the current target  in `.cargo/config.toml` file: `-C link-arg=-Tdefmt.x`.

    ```toml
    [target.thumbv8m.main-none-eabihf]
    rustflags = [
        "-C",
        "link-arg=--nmagic",
        "-C",
        "link-arg=-Tlink.x",
        "-C",
        "link-arg=-Tdefmt.x",
        "-C",
        "target-cpu=cortex-m33",
    ]
    ```

4. Specify the log-level for `defmt` in the `.cargo/config.toml` file:

    ```toml
    [env]
    DEFMT_LOG = "debug"
    ```

5. Enable `rtt` in the `Embed.toml` file:

    ```toml
    [default.rtt]
    enabled = true
    ```

6. Compile your binary (with `defmt::info!` statements), flash it and run it on the target Pico 2 W:

    ```bash
    cargo run --example on-board-blink
    ```

    This should open an RTT console that shows the log messages emitted by the `defmt::info!` statements in your code.

## Starting a debugging session

 Prevent lines being merged or re-ordered during the build step of your program. This kind of changes can make it harder for the debugger to stop at the right breakpoints. Add the following to your `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```

To be sure the new configuration is used, you can reset the `target` build cache:

```bash
cargo clean
cargo run --example [BINARY_EXAMPLE_NAME]
```

Install the multi-architecture version of `gdb` to be able to debug the Raspberry Pi Pico board:

```bash
sudo apt-get install gdb-multiarch
```

The exact binary name may vary, but it is important that the installed `gdb` supports the architecture of your target chip. In the case of a Pico 2, `gdb` needs `ARM` support built-in.

Configure the `Embed.toml` file to enable the `gdb` server:

```toml
[default.gdb]
enabled = true
```

Then run the following command to create and connect a `gdb` debugging client:

```bash
gdb-multiarch target/thumbv8m-none-eabi/debug/[BINARY_EXAMPLE_NAME]
```

Within the `gdb` client on your laptop, you have to connect to the running `GDB` server on the debug Pico:

```gdb
target remote :1337
monitor reset halt # optionally resets to the first instruction
```

_Remark: In VS Code, you can install the `probe-rs-debug` extension to use the `probe-rs` toolkit for debugging. It uses some other kind of protocol than `gdb`. See [instructions](https://probe.rs/docs/tools/debugger/)_

## Jumping between breakpoints

Breakpoints can be set in the `gdb` client by using the `break` command followed by a line number or function name:

```gdb
break [FUNCTION_NAME]  # Set a breakpoint at a specific function
break [LINE_NUMBER]  # Set a breakpoint at a specific line number
break [FILE_NAME]:[LINE_NUMBER]  # Set a breakpoint at a specific line in a file
```

You can also write hardware breakpoints directly in your code with `cortex_m::asm::bkpt()`.

To progress throughout the execution of your debugged program you can use:

```gdb
continue  # Continue execution until the next breakpoint is hit
next # Step to the next line of code
```
