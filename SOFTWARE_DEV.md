# Software development


## System dependencies

On NixOS, you can start a shell with the necessary dependencies with:

```bash
nix-shell
```


## Rust dependencies

Install the Rust compiler components

```bash
rustup install stable-x86_64-unknown-linux-gnu
rustup component add rust-analyzer
rustup target add thumbv6m-none-eabi
```


## Embedded software concepts

TODO:

- What is PIO, SIO?


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
- Pico: https://crates.io/crates/rp2040-hal/0.10.2


BSP = Board support packages:
- Pico: https://crates.io/crates/rp-pico


EMBASSY = Asynchronous HAL / BSP
- Pico: https://crates.io/crates/embassy-rp
- ESP32C6: https://docs.esp-rs.org/esp-hal/esp-hal/0.23.1/esp32c6/esp_hal




## Build / cross-compile 

Verify that the right compiler options are set in `.cargo/config.toml` (a `memory.x` file is needed), then build with:

```bash
cargo build 
```

This will produce an ELF binary (without extension) under `target/thumbv6m-none-eabi/debug`.



## Flashing on target

Depends on type of connection with target device.

See:

- [FLASH_PICO_UF2.md](FLASH_PICO_UF2.md) for flashing the Pico without a debug probe.
- [DEBUG_PROBE_USAGE.md](DEBUG_PROBE_USAGE.md) for flashing with a debug probe.



## Optimisation


Tools to analyse compiled embedded binaries:

```bash
cargo install cargo-binutils
rustup component add llvm-tools
```

https://github.com/rust-embedded/cargo-binutils


Tips to minimise binary size:

https://github.com/johnthagen/min-sized-rust