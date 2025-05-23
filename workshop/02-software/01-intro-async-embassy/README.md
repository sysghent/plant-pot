
# Embedded async framework: Embassy

The Embassy framework provides an asynchronous executor that can be used to run many asynchronous tasks concurrently. It is co-operative.

Different tasks can have different priorities by assigning them to executors on different cores manually.

_**Remark**: If you want more automatic scheduling, you need to use a different framework, such as RTIC that can suspend running tasks. Tasks will not be async._

The Embassy project provides a plugin that allows to use the specific hardware of a Raspberry Pi Pico through safe abstractions: <https://crates.io/crates/embassy-rp>.

Examples can be found in the `examples` folder of the Embassy project: <https://github.com/embassy-rs/embassy/tree/main/examples/rp235x/src/bin>.
