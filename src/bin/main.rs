use zkvm_from_scratch::{utils, vm::VirtualMachine};

fn main() {
    println!("🚀 ZKVM from Scratch - Development Environment");
    println!("==============================================");

    // Create a new virtual machine instance
    let mut vm = VirtualMachine::new();
    println!("✓ Virtual machine created");
    println!("  - PC: {}", vm.pc);
    println!("  - Memory size: {}", vm.memory.len());
    println!("  - Stack size: {}", vm.stack.len());

    // Demonstrate some basic functionality
    vm.stack.push(42);
    vm.stack.push(100);
    println!("✓ Added values to stack: {:?}", vm.stack);

    // Test hex utilities
    let test_bytes = vec![0xde, 0xad, 0xbe, 0xef];
    let hex_string = utils::bytes_to_hex(&test_bytes);
    println!("✓ Hex conversion test: {:?} -> {}", test_bytes, hex_string);

    // Reset VM
    vm.reset();
    println!("✓ Virtual machine reset");
    println!("  - PC: {}", vm.pc);
    println!("  - Stack size: {}", vm.stack.len());

    println!("\n🎉 Development environment is working!");
    println!("Ready to build your zero-knowledge virtual machine!");
}
