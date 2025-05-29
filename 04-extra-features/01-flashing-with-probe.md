# Flash with hardware debug probe

## Official Raspberry Pi Pico Debug Probe

<https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html>

### Install `probe-rs` on laptop

Install `probe-rs` on your development machine. (You may need to install a more recent version if break-points are not recognised.)

```bash
cargo install probe-rs-tools --locked --force --git https://github.com/probe-rs/probe-rs --rev b2562d9b9bfba8fc6c690eff9d7cb565c777041d
```

 Add udev rules to be able to use `probe-rs` without `sudo` as mentioned in the [documentation](https://probe.rs/docs/getting-started/probe-setup/).

If your udev rules are not being used, verify that the `udev` rules mention the right IDs of the hardware debug probe.

### Configure `probe-rs` for the Pico

Edit the `Embed.toml` file to instruct the Pico to automatically run a `gdb` debugging server when running.

Flash the binary to the Pico (and run it) with [`cargo-flash`](https://probe.rs/docs/tools/cargo-flash/):

```bash
cargo flash
```

### Compile, flash and run

Enable the `probe-rs` runner in the `.cargo/config.toml` file. This will set the runner to the flash functionality by the `probe-rs` tool. Then just run:

```bash
cargo run
```

This tool can flash the binary over SWD (Serial Wire Debug) to the target board.

## Preparing code for RTT debugging

The `RTT` logging protocol does not require you to configure a serial connection (including baud rate, etc.).

### Add "log" statements

Import the `defmt` crate to add log statemens throughout your code. Just import macro's such as `defmt::info!`, `defmt::error!`, and `defmt::warn!` to log messages, similar to how you would use `log::info!` in standard Rust.

Then you have to enable a "transport" for `defmt` which is usally `RTT`, implemented by linking your code with the `defmt-rtt` crate.

- Add `defmt-rtt` to your `Cargo.toml` file:
- Add a compiler flag to your `.cargo/config.toml` file: `-C link-arg=-Tdefmt.x`.

### Connecting an RTT client

If you want to connect an `RTT` client on your laptop to the Pico, right after flashing, you can  with `embed`:

```bash
cargo embed
```

In terminal, a monitor should open, showing the output of the `defmt` logging commands on the connected Pico board.

To exit the RTT output monitor, press `CTRL+C`.
