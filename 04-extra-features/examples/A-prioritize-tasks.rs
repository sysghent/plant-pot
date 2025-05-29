//! # Parallelism on a multicore microcontroller
//!
//! The Raspberry Pi Pico 2 W has two cores. This allows to run multiple tasks
//! in parallel. The Embassy framework provides a way to run tasks on different
//! cores. This is done by assigning tasks to different executors. Each executor
//! runs on a different core.
//!
//! In this exercise you have to try to run two tasks in parallel, not just
//! concurrently, but simultaneously on two different cores.
//!
//! _**Remark**: Notice that both tasks on both cores are blocking
//! (non-asynchronous). In other words, we don't actually need the Embassy
//! framework to run these tasks. However, there are no threads available on the
//! Pico and we need the 'spawn' made specifically for this micro-controller
//! architecture: 'cortex-m'._

#![no_std]
#![no_main]

use cortex_m_rt as _;
use embassy_executor::{Executor, Spawner, main};
use embassy_futures::yield_now;
use embassy_rp::{
    config::{self},
    multicore::Stack,
};
use static_cell::StaticCell;
static CORE1_ASYNC_EXECUTOR: StaticCell<Executor> = StaticCell::new();

static mut CORE1_VAR_STACK: Stack<4096> = Stack::new();

#[main]
async fn main(_spawner: Spawner) -> ! {
    let _p = embassy_rp::init(config::Config::default());

    let _stack = unsafe { &mut *core::ptr::addr_of_mut!(CORE1_VAR_STACK) };

    let _second_core_task = || {
        let on_board_executor = CORE1_ASYNC_EXECUTOR.init(Executor::new());
        on_board_executor.run(|spawner| {
            spawner.spawn(low_priority_task()).unwrap();
        });
    };

    todo!("Use spawn_core1 to run the second task on the second core 'core1' (in a blocking way).");
}

#[embassy_executor::task]
pub async fn low_priority_task() -> ! {
    loop {
        yield_now().await;
    }
}

#[embassy_executor::task]
pub async fn high_priority_task() -> ! {
    loop {
        yield_now().await;
    }
}
