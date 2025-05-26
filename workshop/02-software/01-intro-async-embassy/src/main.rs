// This tells the Rust compiler not to link the standard library with this
// binary. This will keep the binary small and suitable for embedded systems.
#![no_std]
// This indicates that the `main` function written in this file is not a typical main function
// compared to a standard Rust application. Instead, it is an entry point for an embedded
// application. The main function cannot end since there is no operating system to return to.
#![no_main]

// Linked crates
// The code does not directly contain imports of objects from these libraries,
// but these libraries contain static data that is important to be able to run
// the code on an embedded system.
use cortex_m_rt as _;
use defmt_rtt as _;
// Real import dependencies
use embassy_executor::{Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::config::Config;
use panic_probe as _;

#[main]
async fn main(_spawner: Spawner) -> ! {
    let _p = embassy_rp::init(Config::default());
    loop {
        // Because a `!` / "never" output type is required for the main function, we use
        // a small hack to keep the program running indefinitely.
        // The `yield_now` function is an async function that yields control back to the
        // executor, allowing other tasks to run.
        yield_now().await;
    }
}
