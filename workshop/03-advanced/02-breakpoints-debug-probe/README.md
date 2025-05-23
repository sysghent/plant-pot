# Debugging with RTT over SWD

It is possible to configure a second Raspberry Pi Pico board as a hardware debug probe.

## Preparations

### Flash a `picoprobe`

Download from <https://github.com/raspberrypi/debugprobe/releases/tag/debugprobe-v2.2.2>

Flash the `picoprobe.uf2` file to the second Raspberry Pi Pico board.

Connect the newly flashed `picoprobe` board to your development Pico board. Instructions can be found [here](https://mcuoneclipse.com/2022/09/17/picoprobe-using-the-raspberry-pi-pico-as-debug-probe/).

### Install `probe-rs`

Install `probe-rs` on your development machine. (You may need to install a more recent version if break-points are not recognised.) Add udev rules to be able to use `probe-rs` without `sudo` as mentioned [here](https://probe.rs/docs/getting-started/probe-setup/).

If your udev rules are not being used, verify that the `udev` rules mention the right IDs of the hardware debug probe.

### Editor debug extensions

TODO: Add alternative non-VSCode instructions.

In VS code you can install the `probe-rs-debug` extension to use the `probe-rs` debugger. This extension is not required, but it makes debugging easier.

### Configure debug extension

Update the path to the binary in the file `.vscode/launch.json` (at the top of this repository). Install the `` extension.

## Actual debugging

### Compile, flash and run

Enable the `probe-rs` runner in the `.cargo/config.toml` file. This will set the runner to the flash functionality by the `probe-rs` tool. Then just run:

```bash
cargo run
```

This tool can flash the binary over SWD (Serial Wire Debug) to the target board.

### Add breakpoints

Add breakpoints in your code.

- In VSCode, click on the margin next to the line number where you want to add a breakpoint. A red dot should appear.
- You can also add hardware breakpoints that stop any debugger on this line: `cortex_m::asm::bkpt()`

### Launch debug session

Select debug session in the top left corner of VSCode and click on the green arrow to start debugging.

On the left pane you can inspect the registers, variable values on stack while waiting on a breakpoint.

### Tips

If you encounter problems, make sure: that the Cortex CPU is not sleeping. To solve this, produce artificial wake-ups with `yield_now()`.

Prevent your source code from being optimised away. This may not be necessary, but it can help with debugging. Add the following to your `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```

Reset the build cache to make sure the new settings are used:

```bash
cargo clean
```

### Defmt RTT output

Now that you have a hardware debug probe, you can use the `defmt` crate to send RTT output to your host machine. This is a more efficient way to send debug information than using serial output over USB.

Just import macro's such as `defmt::info!`, `defmt::error!`, and `defmt::warn!` to log messages, similar to how you would use `log::info!` in standard Rust.
