
# Flashing Pico without hardware debug probe

If you don't have an extra hardware debug probe, you can still flash the Pico using the following steps.

## Mounting the Pico

Steps for Pico without "Raspberry Pi hardware debug probe":

1. Unplug the Pico.
2. Hold the "BOOTSEL" button on the pico.
3. Connect it over USB to your computer.
4. A pop-up should appear saying a storage device was connected.
5. Mount the storage device (if necesssary).

## Patching binary for Pico

If you have already played around with Raspberry Pico, you might have `picotool` installed already. This tool can be used to prepare the Rust ELF binary for flashing.

Otherwise, install the `elf2uf2-rs` tool to convert the ELF to UF2.

```bash
cargo install elf2uf2-rs
```

Then you can run the following command to convert the ELF binary to a UF2 binary:

```bash
elf2uf2-rs target/thumbv6m-none-eabi/debug/embassy-pico-blink
```

This will create a file called `embassy-pico-blink.uf2` in the current directory.

## Flashing Pico

Drop the `embassy-pico-blink.uf2` file on the mounted storage device. This will flash the Pico with the new firmware. The Pico will reboot automatically after flashing.

## Automate build, patch and flash

If your `.cargo/config.toml` file is set up correctly, you can use the following command to compile, patch and flash you `main.rs` executable on the Pico in one step:

```bash
cargo run
```

## Connect (when Pico runs USB serial)

On Linux:

```bash
sudo apt install tio
sudo tio /dev/ttyACM0
```

_Remark: If `ttyACM0` is not the right device, you can find the right device with: `sudo dmesg -W`. Connect the Pico in BOOTSEL mode. Observe the output of `dmesg` to know the name of the new serial connection._

Exit with CTRL-T Q.
