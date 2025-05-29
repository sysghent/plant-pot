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
