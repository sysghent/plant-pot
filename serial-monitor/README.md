
If you also want to communicate with the device over serial port, you need:

```bash
cargo add heapless usbd-serial usb-device
```


### Serial echo

The example [./examples/serial_comm.rs] echos serial input on the serial port

Launch `sudo dmesg | tail -f` in a terminal. Connect the Pico in BOOTSEL mode.

Observe the output of `dmesg` to know the name of the new serial connection.

Mount the Pico storage device and flash the Rust program with

```bash
cargo run --example serial_comm
```

On Linux:

```bash
sudo apt install minicom
sudo minicom -b 115200 -D /dev/ttyACM1
```

Type some letters in the terminal and see how the flashed program transforms them.

Press CTRL-A Z X to quit.