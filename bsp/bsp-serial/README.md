# BSP - Serial over USB

This repository contains a setup for the Raspberry Pico that allows to communicate with it over USB and a serial interface.


## Flash

Mount the Pico and simply run

```bash
cargo run
```

## Connect

Launch `sudo dmesg | tail -f` in a terminal. Connect the Pico in BOOTSEL mode.

Observe the output of `dmesg` to know the name of the new serial connection.


On Linux:

```bash
sudo apt install minicom
sudo minicom -b 115200 -D /dev/ttyACM1
```


## Usage

Type some letters in the terminal and see how the flashed program transforms them.

Press CTRL-A Z X to quit.