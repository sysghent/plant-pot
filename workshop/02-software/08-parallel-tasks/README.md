# Parallelism on a multicore microcontroller

The Raspberry Pi Pico 2 W has two cores. This allows to run multiple tasks in parallel. The Embassy framework provides a way to run tasks on different cores. This is done by assigning tasks to different executors. Each executor runs on a different core.

In this exercise you have to try to run two tasks in parallel, not just concurrently, but simultaneously on two different cores.

_**Remark**: Notice that both tasks on both cores are blocking (non-asynchronous). In other words, we don't actually need the Embassy framework to run these tasks. However, there are no threads available on the Pico and we need the 'spawn' made specifically for this micro-controller architecture: 'cortex-m'._
