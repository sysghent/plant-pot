
# Flashing through USB mass storage

## Two ways to flash

The easiest way to get started fast is by turning the Pico into a USB mass storage device. This allows you to flash the firmware by simply copying a file onto the Pico. See the following section for details on how to do this.

In case you want to flash firmware often and don't want to re-plug the Pico every time, you can also use a JTAG or SWD programmer (together with `probe-rs`). We will see a way to do this later.

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

If you don't want to do this manually every time, you can configure the `elf2uf2-rs -d` as a runner in you `.cargo/config.toml` file:

```toml
[target.thumbv8m.main-none-eabihf]
runner = "elf2uf2-rs -d"
```

It is also possible to use `picotool` if you happen to have it installed already:

```toml
[target.thumbv8m.main-none-eabihf]
runner = "picotool load -u -v -x -t elf"
```

## No re-plugging

Later on you may use `probe-rs` instead of `elf2uf2-rs` to flash the binary. This binary crate provides a couple of tools that allow you to flash and debug the Raspberry Pico without having to re-plug it every time.

Once you have hardware debugger set up (see later on), you can configure the `probe-rs` runner in your `.cargo/config.toml` file:

```toml
[target.thumbv8m.main-none-eabihf]
runner = "probe-rs run --chip RP2350"
```

Or, in case you have `Embed.toml` set up, you can use the `cargo-embed` tool to flash the binary (and open a GDB session immediately):

```toml
[target.thumbv8m.main-none-eabihf]
runner = "cargo embed"
```
