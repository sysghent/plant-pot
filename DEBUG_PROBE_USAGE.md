# Using a debug probe



For using `probe-rs`, you need to add udev rules.



## Debugging with logging and breakpoints

Add to your `Cargo.toml`:

```toml
[profile.dev]
debug = 2
opt-level = 0
```
Reset the build cache after changes to `Cargo.toml`:

```bash
cargo clean
```

Make sure you build before starting the debugger:

```bash
cargo build
```


Update the path to the binary in `.vscode/launch.json`. Install the `probe-rs-debug` extension. 

Add breakpoints in your code.

- In VSCode, click on the margin
- Hardware breakpoints: `cortex_m::asm::bkpt()`