# Step 1 - Build

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

## Common initialisation steps

The basic, required dependencies are already listed in the top-level (workspace-level) `Cargo.toml` file.

For solving the exercises, you might need to add additional dependencies.

Examples can be found in the `examples` folder of the Embassy project: <https://github.com/embassy-rs/embassy/tree/main/examples/rp235x/src/bin>.
