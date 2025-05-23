# Software development

## System dependencies

On NixOS, you can start a shell with the necessary dependencies with:

```bash
nix-shell --pure
```

## Rust dependencies

Install the Rust compiler components

```bash
rustup install stable-x86_64-unknown-linux-gnu
rustup component add rust-analyzer

rustup target add thumbv8m.main-none-eabihf
```
  
## Basics of embassy

In this tutorial, we will use the Raspberry Pi Pico board with the Embassy framework.

The Embassy framework provides an asynchronous executor that can be used to run many asynchronous tasks concurrently. It is co-operative.

Different tasks can have different priorities by assigning them to executors on different cores manually.

_**Remark**: If you want more automatic scheduling, you need to use a different framework, such as RTIC that can suspend running tasks. Tasks will not be async._

The Embassy project provides a plugin that allows to use the specific hardware of a Raspberry Pi Pico through safe abstractions: <https://crates.io/crates/embassy-rp>.

## Follow exercises

This folder contains the exercises for the plant pot workshop. Each subfolder contains a separate exercise with its own README and starter code. Begin each exercise by reading the README in the corresponding subfolder.

The basic, required dependencies are already listed in the top-level (workspace-level) `Cargo.toml` file.

For solving the exercises, you might need to add additional dependencies.

Examples can be found in the `examples` folder of the Embassy project: <https://github.com/embassy-rs/embassy/tree/main/examples/rp235x/src/bin>.

- [01](./01/README.md)

## Launching a debug session

Use `probe-rs`.
