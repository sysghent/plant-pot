
# Flashing through USB mass storage

## Mount

Follow these steps to mount the Raspberry Pi Pico as a mass storage device:

1. Unplug the Pico.
2. Hold the "BOOTSEL" button on the pico.
3. Connect it over USB to your computer.
4. A pop-up should appear saying a storage device was connected.
5. Mount the storage device (if necesssary).

## Patch binary

If you have already played around with Raspberry Pico, you might have `picotool` installed already. This tool can be used to prepare the Rust ELF binary for flashing.

Otherwise, install the `elf2uf2-rs` tool to convert the ELF to UF2.

```bash
cargo install elf2uf2-rs
```

Don't forget to add `~/.cargo/bin` to your `PATH`.

If you are using Linux, you need some system dependencies before you can follow along with the exercises in this workshop:

Then you can run the following command to convert the ELF binary to a UF2 binary:

```bash
elf2uf2-rs target/thumbv8m.main-none-eabihf/debug/embassy-pico-blink
```

This will create a file called `embassy-pico-blink.uf2` in the current directory.

## Drop on storage device

Drop the `embassy-pico-blink.uf2` file on the mounted storage device. This will flash the Pico with the new firmware. The Pico will reboot automatically after flashing.

_Remark (informational): If your `.cargo/config.toml` file is set up correctly, you can compile, patch and flash in one step: `cargo run`._

_Remark: Later on you may use `probe-rs` instead of `elf2uf2-rs` to flash the binary. It is also possible to use `picotool` if you happen to have it installed already._
