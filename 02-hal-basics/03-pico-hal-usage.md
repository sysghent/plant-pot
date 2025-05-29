# Introduction Pico 2 HAL crate

The HAL crate is called `rp235x-hal` and provides a hardware abstraction layer for the Raspberry Pi Pico 2 W microcontroller.

When using the HAL crate, just start programs with

```rust
#![no_std]
#![no_main]
use rp235x_hal as hal;
use hal::{entry, pac, prelude::*};


#[entry]
fn main() -> ! {
    // Your main application code here
}
```

## Compilation

Compile the ARM Cortex-M version:

```bash
cargo build --target thumbv8m.main-none-eabihf
```

Compile the RISC-V version (include a declaration of a `DefaultIrqHandler`):

```bash
cargo build --target riscv32imac-unknown-none-elf
```
