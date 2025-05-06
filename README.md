# Smart plant pot

_(Notes for a workshop organised by Hugo & Willem in Ghent in June 2025.)_

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the humidity is too dry.

We will use hardware (est. 25 euros):

- Raspberry Pico (with headers)
- Analogue (?) capacitive humidity sensor
- Small 5V water pump
- Breadboard
- Wires


## Install toolchain for Pico

Install the Rust compiler components

```bash
rustup install stable-x86_64-unknown-linux-gnu
rustup component add rust-analyzer
rustup target add thumbv6m-none-eabi
```

Setup the linker configuration in `.cargo/Config.toml`. It is important to have a `memory.x` file in the root for linking.

## Build

You can now build with:

```bash
cargo build 
```

This will produce an ELF binary (without extension) under [target/ thumbv6m-none-eabi](./target/thumbv6m-none-eabidy).

## Flash setup

Hold the button on the pico while connecting it over USB until the device is detect by Linux.

Mount the storage device exposed by the Pico bootloader.

To be able to flash, we need a utility to convert the ELF binary produced by the Rust compiler into a file that can be dropped on the mass storage exposed by the BOOTSEL bootloader mode of the Pico:

```bash
cargo install elf2uf2-rs
```

If you use the provided `.cargo/Config.toml` file, you can just run the following to flash directly on the Pico and reconnect to it in normal mode:

```bash
cargo run
```

This will implicitly call `elf2uf2-rs -d` to convert the ELF and copy it to the Pico storage.

