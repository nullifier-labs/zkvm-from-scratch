use crate::vm::{Instruction, VmState};

#[derive(Debug, Clone)]
pub struct ExecutionTrace {
    pub steps: Vec<TraceStep>,
    pub initial_state: VmState,
    pub final_state: VmState,
}

#[derive(Debug, Clone)]
pub struct TraceStep {
    pub step_index: usize,
    pub pc_before: u32,
    pub pc_after: u32,
    pub registers_before: [u32; 32],
    pub registers_after: [u32; 32],
    pub instruction: Instruction,
    pub memory_accesses: Vec<MemoryAccess>,
    pub intermediate_values: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct MemoryAccess {
    pub addr: u32,
    pub value_before: u32,
    pub value_after: u32,
    pub is_write: bool,
}

#[derive(Debug, Clone)]
pub struct WitnessData {
    pub trace: ExecutionTrace,
    pub public_inputs: Vec<u32>,
    pub private_inputs: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub trace_commitment: Vec<u8>,
    pub witness: Vec<u8>,
}

pub trait ProofSystem {
    type Error: std::fmt::Debug;

    fn generate_proof(&self, trace: &ExecutionTrace) -> Result<Proof, Self::Error>;
}

pub struct Prover<P: ProofSystem> {
    proof_system: P,
}

impl<P: ProofSystem> Prover<P> {
    pub fn new(proof_system: P) -> Self {
        Self { proof_system }
    }

    pub fn generate_execution_trace(
        &self,
        vm_state: &mut VmState,
        max_steps: usize,
    ) -> Result<ExecutionTrace, &'static str> {
        let initial_state = vm_state.clone();
        let execution_steps = vm_state.run_with_trace(max_steps)?;
        let final_state = vm_state.clone();

        let mut trace_steps = Vec::new();
        for (index, exec_step) in execution_steps.iter().enumerate() {
            let mut memory_accesses = Vec::new();

            // Convert memory reads
            for (addr, value) in &exec_step.memory_reads {
                memory_accesses.push(MemoryAccess {
                    addr: *addr,
                    value_before: *value,
                    value_after: *value,
                    is_write: false,
                });
            }

            // Convert memory writes
            for (addr, old_value, new_value) in &exec_step.memory_writes {
                memory_accesses.push(MemoryAccess {
                    addr: *addr,
                    value_before: *old_value,
                    value_after: *new_value,
                    is_write: true,
                });
            }

            trace_steps.push(TraceStep {
                step_index: index,
                pc_before: exec_step.pc_before,
                pc_after: exec_step.pc_after,
                registers_before: exec_step.registers_before,
                registers_after: exec_step.registers_after,
                instruction: exec_step.instruction,
                memory_accesses,
                intermediate_values: exec_step.intermediate_values.clone(),
            });
        }

        Ok(ExecutionTrace {
            steps: trace_steps,
            initial_state,
            final_state,
        })
    }

    pub fn generate_witness(
        &self,
        trace: &ExecutionTrace,
        public_inputs: Vec<u32>,
        private_inputs: Vec<u32>,
    ) -> WitnessData {
        WitnessData {
            trace: trace.clone(),
            public_inputs,
            private_inputs,
        }
    }

    pub fn prove_execution(&self, trace: &ExecutionTrace) -> Result<Proof, P::Error> {
        self.proof_system.generate_proof(trace)
    }
}
