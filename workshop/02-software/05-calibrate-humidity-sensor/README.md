# Calibrate humidity sensor

In this exercise you need to find a way to transform raw ADC values into a humidity value.

The ADC values are n-bit. Find how many bits are used for the ADC on the Pico. Then find the minimum and maximum for a cup of water and a air.

Use this to make a formula that transforms the ADC value into a humidity value.

This step is similar to the `map` function in ArduinoIDE. It is just a linear transformation.
