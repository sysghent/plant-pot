# Print-debugging with RTT

The `RTT` logging protocol does not require you to configure a serial connection (including baud rate, etc.).

## Add "log" statements

Import the `defmt` crate to add log statemens throughout your code. Just import macro's such as `defmt::info!`, `defmt::error!`, and `defmt::warn!` to log messages, similar to how you would use `log::info!` in standard Rust.

Then you have to enable a "transport" for `defmt` which is usally `RTT`, implemented by linking your code with the `defmt-rtt` crate.

- Add `defmt-rtt` to your `Cargo.toml` file:
- Add a compiler flag to your `.cargo/config.toml` file: `-C link-arg=-Tdefmt.x`.

## Connecting an RTT client

If you want to connect an `RTT` client on your laptop to the Pico, right after flashing, you can  with `embed`:

```bash
cargo embed
```

In terminal, a monitor should open, showing the output of the `defmt` logging commands on the connected Pico board.

To exit the RTT output monitor, press `CTRL+C`.
