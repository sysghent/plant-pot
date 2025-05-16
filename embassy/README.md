# Async plant

An asynchronous version of the _smart plant pot watering system_.

Makes use of the [Embassy](https://embassy.dev/) framework.

Currently the two cores of the Pico are used. Two executors are created, one for each core.

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