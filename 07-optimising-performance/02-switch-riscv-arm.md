# Switch between RISC-V and ARM

The Pico 2 W has four cores: two RISC-V cores and two ARM cores. Only one type can be active at a time during runtime.

For flashing to the Pico 2 W and booting the Pico in its RISC-V mode:

```bash
cargo run --target=riscv32imac-unknown-none-elf # For RISC-V cores
```

Or use the provided alias in the `.cargo/config.toml` file:

```bash
cargo run-riscv
```

TODO: I have not experiment with the RISC-V cores yet, so I don't know if this works. If you have a Pico 2 W, please try it out and let me know if it works.

You can also reboot at run-time into either both RISC-V or both ARM cores as shown in the [`arch_flip` example](https://github.com/rp-rs/rp-hal/blob/main/rp235x-hal-examples/src/bin/arch_flip.rs).
