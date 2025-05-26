# Workshop: make a smart plant pot

(Notes for a workshop organised by Hugo & Willem in Ghent on 4th of June 2025.)

In this workshop you will learn how to create a plant pot that can automatically add water to itself when the humidity of the earth in the pot is too dry.

## Requisites

Please bring:

- A laptop (preferably with `rustup` installed and Linux)
- A USB-C cable
- A water container (e.g. a cup of water)

Nice, but not required, read:

- Embassy book <https://embassy.dev/book/#_for_beginners>
- Rust book <https://doc.rust-lang.org/book>

## Provided

You can borrow/buy from us (or bring your own):

- Raspberry Pico 2 W (if you bring your own board, pick one with [Embassy support](https://embassy.dev/book/#_getting_a_board_with_examples))
- Analogue capacitive humidity sensor
- 5V water pump
- Breadboard
- Jumper wires
- LED
- MOSFET transistor
- resistor
- Water tubes (TODO: buy in Brico)

You can buy the hardware from us at the end of the workshop.

To follow along, you can follow the structure of the folder [`Workshop`](./workshop/README.md).

## Homework

After the workshop, you will still need to:

- Buy a plant
- Provide a plastic cover for the electronics
- Provide battery power

## Install system dependencies

If you are using Linux, you need some system dependencies before you can follow along with the exercises in this workshop:

- The Rust compiler toolchain manager `rustup`: provided by your distribution and contains `cargo`.
- A small utility to prepary compiled binaries for flashing   `elf2uf2-rs`: installed through `cargo install elf2uf2-rs` (don't forget to add `~/.cargo/bin` to your `PATH`).

_Remark: Later on you may use `probe-rs` instead of `elf2uf2-rs` to flash the binary. It is also possible to use `picotool` if you happen to have it installed already._

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
