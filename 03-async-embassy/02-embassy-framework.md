
# Embassy

The Embassy framework provides an asynchronous executor that can be used to run many asynchronous tasks concurrently.

Different tasks can have different priorities by assigning them to executors on different cores manually. This will be demonstrated in one of the exercises.

Embassy tasks are run cooperatively: we assume they will give up (yield) control voluntarily to other tasks. If you want more automatic scheduling of tasks, you need to use a different framework, [RTIC](https://github.com/rtic-rs/rtic) that can suspend running tasks.

For a more standard synchronous (blocking) API, you can use the rp253x-hal crate, which is a hardware abstraction layer (HAL) for the Raspberry Pi Pico 2 W. This HAL provides a synchronous API to interact with the hardware peripherals of the micro-controller.

## Standardized API

The Embassy project provides plugin crates for different support micro-controllers. Thes plugin crates offer a standardize API to do common operations on these micro-controllers.

There is a "[plugin crate](https://crates.io/crates/embassy-rp)" for the different versions of Raspberry Pico such as the "Pico 2 W" (which we will use in this workshop).

## Getting started

Most of the content of this workshop is inspired by the [`examples`](https://github.com/embassy-rs/embassy/tree/main/examples/rp235x/src/bin) of the Embassy project.

If you want do something with another micro-controller, you can look at the "examples" directories of other `embassy-[MODEL]` crates where "MODEL" is the specific model of the micro-controller you are using.
