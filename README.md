# Smart plant pot

_(Notes for a workshop organised by Hugo & Willem in Ghent in June 2025.)_

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the humidity is too dry.

We will use hardware (est. 25 euros):

- Raspberry Pico H (with soldered headers)
- Analogue (?) capacitive humidity sensor
- Small 5V water pump
- Small breadboard
- Jumper wires
- Plant + pot (bring your own)

## Install toolchain for Pico

Install the Rust compiler components

```bash
rustup install stable-x86_64-unknown-linux-gnu
rustup component add rust-analyzer
rustup target add thumbv6m-none-eabi
```

Setup the linker configuration in `.cargo/Config.toml`. It is important to have a `memory.x` file in the root for linking.


## Levels of abstraction

Processor: 
- cortex-m https://crates.io/crates/cortex-m

Peripheral access crate (low level):
- https://crates.io/crates/rp2040-pac


Hardware access layer (medium level): 
- https://crates.io/crates/rp2040-hal/0.10.2
- embassy-rp https://crates.io/crates/embassy-rp

Board support packages (high level):
-  https://crates.io/crates/rp-pico

## Build

You can now build with:

```bash
cargo build 
```

This will produce an ELF binary (without extension) under [target/ thumbv6m-none-eabi](./target/thumbv6m-none-eabidy).

## Mounting the Pico

Hold the button on the pico while connecting it over USB until the device is detected by Linux.

Mount the storage device exposed by the Pico bootloader.

## Modifying binary

Either:

- use `picotool` to convert the ELF to UF2 and copy it manually or 
- install the `elf2uf2-rs` tool to convert the ELF to UF2 and flash it directly

In case you choose the last:

```bash
cargo install elf2uf2-rs
```

## Flashing

If you use the same `.cargo/Config.toml` file, you compile (if necessary), convert to UF2 and copy it to the Pico storage with:

```bash
cargo run
```

Otherwise, use a manual invocation of  `elf2uf2-rs -d` do this step.

