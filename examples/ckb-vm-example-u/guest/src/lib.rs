#![cfg_attr(feature = "guest", no_std)]

#[cfg(feature = "guest")]
extern crate alloc;
use ckb_vm::Bytes;
use jolt::{end_cycle_tracking, start_cycle_tracking};

mod exec_syscall_handler;
mod executor_asm;

#[jolt::provable(
    max_input_size = 4_000_000,
    max_trace_length = 1073741824, // 2^26
    stack_size = 524288, // 512KB
    heap_size = 67108864 // 64MB — testing if OOM is the cause
)]
fn verify_script(program_elf: &[u8], trace_data: &[u8], script_version: u8) {
    start_cycle_tracking("ckb-vm replay");

    executor_asm::VmExecutor::new(
        Bytes::copy_from_slice(trace_data),
        Bytes::copy_from_slice(program_elf),
        script_version,
    )
    .execute();

    end_cycle_tracking("ckb-vm replay");
}
