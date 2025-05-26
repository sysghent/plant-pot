# Tune motor speed with PWM

Change the speed of the pump's motor is not strictly necessary. However, it is common in other projects.

In this project you will find a way to slow down the motor using pulse width modulation (PWM). This is a technique used to control the amount of power delivered to an electrical device by varying the width of the pulses in a pulse train.

The average power delivered to the load is proportional to the duty cycle of the pulse train.

See <https://pico.implrust.com/led/pwm-rp2350.html>

## PioPWM

The Pico board also has multiple PIO peripherals. This is a programmable input/output peripheral that can be used to implement custom protocols and control devices.

Creating a PWM output with the PIO peripheral requires more work, but may be more performant than using simpler ways to drive PWM outputs.

See <https://github.com/embassy-rs/embassy/blob/main/examples/rp235x/src/bin/pio_pwm.rs>
