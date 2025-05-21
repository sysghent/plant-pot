# Plant pot with ESP32C6 with Embassy

This device is single core.


Hardware datasheet: https://www.espressif.com/sites/default/files/documentation/esp32-c6-wroom-1_wroom-1u_datasheet_en.pdf


Libraries specific to ESP32:

- https://docs.rs/esp-hal-embassy/0.6.0/esp_hal_embassy/
- https://docs.rs/embedded-hal-async/latest/embedded_hal_async/
- https://docs.esp-rs.org/esp-hal/esp-hal/0.23.1/esp32c6/esp_hal


Sample code: https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin

## Dependencies

Make sure all system dependencies are installed. On NixOS, you can just do:

```bash
nix-shell ../shell.nix
```

Or just get the latest `probe-rs` release:

```bash
cargo install probe-rs-tools
```


## Run

This will compile a debug build, prepare it, transfer (flash) it, run it.

```bash
cargo run
```


