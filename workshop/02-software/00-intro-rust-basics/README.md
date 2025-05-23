# Simple Rust example

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
