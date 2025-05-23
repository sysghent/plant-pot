# Read humidity with ADC

In this exercise you will read an output value of the ADC. You will use this value as input to compute the humidity later on.

For now, you just have to know that the ADC unit measures analogue values and converts them into digital values. It does this kind of independently of the CPU and pushes digital values into a FIFO queue / buffer. The CPU can then read the buffer and use the digital values.

We will only use the latest ADC value in the FIFO buffer.

_*Important*: Before you can use the ADC in Rust, you have to manually initialize the clocks._
