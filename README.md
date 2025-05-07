# Smart plant pot

_(Notes for a workshop organised by Hugo & Willem in Ghent in June 2025.)_

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the humidity is too dry.

You can borrow from us:

- Raspberry Pico H (with pre-soldered headers)
- Analogue capacitive humidity sensor
- Small 5V water pump
- Small breadboard
- Jumper wires

You can buy components at the end for a small fee.

Please bring a plant in a pot (or use a glass of water). 

## Raspberry Pico

Datasheet: https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf

## Install toolchain for Pico

Install the Rust compiler components

```bash
rustup install stable-x86_64-unknown-linux-gnu
rustup component add rust-analyzer
rustup target add thumbv6m-none-eabi
```

Setup the linker configuration in `.cargo/Config.toml`. It is important to have a `memory.x` file in the root for linking.


## Levels of abstraction

From low to high.

Processor: 
- cortex-m https://crates.io/crates/cortex-m

Peripheral access crate (low level):
- https://crates.io/crates/rp2040-pac (generated from svd-files)


Hardware access layer (medium level): 
- https://crates.io/crates/rp2040-hal/0.10.2
- embassy-rp https://crates.io/crates/embassy-rp

Board support packages (high level):
-  https://crates.io/crates/rp-pico (actually not that high-level), examples copied in this repo

## Templates

You can get started using one of the templates in this repository.

## Build

Build with:

```bash
cargo build -p package-name
```

If you want to be explicit:

```bash
cargo build --target thumbv6m-none-eabi -p package
```

This will produce an ELF binary (without extension) under [target/thumbv6m-none-eabi/debug](./target/thumbv6m-none-eabidy).

## Mounting the Pico

Steps:

1. Unplug the Pico.
2. Hold the "BOOTSEL" button on the pico.
3. Connect it over USB to your computer.
4. A pop-up should appear saying a storage device was connected.
5. Mount the storage device (if necesssary).


## Modifying binary

If you have already played around with Raspberry Pico, you might have `picotool` installed already. This tool can be used to prepare the Rust ELF binary for flashing.


If you haven't, you can  install the `elf2uf2-rs` tool. This tool will also convert the ELF to UF2 but flash it directly afterwards. Install it with:

```bash
cargo install elf2uf2-rs
```

## Flashing

If you prefer a manual approach, use the target `thumbv6m-none-eabi` and runner `elf2uf2-rs` as specified in the `.cargo/config.toml` file.

```bash
elf2uf2-rs -d target/thumbv6m-none-eabi/debug/embassy
```
If you don't want to type that much, you can use the same `.cargo/config.toml` file as we do (with a pre-configured runner):

```bash
cargo run
```


## Further optimisations

Tools to analyse binary:

```bash
cargo install cargo-binutils
rustup component add llvm-tools
```

https://github.com/rust-embedded/cargo-binutils


Tips to minimise binary size:

https://github.com/johnthagen/min-sized-rust
