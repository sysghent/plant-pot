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

## Start the first exercise

Go to the first step in [01_build](./01_build/README.md)

## Launching a debug session

Use `probe-rs`.

## Advanced topics

[Advanced topics](../docs/SOFTWARE_ADVANCED.md) are optional and can be skipped if you are not interested in them.
