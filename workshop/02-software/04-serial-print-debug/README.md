# Serial print debugging

In this exercise you will see how to `print`-debug your code. You will find a way to send simple string messages over the USB cable to your laptop. This is a simple way to debug your code without using a debugger.

## Pre-requisites

Install a serial monitor. On Linux you can use `tio` or `picocom`. On Windows you can use `PuTTY` or `Tera Term`. On MacOS you can use `screen` or `CoolTerm`.

## Connecting

After plugging in the Pico (without pressing the BOOTSEL button), you should see a new serial device appear in `/dev/`. This is the USB serial device that you will use to communicate with the Pico.

```bash
sudo tio /dev/ttyACM0
```

_Remark: If `ttyACM0` is not the right device, you can find the right device with: `sudo dmesg -W`. Disconnect the Pico. Observe the output of `dmesg` to know the name of the new serial connection. Plug it back in if necessary._

The `tio` serial monitor can be exited with the strange shortcut `CTRL-T Q`.
