# Simple Rust example

There are a few good resources to learn Rust. The best (and simplest) one is the [Rust book](https://doc.rust-lang.org/book/).

A working Rust environment can quickly be setup using `rustup`. Installing `rustup` also installs the `cargo` package manager.

On NixOS, you can start a shell with all the necessary dependencies with `nix-shell --pure`.

## Install Rust components

If you haven't done this before, you need to install the compiler toolchain for your laptop:

```bash
rustup install stable-x86_64-unknown-linux-gnu
```

You also need to install Rust compiler components to be able to cross-compile binaries on your laptop for the Raspberry Pico 2 W (which has a different CPU architecture than your laptop):

```bash
rustup target add thumbv8m.main-none-eabihf
```

If you want to use `risc-v` cores, you also need to install the `riscv32imac-unknown-none-elf` target:

```bash
rustup target add riscv32imac-unknown-none-elf
```
