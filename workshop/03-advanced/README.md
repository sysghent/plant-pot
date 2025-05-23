# Advanced aspects of flashing the Pico

## Levels of abstraction (optional)

Writing programs for microcontrollers can be done at different levels of abstraction.

### Low level

The Embassy implementation is based on the following low-level crates:

- MCU: CPU core driver <https://crates.io/crates/cortex-m>
- PAC: peripheral access crate  (usually generated from svd-files) <https://crates.io/crates/rp235x-pac>

### High level

There slightly higher level abstractions available, which are more user friendly.

- HAL: hardware access layer <https://crates.io/crates/rp235x-hal>
- BSP: board support packages
- Embassy: co-operative async <https://crates.io/crates/embassy>
- RTIC: pre-emptive async  <https://github.com/rtic-rs/rtic>
