# Blink light with timers

In this exercise you will do the most minimal thing you can do with Embassy: blink an LED with some interval.

This is also called the "Hello World" of embedded programming.

Notice that Embassy provide a `Duration` type that can be used for all supported micro-controllers. This is a good way to make your code portable between different platforms.

Underneath the `async` front-end, Embassy creates a hardware timer. If you prefer to instead use a hardware timer directly, you can drop Embassy and use the `rp235x-hal` crate directly. This is a lower-level approach that requires more code to achieve the same result.
