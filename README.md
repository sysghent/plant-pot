# Workshop: make a smart plant pot

Notes for a workshop organised by Hugo & Willem in Ghent on 4th of June 2025. Register on [Mobilizon](https://mobilizon.be/events/3babf471-434d-431c-972c-b0bbae57b64c).

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the humidity of the earth in the pot is too dry.

## Overview

### Prerequisites

Please bring:

- **Laptop**
- **Micro-USB cable**
- **Water container** (e.g. a cup of water)

### Provided

You can borrow/buy from us (or bring your own):

- [Raspberry Pico 2 W](https://datasheets.rapberrypi.com/picow/pico-2-w-datasheet.pdf): ~ 10 €
- Analogue capacitive humidity sensor: ~ 5 €
- JST SH 1mm Pitch 3 Pin to Male: 2.50 €
- 3V submersible water pump ~ 5 €
- Breadboard: ~ 5 €
- Jumper wires ~ 1 €
- LED ~ 0.5 €
- MOSFET transistor ~ 0.5 €
- resistor ~ 0.5 €
- soldering iron ~ 30 €

You can buy the hardware from us at the end of the workshop.

### Homework

After the workshop, you should be able to continue and finalise the project at home. You will need to:

- Buy a plant: 10 €
- Provide a waterproof case for the electronics: 2 €
- Provide battery power: 4 €

## Preparations

First, you have to configure your development environment on your laptop to be able to run Rust code on the Raspberry Pico 2 W.

1. Install [`rustup`](https://www.rust-lang.org/tools/install).

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    On certain operating systems, the `rustup` package is already available in the package manager. For example, on Debian-based systems, you can install it with:

    ```bash
    sudo apt install rustup
    rustup install stable --profile default
    ```

2. Verify that `cargo` and `rustc` are available in your shell's `PATH`:

    ```bash
    cargo --version
    rustc --version
    ```

3. Install compiler components for future cross-compilation:

    ```bash
    rustup target add thumbv8m.main-none-eabihf
    ```

4. Install `picotool` to flash the Raspberry Pico.

    ```bash
    sudo apt install picotool
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

## Hello world demo

This is the first time you have to use the breadboard. Look at the official Pico 2 W "pinout" SVG diagram provided in the [./cheatsheets](./cheatsheets) directory. It shows the different pins on the Pico 2 W and their functions.

_**Warning**: Disconnect the USB cable from the probe to prevent hardware damage, until you have completed the wiring._

Pick a pin where the voltage is high (or can be set high), close to a ground pin. If you put an LED in between, it should light because of the current flowing from the high to the ground pin.

To prevent the LED from burning up or other accidents, it might be best to put a resistor in series (in front of behind) with the LED. The resistance of such a resistor can be computed as `R = (DV_out - DV_led) / I_max`.

> **Exercise**: Check whether it is necessary to put a resistor in series with the LED to limit the current. What would be the resistance of such a resistor in Ohm? Read the cheatsheet about resistor Ohm codes to know which resistor to pick.

Unplug your Pico and plug it back in while holding the "BOOTSEL" button. This will attach the Pico as a mass storage device.

Run the following command to run a basic "Hello World" program on the Pico:

```bash
cargo run --example on-board-blink
```

The mass storage device should no be automatically unmounted and the Pico should reboot into the program that you just flashed.

You should see the on-board LED blinking.

Before you dip your toes into the rest of the code in this workshop, it might helpful to read about the basics of the Rust programming language. A good starting point is the [Rust book](https://doc.rust-lang.org/book/).

More advanced books are:

- For an introduction, see [Programming Rust](https://www.amazon.com/Programming-Rust-Fast-Systems-Development/dp/1492052590)
- For a more in-depth tutorial in how async Rust works and coroutines, have a look at [Asynchronous programming in Rust](https://www.packtpub.com/en-mt/product/asynchronous-programming-in-rust-9781805128137).

**Exercise**: Make sure that you understand the concepts of `struct`s, `impl` blocks, and `async` functions in Rust.

To check if your code compiles and satisfies basic style conventions, you can run:

```bash
cargo clippy --examples
```

The Embassy framework, used in the project, provides an asynchronous executor that can be used to run many asynchronous tasks concurrently. Embassy tasks are run cooperatively: we assume they will give up (yield) control voluntarily to other tasks.

The Embassy project provides plugin crates for different support micro-controllers such as the Raspberry Pi Pico 2 W: [embassy-rp](https://crates.io/crates/embassy-rp). This crate is already added as a dependency to this project with the right configuration.

Let's take a closer look at the main loop in the `examples/on-board-blink.rs` file:

```rust
let mut ticker: Ticker = Ticker::every(Duration::from_millis(500));
loop {
    led_pin.set_high();
    ticker.next().await;
    led_pin.set_low();
    ticker.next().await;
}
```

> **Exercise**: Learn about the `Duration` type in Embassy. Modify the code in the main loop to make the LED blink faster or slower.

If you have used other asynchronous frameworks before, this concept may be familiar to you. The `Ticker` type can be compared to Tokio's `Interval`.

You might be thinking:

> "Do we really need an async framework for this?"

Below the surface, Embassy uses hardware timers and interrupts to put the CPU of the Pico to sleep while it is _waiting_. This is more efficient than busy-waiting, as is often done in Python (think: `while True:`).

## Measuring moisture

From now on, you need to have a moisture sensor connected to the Raspberry Pico. The moisture sensor has three pins: VCC, GND, and the signal pin.

- Connect the VCC pin to the 3.3V output pin on the Pico,
- the GND pin to a ground pin on the Pico,
- and the signal pin to one of the ADC-capable pins on the Pico. This is the part that provides information to the Pico about how much moisture is in the soil.

A microcontroller is not continuously powered, but turns millions of times per second (CPU clock cycles). This means that we cannot really have a continuous measurement of the moisture in the soil. Instead, it is a discrete measurement that we can take at regular intervals.

We have to use the ADC (Analog-to-Digital Converter) of the Raspberry Pico to measure the moisture in the soil. The ADC converts the analog signal from the moisture sensor into a digital value that can be processed by the microcontroller.

> **Exercise**: Find all the pins on the Pico that can be used as ADC inputs.

Once you have a good visual idea of this, do the next exercise.

> **Exercise**: How many bits are used by the ADC on the Raspberry Pico? How many different values can it measure?

Now you can flash an example program on your Pico. Remember from last time that you have to hold the "BOOTSEL" button while plugging in the Pico to mount it as a mass storage device.

```bash
cargo run --example calibrate-moisture-sensor
```

I created a simple program that can measure the moisture in the soil using ADC. The main loop has this code:

```rust
let level = adc.read(&mut humidity_pin).await.unwrap();
let voltage = adc_reading_to_voltage(level);
let humidity = voltage_to_humidity(voltage);
```

Notice that we need a handle to the ADC component and also a pin configured as an ADC channel.

> **Exercise**: Fill in the `todo!` macro-calls inside the bodies of the conversion functions `adc_reading_to_voltage` and `voltage_to_humidity`. Hint: these functions are similar to the `map` function in ArduinoIDE.

Maybe more difficult:

> **Exercise**: In which situations would need to sample multiple ADC values in on read?

## Serial IO

For this section, you can detach all attached wires and components from the Raspberry Pico. You will not need them for the next exercises.

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

4. Connect to the Pico from your laptop using a virtual serial connection that runs over USB:

    ```bash
    tio /dev/serial/by-id/usb-c0de_USB-serial_example-if00
    ```

6. If you are not able to connect, you can try different parameters for the serial connection or a different device path.

    ```bash
    tio -s 1 -d 8 -p none -b 9600 /dev/ttyACM1
    ```

From now on, you can send bytes to the Pico and also receive bytes from the Pico. All keyboard commands for `tio` are listed in [GitHub](https://github.com/tio/tio#32-key-commands). The most important one is `ctrl-t q` to quit the serial monitor.

Try it out by running one of the official Embassy examples:

Every time you finish a line by pressing `Enter`, the Pico will echo back the line you typed. This is a good way to test if the serial connection is working correctly.

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

## HTTP notifications

The setup of HTTP communication in Rust is more difficult than in MicroPython. On the other hand, it is more powerful and flexible.

1. Make an account on [Ntfy](https://docs.ntfy.sh).

2. Install the mobile Ntfy app on your phone (optional) or use another platform to receive notifications.

3. Try publishing a notification from you command line using `curl`:

    ```bash
    curl -X POST https://ntfy.sh/sysghent -d "$USER will water the plants!"
    ```

Instead of using `curl` you can also use your Pico to send notifications.

```bash
cargo run --example http-notifications
```

> **Exercise**: Make the messages emitted to `ntfy` by the Pico prettier or more informative (e. g. containing some numerical data).

## Using a hardware debugger

We will configure one Pico to act as a hardware debugging probe for another Pico, using the SWD protocol. This allows you to debug the code running on the target Pico using GDB or other debugging tools.

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

You can now flash the target Pico 2 W by running:

```bash
cargo embed --example on-board-blink
```

With this command, it is not necessary anymore to hold the "BOOTSEL" button while plugging in the target Pico 2 W. The `cargo embed` command will automatically flash the program to the target Pico 2 W.

Configure the `cargo embed` command as a runner in your `.cargo/config.toml` file, so you can run it with `cargo run`:

```toml
[target.thumbv8m.main-none-eabi]
runner = "cargo embed --chip RP235x"
```

Now you can just use a shorter command (and prevent problems caused by  re-plugging) to compile and flash in one step:

```bash
cargo run --example on-board-blink
```

## Using RTT for logging

RTT (Real-Time Transfer) is a logging protocol that can be used on top of an SWD connection.

Import the `defmt` crate to add log statemens throughout your code. Just import macro's such as `defmt::info!`, `defmt::error!`, and `defmt::warn!` to log messages, similar to how you would use `log::info!` in standard Rust.

Then you have to enable a "transport" for `defmt` which is usally `RTT`, implemented by linking your code with the `defmt-rtt` crate. `defmt-rtt` is the analogue of the

- Add `defmt-rtt` to your `Cargo.toml` file:
- Add a compiler flag to your `.cargo/config.toml` file: `-C link-arg=-Tdefmt.x`.

## Breakpoints

First, build your binary. Prevent lines being merged or re-ordered during the process of optimisation. This process can make it harder for the debugger to stop at the right breakpoints. Add the following to your `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```

To be sure the new configuration is used, you can reset the `target` build cache:

```bash
cargo clean
cargo build --example [BINARY_EXAMPLE_NAME]
```

Install the multi-architecture version of `gdb` to be able to debug the Raspberry Pi Pico board:

```bash
sudo apt-get install gdb-multiarch
```

The exact binary name may vary, but it is important that the installed `gdb` supports the architecture of your target chip. In the case of a Pico 2, `gdb` needs `ARM` support built-in.

Then run the following command to create and connect a `gdb` debugging client:

```bash
gdb-multiarch target/thumbv8m-none-eabi/debug/[BINARY_EXAMPLE_NAME]
```

Within the `gdb` client on your laptop, you have to connect to the running `GDB` server on the debug Pico:

```gdb
target remote :1337
monitor reset halt # optionally resets to the first instruction
```

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

_Remark: In VS Code, you can install the `probe-rs-debug` extension to use the `probe-rs` debugger._
