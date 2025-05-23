# Serial print debugging

On Linux:

```bash
sudo apt install tio
sudo tio /dev/ttyACM0
```

_Remark: If `ttyACM0` is not the right device, you can find the right device with: `sudo dmesg -W`. Connect the Pico in BOOTSEL mode. Observe the output of `dmesg` to know the name of the new serial connection._

Exit with CTRL-T Q.
