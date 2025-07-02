use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkvm_from_scratch::{decode_hex, encode_hex, MerkleTree, Prover, StarkProver, VmState};

fn benchmark_vm_creation(c: &mut Criterion) {
    c.bench_function("vm_creation", |b| {
        b.iter(|| {
            let vm = black_box(VmState::new(1024 * 1024));
            vm
        })
    });
}

fn benchmark_vm_execution(c: &mut Criterion) {
    c.bench_function("vm_instruction_execution", |b| {
        b.iter(|| {
            let mut vm = VmState::new(1024 * 1024);
            let program = vec![0x33, 0x01, 0x10, 0x00]; // ADD instruction
            vm.memory.load_program(&program, 0).unwrap();
            vm.registers[1] = black_box(42);
            vm.registers[2] = black_box(17);
            vm.run(1).unwrap();
        })
    });
}

fn benchmark_merkle_tree_creation(c: &mut Criterion) {
    let data: Vec<Vec<u8>> = (0..1000i32).map(|i| i.to_le_bytes().to_vec()).collect();
    let leaves: Vec<&[u8]> = data.iter().map(|v| v.as_slice()).collect();

    c.bench_function("merkle_tree_1000_leaves", |b| {
        b.iter(|| {
            let tree = black_box(MerkleTree::new(leaves.clone()));
            tree
        })
    });
}

fn benchmark_hex_conversion(c: &mut Criterion) {
    let test_data = vec![0u8; 1024]; // 1KB of data

    c.bench_function("bytes_to_hex_1kb", |b| {
        b.iter(|| {
            let hex = black_box(encode_hex(&test_data));
            hex
        })
    });

    let hex_string = encode_hex(&test_data);
    c.bench_function("hex_to_bytes_1kb", |b| {
        b.iter(|| {
            let bytes = black_box(decode_hex(&hex_string).unwrap());
            bytes
        })
    });
}

fn benchmark_zkvm_proof_generation(c: &mut Criterion) {
    c.bench_function("zkvm_proof_generation", |b| {
        b.iter(|| {
            let mut vm = VmState::new(1024 * 1024);
            let program = vec![0x33, 0x01, 0x10, 0x00]; // ADD instruction
            vm.memory.load_program(&program, 0).unwrap();
            vm.registers[1] = black_box(42);

            let stark_prover = StarkProver::default();
            let prover = Prover::new(stark_prover);
            let trace = prover.generate_execution_trace(&mut vm, 1).unwrap();
            let _proof = prover.prove_execution(&trace).unwrap();
        })
    });
}

criterion_group!(
    benches,
    benchmark_vm_creation,
    benchmark_vm_execution,
    benchmark_merkle_tree_creation,
    benchmark_hex_conversion,
    benchmark_zkvm_proof_generation
);

criterion_main!(benches);
