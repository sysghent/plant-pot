
# Embedded async framework: Embassy

The Embassy framework provides an asynchronous executor that can be used to run many asynchronous tasks concurrently.

Different tasks can have different priorities by assigning them to executors on different cores manually. This will be demonstrated in one of the exercises.

_**Remark**: Different tasks are run co-operatively. If you want more automatic scheduling of tasks, you need to use a different framework, [RTIC](https://github.com/rtic-rs/rtic) that can suspend running tasks._

## Standardized API

The Embassy project provides plugin crates for different support micro-controllers. Thes plugin crates offer a standardize API to do common operations on these micro-controllers.

There is a "[plugin crate](https://crates.io/crates/embassy-rp)" for the different versions of Raspberry Pico such as the "Pico 2 W" (which we will use in this workshop).

## Getting started

Most of the content of this workshop is inspired by the [`examples`](https://github.com/embassy-rs/embassy/tree/main/examples/rp235x/src/bin) of the Embassy project.

If you want do something with another micro-controller, you can look at the "examples" directories of other `embassy-[MODEL]` crates where "MODEL" is the specific model of the micro-controller you are using.
