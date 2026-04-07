use std::time::Instant;
use tracing::info;

pub fn main() {
    tracing_subscriber::fmt::init();

    // Load the CKB script example data
    let program_elf = std::fs::read("examples/ckb-vm-example-u/script_example/program_elf.bin")
        .expect("failed to read program_elf.bin");
    let trace_data = std::fs::read("examples/ckb-vm-example-u/script_example/trace_data.bin")
        .expect("failed to read trace_data.bin");
    let script_version_bytes =
        std::fs::read("examples/ckb-vm-example-u/script_example/script_version.bin")
            .expect("failed to read script_version.bin");
    let script_version: u8 = match script_version_bytes.as_slice() {
        b"V0" => 0,
        b"V1" => 1,
        b"V2" => 2,
        _ => panic!("Unknown script version"),
    };

    info!(
        "Loaded script: ELF={} bytes, trace={} bytes, version={}",
        program_elf.len(),
        trace_data.len(),
        script_version
    );

    let target_dir = "/tmp/jolt-guest-targets";
    let mut program = guest::compile_verify_script(target_dir);

    info!("Preprocessing...");
    let shared_preprocessing = guest::preprocess_shared_verify_script(&mut program);
    let prover_preprocessing = guest::preprocess_prover_verify_script(
        shared_preprocessing.clone().expect("Preprocessing failed"),
    );
    let verifier_setup = prover_preprocessing.generators.to_verifier_setup();
    let verifier_preprocessing = guest::preprocess_verifier_verify_script(
        shared_preprocessing.unwrap(),
        verifier_setup,
        None,
    ); // Maybe i need to fix that to use ZK mode

    let prove = guest::build_prover_verify_script(program, prover_preprocessing);
    let verify = guest::build_verifier_verify_script(verifier_preprocessing);

    info!("Proving...");
    let now = Instant::now();
    let (output, proof, io_device) = prove(&program_elf, &trace_data, script_version);
    info!("Prover runtime: {} s", now.elapsed().as_secs_f64());

    info!("Raw output: {:?}, panic: {}", output, io_device.panic);

    info!("Verifying...");
    let is_valid = verify(
        &program_elf,
        &trace_data,
        script_version,
        output,
        io_device.panic,
        proof,
    );
    info!("valid: {is_valid}");
}
