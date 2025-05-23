# Setup the WiFi stack of Embassy

Creating a WiFi network stack is a little bit too complicated, so you just have to write a small function to randomize the seed used for authentication WiFi. (It is not required to be random, but probably a good idea.)

The core of this code has been taken from the [example code](https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/blinky_wifi_pico_plus_2.rs) of the Embassy project.
