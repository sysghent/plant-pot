# Using a debug probe

It is possible to configure a second Raspberry Pi Pico board as a hardware debug probe.

## Flashing `picoprobe`

Download from <https://github.com/raspberrypi/debugprobe/releases/tag/debugprobe-v2.2.2>

TODO: provide a precompiled binary for the Pico 2.

## Install `probe-rs` on host

Install `probe-rs`. Add udev rules to be able to use `probe-rs` without `sudo`.

<https://probe.rs/docs/getting-started/probe-setup/>

If your udev rules are not being used, verify that the `udev` rules mention the right IDs of the hardware debug probe.

## Disable optimisations (optional)

Add to your `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```

Reset the build cache after changes to `Cargo.toml`:

```bash
cargo clean
```

## Configure debug session

Update the path to the binary in `.vscode/launch.json`. Install the `probe-rs-debug` extension.

## Add breakpoints

Add breakpoints in your code.

- In VSCode, click on the margin
- Hardware breakpoints: `cortex_m::asm::bkpt()`

## Launch debug session

Select debug session in the top left corner of VSCode and click on the green arrow to start debugging.

TODO: Add alternative non-VSCode instructions.

If you encounter problems, make sure:

- you have a recent `probe-rs` version installed.
- the Pico Cortex CPU is not sleeping (artificial wake-ups can be done with `yield_now()`).
