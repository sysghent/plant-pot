# Switch between RISC-V and ARM

The Pico 2 W has four cores: two RISC-V cores and two ARM cores. Only one type can be active at a time during runtime.

You can decide which type you use at compile time by specifying the target architecture.

```bash
cargo build --target=thumbv8m.main-none-eabihf # For ARM Cortex-M cores
cargo build --target=riscv32imac-unknown-none-elf # For RISC-V cores
```

You can also switch at run-time between the RISC-V and ARM cores by using the `arch_flip` example (and rebooting in the desired cores).

<https://github.com/rp-rs/rp-hal/blob/main/rp235x-hal-examples/src/bin/arch_flip.rs>
