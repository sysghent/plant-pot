# Step 1 - Blink

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

## Initializing the board

Create peripherals.

**Task**: turn on the onboard LED (and loop forever).
