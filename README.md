Here is the corrected version of your text, with grammar and spelling mistakes fixed for clarity and accuracy.

# Workshop: Make a Smart Plant Pot

*Notes for a workshop organized by Hugo & Willem in Ghent on June 4, 2025, for [SysGhent](https://sysghent.be/events/plant-pot).*

In this workshop, you will learn how to create a plant pot that can automatically water itself when the soil is too dry and send notifications over Wi-Fi.

## Overview

### Prerequisites

Please bring:

* A laptop (preferably with Linux installed)
* A laptop charger
* A Micro-USB cable
* A water container (a sponge or a pot for wet vs. dry soil testing is also fine)

It might be helpful to look up these Rust concepts at home before you join the workshop:

* What closures are and what they capture.
* The idea behind concurrency with asynchronous programming. Read some concrete examples of async Rust programs.
* The concepts of ownership and moving values in Rust.

See the [official Rust book](https://doc.rust-lang.org/book/) for more information on these concepts.

### Provided

You can borrow from us (or bring your own):

* [Raspberry Pi Pico 2 W](https://datasheets.raspberrypi.com/picow/pico-w-datasheet.pdf): \~€10
* Analog capacitive moisture sensor: \~€4
* JST SH 1mm Pitch 3-Pin to Male cable: €1.5
* 3V submersible water pump: \~€4
* Breadboard: \~€5
* Jumper wires: \~€1
* LED: \~€0.5
* MOSFET transistor: \~€0.5

You can purchase the hardware used during the workshop at its conclusion.

*Note: The Pico 2 W also has RISC-V cores, but at the moment they are less commonly used than the ARM cores. If you want to use them and cross-compile, you will need to install the RISC-V Rust compiler toolchain and replace Cortex code with RISC-V code. This workshop will focus on the ARM cores.*

### Homework

After the workshop, you should be able to continue and finish the project at home. You will need to:

* Buy a plant: €10
* Provide a waterproof case for the electronics: €2
* Provide battery power: €4

*Remark: If you didn't get a chance to debug during the workshop, you can also buy a "JST SH 1mm pitch 3-pin to male jumper" cable for debugging. See the last section of this document about debugging.*

## Preparations

First, clone this repository on your laptop:

```bash
git clone https://github.com/sysghent/plant-pot.git
```

This will allow you to easily run and tweak the example code in this repository.

### Main Development Dependencies

Next, you need to install Rust and add some exceptions to your `udev` rules to be able to flash the Raspberry Pi Pico 2 W without root privileges.

1. Install [`rustup`](https://www.google.com/search?q=%5Bhttps://www.rust-lang.org/tools/install%5D\(https://www.rust-lang.org/tools/install\)).

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    On certain operating systems, the `rustup` package is already available in the package manager. For example, on Debian-based systems, you can install it with:

    ```bash
    sudo apt install rustup
    ```

    **Warning**: Do not install it through `brew` on macOS. This may lead to breakage.

2. Verify that `cargo` and `rustc` are available in your shell's `PATH`:

    ```bash
    cargo --version
    rustc --version
    ```

3. Add the `~/.cargo/bin` directory to your shell's `PATH`. This will allow you to use `rustup` and binaries built with `cargo` from anywhere in the terminal.

4. Install compiler components for local development and cross-compilation for the Pico target:

    ```bash
    rustup install stable --profile default
    rustup target add thumbv8m.main-none-eabihf
    ```

## Configure a Hardware Debugger

On most popular microcontrollers used for educational purposes, there is already some hardware debugging support (also called a **hardware debug probe**) on the board itself, such as on the [Micro:bit](https://microbit.org/) or the [ESP32](https://www.espressif.com/en/products/socs/esp32).

Having this debug probe allows you to debug the code running on the target Pico using GDB or other debugging tools.

A debug probe comes in the form of a small secondary chip that can be used to debug the main microcontroller on the board.

The Pico family of microcontrollers does not have this feature built-in. You have two options for debugging a Pico:

* It is possible to turn a spare Raspberry Pi Pico into a hardware debugging probe for another Pico.
* Buy (or borrow) an official Raspberry Pi hardware debug probe.

In this workshop, we will pursue the first option. If you get stuck, feel free to ask for a pre-made hardware debugger.

### Turning a Normal Pico into a Debugger Pico

The Raspberry Pi Foundation provides images for Picos that can be flashed to turn a Pico into a hardware debugger.

1. Download the latest `debugprobe_on_pico.uf2` flash image from the official [`debugprobe`](https://github.com/raspberrypi/debugprobe/releases)releases.
2. Attach the Pico to your laptop while holding the white BOOTSEL button. A mass storage device will appear in your file manager. It will be called something like `RPI-RP2`.
3. Drop the downloaded `.uf2` file onto the mass storage drive emulated by the Pico. Wait for a fraction of a second while the Pico unmounts and reboots as a fresh `debugprobe`.

Now you have successfully made a cheap hardware debugging probe.

### Wire Target to Debugger

Let's make some aliases:

* Assume **D** is the homemade debug probe (a Pico).
* Assume **T** is the target Pico.

Right now, there is no cabling between the debug probe and the target Pico. The cables should be connected such that **D** can detect **T** over the SWD debugging protocol.

***Important**: For this step, you need to have a JST-SH cable. You can find them on [Kiwi](https://www.kiwi-electronics.com/en/jst-sh-1mm-pitch-3-pin-to-male-headers-cable-100mm-long-19930), but they can be hard to find.*

1. Plug the white connector of the JST cable into the SWD socket of **D**.

2. Place **T** and **D** in parallel with their USB ports facing upwards (to prevent confusion).

3. Connect the male jumper pins. The three male header pins from **T**'s JST cable should be connected to **D** as follows:

      * **T** left (yellow) \<-\> **D** pin 5
      * **T** middle (black) \<-\> **D** pin 3
      * **T** right (orange) \<-\> **D** pin 4

    Instead of pin numbers, you can also use the pin names:

      * **T** SWCLK \<-\> **D** GP3
      * **T** SWDIO \<-\> **D** GP2
      * **T** GND \<-\> **D** GND

4. Provide power to **T** using a single USB cable by forwarding power from **D**:

      * **T** GND pin 38 \<-\> **D** pin 38 (Connect ground)
      * **T** VSYS pin 39 \<-\> **D** pin 39 (Connect power supply)

*Remark: You can also connect **T** to **D** for UART communication. However, I have not needed it so far.*

### Configure Flashing from Laptop

There is still one step remaining: we have to configure our laptop's development environment to enable flashing (this applies to any microcontroller with an onboard or external debugger).

1. Install `cargo-embed`, which is included in the `probe-rs` tool suite.

    ```bash
    cargo install probe-rs-tools
    ```

2. Verify that `cargo-embed` is available in your shell's `PATH` (`cargo-[CMD]` can be called with `cargo [CMD]`):

    ```bash
    cargo embed --version
    ```

3. Add `udev` rules for `probe-rs` as described in the [probe-rs documentation](https://probe.rs/docs/getting-started/probe-setup/).

    ```bash
    sudo curl --output /etc/udev/rules.d/69-probe-rs.rules "https://probe.rs/files/69-probe-rs.rules"
    sudo udevadm control --reload-rules
    sudo udevadm trigger
    ```

Now you can flash changes in the source code directly to the target Pico (without re-plugging it or holding the BOOTSEL button). The debug probe Pico will function as an intermediary between your laptop and the target Pico.

```bash
cargo run --example external-blink
```

You should see two progress bars running to completion in your terminal. As soon as the flash process is finished:

* **T** will start running the new code.
* A debug server will be started on **D** so that you can step through your code while it runs on **T**.

While the Pico has a generous amount of flash memory, Embassy-produced binaries can sometimes be large. For microcontrollers with less memory, the [Min-sized Rust](https://github.com/johnthagen/min-sized-rust) guide provides tips for reducing binary size.

## Editors / IDE / Development Environment

Install the [rust-analyzer](https://rust-analyzer.github.io/manual.html) language server to get code completion, type hints, and other features in your editor.

```bash
rustup component add rust-analyzer
```

Please make sure you have a good editor or IDE installed. If you are a beginner without much programming experience, you can use:

* [Visual Studio Code](https://code.visualstudio.com/)
* [RustRover](https://www.jetbrains.com/rust/)

If you are more advanced and prefer not to touch your mouse, you can use:

* [Neovim](https://neovim.io/)
* [Helix](https://helix-editor.com/)

*Remark: If you feel uncomfortable with Rust or the editors above, you can also use MicroPython on the Raspberry Pi Pico 2 W. This is a Python interpreter that runs on the Pico and allows you to send Python code to it for real-time interpretation. Read the [official documentation](https://docs.micropython.org/en/latest/rp2/quickref.html) for more information on how to get started with `micropython` on the Raspberry Pi Pico. You can use the [Thonny](https://thonny.org/) editor for MicroPython. However, this workshop will focus on Rust.*

## Blinking an LED

Look at the official Pico 2 W "pinout" SVG diagram provided in the [./cheatsheets](./cheatsheets) directory. It shows the different pins on the Pico 2 W and their functions.

***Warning**: Disconnect the USB cable from the probe until you have completed the wiring to prevent hardware damage.*

Pick a GPIO pin that is easy to find and close to a ground pin. This avoids having to count pins in the middle of the board.

Put the long leg of the LED into the chosen GPIO pin (for example, GPIO 16). Put the short leg into a ground pin. If you want to be cautious, you can add a resistor of around 100 Ohms in series.

```bash
cargo run --example external-blink
```

The connected LED will start flashing.

*Before you dip your toes into the rest of the code in this workshop, it might be helpful to read about the basics of the Rust programming language. A good starting point is the [Rust book](https://doc.rust-lang.org/book/).*

To check if your code compiles and satisfies basic style conventions, you can run:

```bash
cargo clippy --examples
```

Try to make a typo in the `external-blink.rs` file and retry `cargo clippy --examples`. See what happens. Your editor should detect an error and warn you. If this is not the case, please ask for help.

The Embassy framework, used in this project, provides an asynchronous executor that can be used to run many asynchronous tasks concurrently. Embassy tasks are run cooperatively: we assume they will give up (yield) control voluntarily to other tasks.

The Embassy project provides plugin crates for different supported microcontrollers, such as the Raspberry Pi Pico 2 W: [embassy-rp](https://crates.io/crates/embassy-rp). This crate is already added as a dependency to this project with the right configuration.

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

You can see a few `await` keywords in this sample. Just like in C\#, JavaScript, Python, or other languages, the `await` keyword is used to wait for an asynchronous operation to complete. In this case, it waits for the next tick of the `Ticker`.

The `Ticker` type is a primitive provided by Embassy and can be compared to Tokio's `Interval`. Embassy is an asynchronous framework for embedded systems (like microcontrollers). It allows users to **run software without an operating system**.

You might be thinking:

> "Do we really need an async framework for this?"

Below the surface, Embassy uses hardware timers and interrupts to put the Pico's CPU to sleep while it is *waiting*. This is more efficient than constantly looping and running an if-then check, as is often done in Python (think: `while True:`).

In general, using Rust and Embassy for microcontrollers (instead of MicroPython) **may improve performance and energy efficiency significantly**.

To use Embassy on a particular model of microcontroller, you need some glue, which comes in the form of an "adapter" crate. In this workshop, the adapter crate is [`embassy-rp`](https://docs.embassy.dev/embassy-rp/git/rp235xb/index.html). This crate contains useful abstractions that correspond to hardware components on the Raspberry Pi Pico 2 W, such as GPIO pins, ADC channels, and timers.

Let's take a closer look at the `Ticker` type:

```rust
let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
```

The `Duration` type (used in the previous sample) is a generic type offered by Embassy. This means you could easily port the blinker example to other chip architectures (if they support Embassy).

> **Exercise**: Learn about the `Duration` type in Embassy. Modify the code in the main loop to make the LED blink faster or slower.

More advanced books that contain large sections about asynchronous programming are:

* For a general introduction: [Programming Rust](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)
* Written more like a step-by-step tutorial: [Asynchronous Programming in Rust](https://www.packtpub.com/en-mt/product/asynchronous-programming-in-rust-9781805128137).

## A Minimal Embassy Program

It can be useful to start with a minimal Embassy program. The following does nothing but can serve as a template for future programs.

```rust
#![no_std]
#![no_main]

use defmt_rtt as _;
use embassy_executor::{Spawner, main};
use embassy_rp::config::Config;
use panic_probe as _;
use embassy_rp::bind_interrupts;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[main]
async fn main(_spawner: Spawner) -> ! {
   let p = embassy_rp::init(Config::default());
   loop {
      embassy_futures::yield_now().await;
   }
}
```

As you can see, there are two notable attributes at the top of the file.

* `#![no_std]` means that the program does not use the standard library. Embedded systems are too small for the standard library.
* `#![no_main]` means that the program does not have a typical `main` function (with arguments or an exit code) as on a typical operating system. Instead, calling and creating the `main` function is completely handled by the Embassy framework.

Then there are two `use x as _;` lines. These crates don't expose functions or public modules to be used, but they contain setup code that should be included at least once in your embedded program.

* The `panic_probe` crate provides a panic handler that is compatible with Embassy. Panics are **fatal errors**. Every embedded program needs a panic handler because traditional panics would unwind or abort and yield control back to the operating system. This operating system is absent, so we have to tell the compiler how to handle panics. Usually, this means going into an infinite loop.
* The `defmt_rtt` is not useful for the moment, but once you have configured a hardware debugger, it will allow you to log messages to the debugger console. This is useful for debugging your program.

There is a macro call `embassy_rp::bind_interrupts!` that binds hardware interrupts with the Embassy framework. This is necessary to be able to use hardware interrupts in your program. Hardware interrupts can stop the current ongoing computation and jump execution to some handler code elsewhere. Examples of hardware interrupt bindings available on the Pico 2 are:

* `PIO0_IRQ_0` is an interrupt coming from the PIO peripheral.
* `USBCTRL_IRQ` for USB interrupts (relevant in USB serial communication).
* `ADC_IRQ_FIFO` for ADC interrupts (relevant for reading data from the analog-to-digital converter in the moisture sensor).

The `spawner` argument allows users to spawn asynchronous tasks. Keep in mind, however, that each task should be non-generic and completely specified at compile time. This is because the Embassy framework does not support dynamic task creation at runtime.

The last line `loop { yield_now().await }` may seem unnecessary. The reason I have to write it is because the return type of `main` is "never" (written as `!`). The `never` return type is the type for a function that never returns.

Because of the signature of `main`, we cannot simply escape the `main` function. Running this program is the only thing that happens on the microcontroller. So we have to keep looping, even if we have already finished our work.

## Simple Logging

RTT (Real-Time Transfer) is a logging protocol that can be used on top of an SWD connection. It does not require specifying the baud rate, etc.

The `defmt` crate is the most popular crate for logging from embedded Rust programs. It exports macros like `info!` and `debug!`, similar to the macros in the standard `log` or `tracing` crates in Rust.

For the debug probe to actually show the log output from the target, you need to enable a "transport". In the case of `defmt`, it is usually the `RTT` transport using the `defmt-rtt` crate. The `defmt-rtt` crate could be compared to `tracing-subscriber` or other mainstream log consumers.

1. Add `defmt` and `defmt-rtt` as a dependency to your `Cargo.toml` file. Also, enable the `defmt` features for all existing dependencies that have it.

2. Import the `defmt-rtt` module in your binary or library:

    ```rust
    use defmt_rtt as _;
    ```

    This may seem useless, but it enables the setup of data necessary to link the binary against the `defmt-rtt` crate.

3. Add a compiler flag under the current target in the `.cargo/config.toml` file: `-C link-arg=-Tdefmt.x`.

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

4. Specify the log level for `defmt` in the `.cargo/config.toml` file:

    ```toml
    [env]
    DEFMT_LOG = "debug"
    ```

5. Enable `rtt` in the `Embed.toml` file:

    ```toml
    [default.rtt]
    enabled = true
    ```

6. Add invocations of the `defmt` macros throughout your library or binary code (as necessary). For example, you could write:

    ```rust
    use defmt::info;

    async fn main(_spawner: Spawner) -> ! {
       loop {
          info!("A new iteration of the loop has started.");
       }
    }
    ```

    There is nothing stopping you from adding such statements to library code.

7. Compile, flash, and run your binary on the target Pico 2 W:

    ```bash
    cargo run --example on-board-blink
    ```

    This should open an RTT console that shows the log messages emitted by the `defmt` statements in your code.

## USB Serial Input/Output

You may also wish to send input to the Pico. This cannot be done with the debugger (as far as I know). You need to set up a serial connection with your laptop. This usually means attaching a second cable from your laptop to the Pico.

The following steps are required to start up a serial monitor (a kind of terminal) with the target, which is connected over USB:

1. Add yourself to the `dialout` group to be able to access the serial port without root privileges:

    ```bash
    sudo usermod -aG dialout $USER
    ```

    Log out and log back in to apply the changes.

2. Run the serial echo example:

    ```bash
    cargo run --example serial-echo
    ```

3. Install the `tio` tool to be able to read the serial output of the Raspberry Pi Pico:

    ```bash
    sudo apt install tio
    ```

    If you are unable to install or configure `tio`, you can also try `minicom` instead.

4. List serial devices with `tio` (if you receive a "permission denied" error, you may need to re-login or reboot first). Look for the `by-id` section, which is more stable than the `by-path` section:

    ```bash
    tio --list
    ```

5. Connect to the Pico from your laptop using a virtual serial connection that runs over USB (your ID may be different):

    ```bash
    tio /dev/serial/by-id/usb-c0de_USB-serial_example-if00
    ```

6. If you are not able to connect, you can try different parameters for the serial connection or a different device path.

    ```bash
    tio -s 1 -d 8 -p none -b 9600 /dev/ttyACM1
    ```

You can now send bytes to and receive bytes from the Pico. All key commands for `tio` are listed on [GitHub](https://github.com/tio/tio#32-key-commands). The most important one is `Ctrl+T, Q` to quit the serial monitor.

*Remark: It is possible I made mistakes in the implementation of the USB serial wrapper. If you find any, you can take a look at the [example code from Embassy](https://github.com/embassy-rs/embassy/blob/main/examples/rp/src/bin/usb_serial.rs) that I used and compare.*

The serial-over-USB functionality is located in the repository's Rust library, in the file `src/usb.rs`:

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

## Measuring Moisture

From now on, you need to have a moisture sensor connected to the Raspberry Pi Pico. The moisture sensor has three pins: VCC, GND, and the signal pin.

* Connect the VCC pin to the 3.3V output pin on the Pico.
* Connect the GND pin to a ground pin on the Pico.
* Connect the signal pin to one of the ADC-capable pins on the Pico. This pin provides information to the Pico about the moisture level in the soil.

A microcontroller's CPU operates in discrete steps, governed by a clock that cycles millions of times per second. This means we cannot measure a signal continuously; instead, we must take digital samples of the analog signal. We have to obtain digital measurements of the analog signal and feed these measurements to the CPU, which operates on digital values.

We have to use the ADC (Analog-to-Digital Converter) of the Raspberry Pi Pico to measure the moisture in the soil. The ADC converts the analog signal from the moisture sensor into a digital value that can be processed by the microcontroller.

The typical workflow for using the ADC is as follows (assuming we measure the moisture on GPIO pin 26):

```rust
let adc_component = Adc::new(p.ADC, Irqs, Config::default());
let mut moisture_adc_channel = Channel::new_pin(p.PIN_26, Pull::None);
let level = adc_component.read(&mut moisture_adc_channel).await.unwrap();
```

Notice you need to instantiate the `Adc` component first, which is a handle to the ADC hardware on the Raspberry Pi Pico. You also need to create an `ADC` channel that represents an analog input. Calling the `read` method on the ADC component will wait for and take a digital measurement from the ADC.

> **Exercise**: Find all the pins on the Pico that can be used as ADC inputs.

Next:

> **Exercise**: How many bits are used by the ADC on the Raspberry Pi Pico? How many different values can it measure? Is this standard across all microcontrollers?

You can now calculate the moisture in the soil with some constants and helper functions (based on the ADC level). You will roughly need these functions:

```rust
let level = adc.read(&mut moisture_pin).await.unwrap();
let voltage = adc_reading_to_voltage(level);
let moisture = voltage_to_moisture(voltage);
```

However, the function bodies in the example code are empty.

> **Exercise**: Fill in the `todo!` macro calls inside the bodies of the conversion functions `adc_reading_to_voltage` and `voltage_to_moisture`. Hint: these functions are similar to the `map` function in the Arduino IDE.

You can now flash an example program that can be used to calibrate the moisture sensing on your Pico.

```bash
cargo run --example calibrate-moisture-sensor
```

You should be able to see output through a serial monitor or RTT console on your laptop after flashing. Try connecting the moisture sensor and dipping it in water. Do you see any changes?

* Does more water result in a larger voltage?
* What are the low and high voltage readings?

Experiment with a cup of water or a sponge. Reflash your program until you are satisfied with the result. The result should ideally be a floating-point number between 0 and 1 or a moisture percentage.

## Debugging with GDB

Once you start creating slightly more complicated embedded programs, you might want to:

* introspect the values of local variables
* follow the execution flow

For this, you need a piece of software called a debugger. The most commonly used debugger for Rust and C is [GDB](https://en.wikipedia.org/wiki/GNU_Debugger).

*Remark: In VS Code, you can install the `probe-rs-debug` extension to use the `probe-rs` toolkit for debugging. It uses a different protocol than `gdb`. See [instructions](https://probe.rs/docs/tools/debugger/).*

### Setup of `cargo-embed`

Adjust the `Embed.toml` file in the root of this repository if necessary. This file configures the behavior of the `cargo embed` command when run on your laptop.

For example, if the configuration contains the following, a GDB debug server session will be started, and the loaded program will be reset to the first instruction.

```toml
[default.gdb]
enabled = true

[default.reset]
halt_afterwards = true
```

Prevent lines from being merged or reordered during the build step of your program. These kinds of changes can make it harder for the debugger to stop at the right breakpoints. Add the following to `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```

To be sure the new configuration is used, you can clear the `target` build cache with `cargo clean` and then build again:

```bash
cargo clean
cargo build --example [BINARY_EXAMPLE_NAME]
```

### Starting a GDB Client

While searching for an appropriate GDB package, look for one that supports the architecture of your target chip. In the case of a Pico 2, `gdb` needs `ARM` support built in.

Install the multi-architecture version of `gdb`:

```bash
sudo apt-get install gdb-multiarch
```

Then run the following command to create and connect a `gdb` debugging client:

```bash
gdb-multiarch target/thumbv8m.main-none-eabi/debug/[BINARY_EXAMPLE_NAME]
```

*Note: The command may also be `gdb`.*

Within the `gdb` client on your laptop, you have to connect to the running `GDB` server on the debug Pico probe:

```gdb
target remote :1337
monitor reset halt # optionally resets to the first instruction
tui enable
```

Alternatively, you can tell `gdb` to execute these commands automatically by writing them in a `.gdbinit` file in the root of this repository.

### Common GDB Commands

Breakpoints can be set in the `gdb` client by using the `break` command followed by a line number or function name:

```gdb
break [FUNCTION_NAME]  # Set a breakpoint at a specific function
break [LINE_NUMBER]  # Set a breakpoint at a specific line number
break [FILE_NAME]:[LINE_NUMBER]  # Set a breakpoint at a specific line in a file
```

You can also write hardware breakpoints directly in your code with `cortex_m::asm::bkpt()`.

To progress through the execution of your debugged program, you can use:

```gdb
continue  # Continue execution until the next breakpoint is hit
next      # Step to the next line of code
```

For introspection of variables:

```gdb
print [VAR_NAME]
```

## Water Pump

After reverse-engineering the parameters for the moisture sensor, we can now use the data to control a water pump.

The water pump is a small 3V submersible pump controlled by a GPIO pin on the Raspberry Pi Pico. A transistor should be used as a switch between the GPIO pin and the pump.

***Important**: To protect the Pico, you should put a diode in the circuit with the pump to prevent current from flowing back into the Pico when the pump is turned off. This is called a flyback diode.*

*Remark: The transistor allows the water pump to be powered by a higher voltage source, such as a battery or an external power supply. However, in this project, we don't need that.*

You can see the pump in action by running the example:

```bash
cargo run --example water-pump
```

Notice in the [source code](./examples/water-pump.rs) that we are now using a static channel:

```rust
static HUMIDITY_PUBSUB_CHANNEL: PubSubChannel<CriticalSectionRawMutex, f32, 1, 3, 1> =
    PubSubChannel::new();
```

The generic parameter `<_, _, 1, 3, 1>` part means that the channel can cache one value, has a maximum of three subscribers, and one publisher. The `CriticalSectionRawMutex` is used to ensure that the channel can be accessed safely from multiple tasks.

This is certainly more verbose than Tokio's channels, but in an embedded context, you probably don't want to create many subscribers and publishers at runtime. Instead, you want to create them at compile time so that the code is more predictable and deterministic.

Static variables are like global variables. They should be initialized before the actual program runs. Since they "always" have a value, they can be used to communicate between different tasks in the program.

It is important to know that mutating static (global) variables is not allowed by default in Rust. This is because it may cause race conditions between different tasks mutating the static variable in parallel.

A channel usually comes in two parts: an input and an output. Let's take a look at the sending part of an [Embassy channel](https://docs.embassy.dev/embassy-sync/git/default/index.html):

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

The sending task is an async task because we are not interested in measurements that are closer than 500 milliseconds apart. The `Ticker` is used to wait for the next measurement interval and allow other async tasks to run.

The `publisher` is used to send the moisture value to the channel. The `publish_immediate` method is used to send the value immediately and drops any old values not yet consumed by a receiving task.

## Pulse Width Modulation (PWM)

The speed of the motor should not always be at its maximum. Instead, you might want to turn up the motor speed gradually. By default, GPIO pins output the highest voltage (3.3V) when set to high.

PWM can adjust the *average* output voltage of a GPIO pin by rapidly switching it on and off. The ratio of the time the pin is high to the time it is low is called the **duty cycle**. A higher duty cycle means a higher average voltage.

You can try it out by running the example:

```bash
cargo run --example calibrate-speed-water-pump
```

This exercise will allow you to manually set the speed of the water pump by typing a number in the serial monitor.

> **Exercise**: Write a function that parses the bytes coming in over serial connection into moisture numbers.

Next, you should try to adjust the speed of the water pump based on the received intensity numbers.

* Listen for new numbers coming in over the serial connection.
* Parse the numbers and convert them to a speed value.
* Send the speed value through the sender of a `PubSubChannel` to another task.
* Receive the speed value in the task that controls the water pump.
* Compute the duty cycle based on the speed value and set the PWM output accordingly.

> **Exercise**: Use the incoming numbers over serial USB to change the speed of the water pump dynamiccally at runtime.

The Pico board also has multiple PIO peripherals. This is a programmable input/output peripheral that can be used to implement custom protocols and control devices.

Creating a PWM output with the PIO peripheral requires more work, but may be more performant than using simpler ways to drive PWM outputs. See <https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/pio_pwm.rs>

## On-board blink

Strangely, the GPIO pin 25 has been re-assigned to the on-board WiFi chip. This means you need to initialize the Wifi chip before you can use the on-board LED. I have hidden most of the dirty work in the `src/wifi.rs` file. You can have a look at it, but you don't need to understand it completely.

To just blink the on-board LED, you can run the following command:

```bash
cargo run --example on-board-blink
```

## HTTP notifications

The setup of wireless communication in Rust is more difficult than in MicroPython. On the other hand, it may be more powerful and flexible.

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
PASSWORD = "?" # WiFi password
SSID = "?" # WiFi SSID
```

After filling in the secrets (don't commit them to GitHub), you can try out a program that will send notifications regularly to the Ntfy service. If you subscribe to the associated channel / topic, you can receive them on your phone or laptop.

```bash
cargo run --example http-notifications
```

> **Exercise**: Make the messages emitted to `ntfy` by the Pico prettier or more informative (e. g. containing some numerical data).

## Levels of abstraction in embedded Rust

This section provides an overview of the different levels of abstraction that can be used when programming microcontrollers in Rust (or other languages).

### Low level

The lowest level of abstraction for software running on a microcontroller, is the MCU. The MCU enables access to the core processor. See [Cortex-M](https://crates.io/crates/cortex-m).

On top of the MCU, there always is a "peripheral access crate" (the PAC). This crate contains code generated from SVD files provided by the board manifacturer. See the [RP235X-PAC](https://crates.io/crates/rp235x-pac)

The Embassy framework builts on top of the PAC to provide a more intuitive / convenient API for accessing the hardware.

### Medium level

In case you feel like the Embassy framework does not allow you do certaint things, you can fall-back to a more convential level of abstraction, without async/await.

The "hardware access layer" (HAL) is a more convenient way to access the hardware of the microcontroller. It provides a higher level of abstraction than the PAC, but still allows you to access the hardware directly.

The Pico 2 has [rp235x-hal](https://crates.io/crates/rp235x-hal) as a HAL crate. You can view the [examples](https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples), which were used to make this workshop.

*Remark: If you want to be able to **kill async tasks**, you should not use Embassy, but instead use [RTIC](https://github.com/rtic-rs/rtic) which allow pre-emptive killing of running tasks. You can also assign priorities to different tasks, which may be required for sensitive applications. However, it is not yet stable.*

### High level

Normally, for commonly used micro-controllers, there should at least be one good board support package (also called BSP). These so-called packages are actually creates that have a very generic API, but less customisable. For example, in the case of the Microbit controller, the BSP is called [microbit](https://crates.io/crates/microbit) and it allows you draw visual shapes on the on-board LED array.

For the Raspberry Pico 2 W, `embassy` (and the plugin `embassy-rp`) come the closest to a real BSP.

## More reading material

Interesting books about embedded Rust:

* There is a book for beginners in embedded Rust: [Rust Discovery Embedded book](https://docs.rust-embedded.org/discovery-mb2/). It assumes you have bought a Microbit v2 (20 euros).
* There is also a book about embedded Rust using an STM32 chip: [Embedded Rust book](https://docs.rust-embedded.org/book/).
* Another book about Rust and the Pico 2 [Pico Pico](https://pico.implrust.com)
