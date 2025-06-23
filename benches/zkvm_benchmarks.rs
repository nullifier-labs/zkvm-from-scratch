use criterion::{black_box, criterion_group, criterion_main, Criterion};
use zkvm_from_scratch::{crypto::MerkleTree, utils, vm::VirtualMachine};

fn benchmark_vm_creation(c: &mut Criterion) {
    c.bench_function("vm_creation", |b| {
        b.iter(|| {
            let vm = black_box(VirtualMachine::new());
            vm
        })
    });
}

fn benchmark_vm_stack_operations(c: &mut Criterion) {
    c.bench_function("vm_stack_push_pop", |b| {
        b.iter(|| {
            let mut vm = VirtualMachine::new();
            for i in 0..1000 {
                vm.stack.push(black_box(i));
            }
            while !vm.stack.is_empty() {
                black_box(vm.stack.pop());
            }
        })
    });
}

fn benchmark_merkle_tree_creation(c: &mut Criterion) {
    let leaves: Vec<[u8; 32]> = (0..1000)
        .map(|i| {
            let mut leaf = [0u8; 32];
            leaf[0..4].copy_from_slice(&i.to_le_bytes());
            leaf
        })
        .collect();

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
            let hex = black_box(utils::bytes_to_hex(&test_data));
            hex
        })
    });

    let hex_string = utils::bytes_to_hex(&test_data);
    c.bench_function("hex_to_bytes_1kb", |b| {
        b.iter(|| {
            let bytes = black_box(utils::hex_to_bytes(&hex_string).unwrap());
            bytes
        })
    });
}

criterion_group!(
    benches,
    benchmark_vm_creation,
    benchmark_vm_stack_operations,
    benchmark_merkle_tree_creation,
    benchmark_hex_conversion
);

criterion_main!(benches);
