# Shrinking binary size

A good reading resource is the project [Min-sized Rust](https://github.com/johnthagen/min-sized-rust). Just try out some of the tips given there.

The most important tip is probably compiling with `--release` and using the `opt-level = "s"` option in the `Cargo.toml` file.:

```toml
[profile.release]
opt-level = "s"
```

This will optimize the binary size, but it will also make the code run slower.

_Remark: It shouldn't be necessary to remove `debug` information with `debug = 0`. See some articles on [blog](https://cliffle.com/)_
