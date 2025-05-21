
## Flashing Pico without hardware debug probe

If you don't have an extra hardware debug probe, you can still flash the Pico using the following steps.



### Mounting the Pico

Steps for Pico without "Raspberry Pi hardware debug probe":

1. Unplug the Pico.
2. Hold the "BOOTSEL" button on the pico.
3. Connect it over USB to your computer.
4. A pop-up should appear saying a storage device was connected.
5. Mount the storage device (if necesssary).


### Patching binary for Pico

If you have already played around with Raspberry Pico, you might have `picotool` installed already. This tool can be used to prepare the Rust ELF binary for flashing.


If you haven't, you can  install the `elf2uf2-rs` tool. This tool will also convert the ELF to UF2 but flash it directly afterwards. Install it with:

```bash
cargo install elf2uf2-rs
```

### Flashing Pico

If your `.cargo/config.toml` file is set up correctly, you can use the following command to compile and flash the Pico in one step:


```bash
cargo run
```

