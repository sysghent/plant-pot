# Creating a debug probe

It is possible to configure a second Raspberry Pi Pico board as a hardware debug probe.

## Flashing the probe

Download flash image for the hardware debugger from the official [`picprobe` releases](https://github.com/raspberrypi/debugprobe/releases).

In this workshop, we are only using Pico 2 W boards, so download the file `debugprobe_on_pico2.uf2`. Connect the probe that you want to convert into a debug probe in BOOTSEL mode over USB with your laptop. Drop the downloaded `uf2` file on to the mounter Pico drive. This will flash the `picoprobe` firmware onto the Pico board.

_**Warning**: Disconnect the USB cable from the probe to prevent hardware damage, until you have completed the wiring._

## Connecting the probe

Let's say D is the debug Pico 2 W and T is the target Pico 2 W.

1. Take the side with the white socket (JST PH 3-pin) of the  "JST PH 3-Pin to Male Header Cable". Plug it into the SWD socket of the target Pico 2 W (the one you want to debug).
2. Turn both devices with USB ports facing upwards (to prevent confusion).
3. The three male header pins should be connected to the debug probe Pico 2 W as follows:
   - Pin 1 (left, yellow) -> T p5
   - Pin 2 (middle, black) -> T p3
   - Pin 3 (right, orange) -> T p4

Instead of pin number, you can als use the pin names:

T SWCLK <-> D GP3
T SWDIO <-> D GP2
T GND <-> D GND

Provide power to the target Pico 2 W (through the debug probe) by connecting the following pins:

- T p38 <-> D p38 (connect ground)
- T p39 <-> D p39 (connect power supply)

TODO: You might still connect the both for UART communication.

When you are done with the wiring in between target and debug probe, you can connect the debug probe to your laptop.

## Flashing the target

Look up the codename for the model/architecture of target board. For Pico 1, this is `RP235x`. For Pico 2, this is `RP235x`.

Adjust the [`.cargo/config.toml`] file if necessary. For example, if you are using a Pico 2 W as a target, you can use the following configuration (below the section for your current build target):

```toml
[target.thumbv8m.main-none-eabi]
runner = "probe-rs run --chip RP235x"
```

**Note**: The chip name is not the name of the probe, but the name of the target chip you want to flash.

This allows you to flash to a Pico without having to disconnect it, pressing the BOOTSEL button, and then reconnecting it.

Just run:

```bash
cargo run
```

You can also set up a configuration file `Embed.toml` that contains the chip type and certain options (like whether to activate a debugging session with GDB). You can then use `cargo embed` to flash the binary and start a GDB session.

```toml
[target.thumbv8m.main-none-eabi]
runner = "cargo embed"
```
