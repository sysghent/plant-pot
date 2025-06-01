#![no_std]

// This is common in systems where you might not implement every possible
// interrupt handler (yet), but you still need a catch-all.
#[cfg(target_arch = "riscv32")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DefaultIrqHandler() {
    loop {}
}
