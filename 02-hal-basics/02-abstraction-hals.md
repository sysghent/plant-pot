# Abstraction levels in embedded Rust

Writing programs for microcontrollers can be done at different levels of abstraction.

## Low level

The Embassy implementation is based on the following low-level crates:

- MCU: CPU core driver <https://crates.io/crates/cortex-m>. You would normally not use this directly, and only when writing drivers for new hardware.
- PAC: peripheral access crate  (automatically generated from svd-files) <https://crates.io/crates/rp235x-pac>. You would not use this directly, unless you really need hardware-specific definitions or enums.

## Medium level

Instead of the low-level crates, you would normally use the following medium-level crates:

- HAL: hardware access layer <https://crates.io/crates/rp235x-hal>. Examples: <https://github.com/rp-rs/rp-hal/tree/main/rp235x-hal-examples>

There are also asynchronous HALs available:

- Embassy: co-operative async <https://crates.io/crates/embassy>
- RTIC: pre-emptive async  <https://github.com/rtic-rs/rtic>

Even on medium level, you still need to specify a `panic_handler`. Or you can just use an existing definition:

- `panic-probe` (for apps with `defmt` logging)
- `panic-halt` (for normal HAL usage)

## High level

Normally, for commonly used micro-controllers, there should at least be one good board support package (also called BSP).

Unfortunately, for the Raspberry Pico 2 W, there is no good BSP that hides the complexity of the device. There is however a very thin layer on top of the HAL: [rp-pico](https://crates.io/crates/rp-pico).

## More reading material

Hands-on embedded Rust books:

- There is a book for beginners in embedded Rust:  [Rust Discovery Embedded book](https://docs.rust-embedded.org/discovery-mb2/). It assumes you have bought a Microbit v2 (20 euros).
- There is also a book about embedded Rust using an STM32 chip: [Embedded Rust book](https://docs.rust-embedded.org/book/).
- Another book about Rust and the Pico 2 [Pico Pico](https://pico.implrust.com)
