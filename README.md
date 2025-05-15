# Workshop: make a smart plant pot

_(Notes for a workshop organised by Hugo & Willem in Ghent on 4th of June 2025.)_

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the humidity is too dry.

You can borrow from us:

- Raspberry Pico H (with pre-soldered headers)
- Analogue capacitive humidity sensor
- Small 5V water pump
- Small breadboard
- Jumper wires

You can buy components at the end for a small fee.

Please bring a plant in a pot (or use a glass of water). 


## Install toolchain for Pico (and other ARM Cortex-M CPUs)

Install the Rust compiler components

```bash
rustup install stable-x86_64-unknown-linux-gnu
rustup component add rust-analyzer
rustup target add thumbv6m-none-eabi
```



## Levels of abstraction

Writing programs for microcontrollers can be done at different levels of abstraction.

### Low level

Not used directly in this project.

MCU = Drivers for the pocessor core(s): 
- Pico: https://crates.io/crates/cortex-m

PAC = Peripheral access crate  (usually generated from svd-files)
- Pico: https://crates.io/crates/rp2040-pac

### High level

HAL = Hardware access layer: 
- Pico: https://crates.io/crates/rp2040-hal/0.10.2 (example in [hal](./hal))


BSP = Board support packages:
- Pico: https://crates.io/crates/rp-pico (example in [bsp](./bsp))


EMBASSY = Asynchronous HAL / BSP (high level, example in [embassy](./embassy)):
- Pico: https://crates.io/crates/embassy-rp


## Build

Verify that the right compiler options are set in `.cargo/config.toml` (a `memory.x` file is needed), then build with:

```bash
cargo build 
```

This will produce an ELF binary (without extension) under `target/thumbv6m-none-eabi/debug`.

## Special steps for Pico

Datasheet: https://datasheets.raspberrypi.com/pico/pico-datasheet.pdf

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
