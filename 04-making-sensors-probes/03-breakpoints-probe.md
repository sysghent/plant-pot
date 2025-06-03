# Debugging with breakpoints

Previously, you saw how to use `RTT` to print debug messages to the console. This is useful, but it is not a real debugger. In this exercise, you will learn how to use a real software debugger with breakpoints and variable inspection.

## Stripping debug information

Prevent lines being merged or re-ordered during the process of optimisation. This process can make it harder for the debugger to stop at the right breakpoints. Add the following to your `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```

Reset the build cache to make sure the new settings are used:

```bash
cargo clean
```

If you cannot stop at breakpoints, make sure: that the Cortex CPU is not sleeping. To solve this, you can try to produce regular artificial wake-ups with `yield_now()`.

## Debugging with `gdb` (command-line)

Build whatever binary target you want to debug.

```bash
cargo embed
```

Flash the binary to the Raspberry Pi Pico board.

Install the multi-architecture version of `gdb` to be able to debug the Raspberry Pi Pico board:

```bash
sudo apt-get install gdb-multiarch
```

The exact binary name may vary, but it is important that the installed `gdb` supports the architecture of your target chip. In the case of a Pico 2, `gdb` needs `ARM` support built-in.

Then run the following command to create and connect a `gdb` debugging client:

```bash
gdb-multiarch target/thumbv8m-none-eabi/debug/your_binary_name
```

Within the `gdb` client, you have to connect to the running `RTT` server on the Pico:

```gdb
target remote :1337
break [LINE_NUMBER]  # Set a breakpoint at a specific line number
continue  # Continue execution until the breakpoint is hit
```

If your debugging session loops forever, you might have jumped too far in the program before the program was halted at a breakpoint. In that case you have to reset the program with:

```gdb
monitor reset halt
```

## VS Code

In VS code you can install the `probe-rs-debug` extension to use the `probe-rs` debugger. This extension is not required, but it makes debugging easier.

Update the path to the binary in the file `.vscode/launch.json` (at the top of this repository). Install the `` extension.

### Add breakpoints

Add breakpoints in your code.

- In VSCode, click on the margin next to the line number where you want to add a breakpoint. A red dot should appear.
- You can also add hardware breakpoints that stop any debugger on this line: `cortex_m::asm::bkpt()`

### Launch debug session

Select debug session in the top left corner of VSCode and click on the green arrow to start debugging.

On the left pane you can inspect the registers, variable values on stack while waiting on a breakpoint.
