use ckb_vm::{Bytes, DefaultMachineRunner, Register, SupportMachine, Syscalls};
use ckb_vm_fuzzing_utils::{SynchronousSyscalls, SyscallCode};
use protobuf_ckb_syscalls::ProtobufVmRunnerImpls;

#[derive(Clone, Copy)]
struct ExecOverride;

impl<Mac: SupportMachine> Syscalls<Mac> for ExecOverride {
    fn initialize(&mut self, _machine: &mut Mac) -> Result<(), ckb_vm::Error> {
        Ok(())
    }
    fn ecall(&mut self, machine: &mut Mac) -> Result<bool, ckb_vm::Error> {
        let Ok(syscall_code): Result<SyscallCode, _> =
            machine.registers()[ckb_vm::registers::A7].to_u64().try_into()
        else {
            return Ok(false);
        };
        if syscall_code == SyscallCode::Exec {
            machine.set_register(ckb_vm::registers::A0, Mac::REG::from_u64(0));
            return Ok(true);
        }
        Ok(false)
    }
}

fn load_test_data() -> (Bytes, Bytes) {
    let program_elf =
        std::fs::read("script_example/program_elf.bin").expect("failed to read program_elf.bin");
    let trace_data =
        std::fs::read("script_example/trace_data.bin").expect("failed to read trace_data.bin");

    let elf_bytes = Bytes::copy_from_slice(&program_elf);
    let trace_bytes = Bytes::copy_from_slice(&trace_data);

    assert!(
        elf_bytes.starts_with(&[0x7F, 0x45, 0x4C, 0x46]),
        "ELF magic check failed — program_elf is not a valid ELF"
    );

    (elf_bytes, trace_bytes)
}

/// No-asm: interpreter mode using DefaultCoreMachine + DefaultMachine
#[test]
fn debug_ckb_vm_no_asm() {
    let (elf_bytes, trace_bytes) = load_test_data();

    let machine_trace_impls = ProtobufVmRunnerImpls::new_with_bytes(&trace_bytes)
        .expect("Failed to decode protobuf trace");

    let machine_args: Vec<_> = machine_trace_impls
        .args()
        .iter()
        .map(|e| Ok(Bytes::copy_from_slice(e)))
        .collect();

    println!("[no-asm] Machine args count: {}", machine_args.len());

    let machine_syscall = SynchronousSyscalls::new(machine_trace_impls);

    let machine_core =
        ckb_vm::DefaultCoreMachine::<u64, ckb_vm::WXorXMemory<ckb_vm::FlatMemory<u64>>>::new(
            ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
            ckb_vm::machine::VERSION2,
            u64::MAX,
        );

    let mut machine = ckb_vm::RustDefaultMachineBuilder::new(machine_core)
        .syscall(Box::new(ExecOverride))
        .syscall(Box::new(machine_syscall))
        .build();

    machine
        .load_program(&elf_bytes, machine_args.into_iter())
        .expect("Failed to load program");

    println!("[no-asm] Program loaded, running...");

    match machine.run() {
        Ok(exit_code) => println!("[no-asm] VM exited with code: {exit_code}"),
        Err(e) => panic!("[no-asm] VM execution failed: {e:?}"),
    }
}

/// Asm: JIT mode using AsmCoreMachine + AsmMachine
#[test]
fn debug_ckb_vm_asm() {
    let (elf_bytes, trace_bytes) = load_test_data();

    let machine_trace_impls = ProtobufVmRunnerImpls::new_with_bytes(&trace_bytes)
        .expect("Failed to decode protobuf trace");

    let machine_args: Vec<_> = machine_trace_impls
        .args()
        .iter()
        .map(|e| Ok(Bytes::copy_from_slice(e)))
        .collect();

    println!("[asm] Machine args count: {}", machine_args.len());

    let machine_syscall = SynchronousSyscalls::new(machine_trace_impls);

    let asm_core = <ckb_vm::machine::asm::AsmCoreMachine as SupportMachine>::new(
        ckb_vm::ISA_IMC | ckb_vm::ISA_B | ckb_vm::ISA_MOP,
        ckb_vm::machine::VERSION2,
        u64::MAX,
    );

    let core = ckb_vm::machine::asm::AsmDefaultMachineBuilder::new(asm_core)
        .syscall(Box::new(ExecOverride))
        .syscall(Box::new(machine_syscall))
        .build();

    let mut machine = ckb_vm::machine::asm::AsmMachine::new(core);

    machine
        .load_program(&elf_bytes, machine_args.into_iter())
        .expect("Failed to load program");

    println!("[asm] Program loaded, running...");

    match machine.run() {
        Ok(exit_code) => println!("[asm] VM exited with code: {exit_code}"),
        Err(e) => panic!("[asm] VM execution failed: {e:?}"),
    }
}
