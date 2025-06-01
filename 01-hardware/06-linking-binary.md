# Required compiler setup

Inside the [`.cargo/config.toml`](https://doc.rust-lang.org/cargo/reference/config.html) file, we have to specify compiler flags.

## Linker flags

Explanation of the `rustflag` options:

- linker argument `--nmagic` turns off page alignment of sections (which saves flash space)
- linker argument `-T link.x` tells the linker to use `link.x` as a linker script.  This is usually provided by the cortex-m-rt crate, and by default the  version in that crate will include a file called `memory.x` which describes the particular memory layout for your specific chip.
- linker argument `-Tdefmt.x` also tells the linker to use `defmt.x` as a  secondary linker script. This is required to make defmt_rtt work.
