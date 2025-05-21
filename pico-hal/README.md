# Plant pot: HAL - medium level

This repository contains a setup for the Raspberry Pico that allows to communicate with it over USB and a serial interface.


## Flash

Mount the Pico and simply run

```bash
cargo run
```

## Connect

Launch `sudo dmesg -W` in a terminal. Connect the Pico in BOOTSEL mode.

Observe the output of `dmesg` to know the name of the new serial connection.


On Linux:

```bash
sudo apt install tio
sudo tio /dev/ttyACM0
```

Exit with CTRL-T Q.

Or 

```bash
sudo apt install minicom
sudo minicom -b 115200 -D /dev/ttyACM1
```

Press CTRL-A Z X to quit.



