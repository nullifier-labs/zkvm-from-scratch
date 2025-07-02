use super::{Instruction, Opcode, Memory};

#[derive(Debug, Clone)]
pub struct VmState {
    pub registers: [u32; 32],
    pub pc: u32,
    pub memory: Memory,
}

#[derive(Debug, Clone)]
pub struct ExecutionStep {
    pub pc_before: u32,
    pub pc_after: u32,
    pub registers_before: [u32; 32],
    pub registers_after: [u32; 32],
    pub instruction: Instruction,
    pub memory_reads: Vec<(u32, u32)>,
    pub memory_writes: Vec<(u32, u32, u32)>,
    pub intermediate_values: Vec<u32>,
}

impl VmState {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; 32],
            pc: 0,
            memory: Memory::new(memory_size),
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) -> Result<(), &'static str> {
        match instruction.opcode {
            Opcode::Add => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                self.registers[instruction.rd as usize] = val1.wrapping_add(val2);
            }
            Opcode::Sub => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                self.registers[instruction.rd as usize] = val1.wrapping_sub(val2);
            }
            Opcode::Mul => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                self.registers[instruction.rd as usize] = val1.wrapping_mul(val2);
            }
            Opcode::Div => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                if val2 == 0 {
                    return Err("Division by zero");
                }
                self.registers[instruction.rd as usize] = val1 / val2;
            }
            Opcode::Load => {
                let addr = self.registers[instruction.rs1 as usize]
                    .wrapping_add(instruction.imm as u32);
                let value = self.memory.read_word(addr)?;
                self.registers[instruction.rd as usize] = value;
            }
            Opcode::Store => {
                let addr = self.registers[instruction.rs1 as usize]
                    .wrapping_add(instruction.imm as u32);
                let value = self.registers[instruction.rs2 as usize];
                self.memory.write_word(addr, value)?;
            }
            Opcode::Branch => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                if val1 == val2 {
                    self.pc = self.pc.wrapping_add(instruction.imm as u32);
                    return Ok(());
                }
            }
            Opcode::Jump => {
                self.pc = self.pc.wrapping_add(instruction.imm as u32);
                return Ok(());
            }
            Opcode::Nop => {}
        }

        self.registers[0] = 0;
        self.pc = self.pc.wrapping_add(4);
        Ok(())
    }

    pub fn execute_with_trace(&mut self, instruction: Instruction) -> Result<ExecutionStep, &'static str> {
        let pc_before = self.pc;
        let registers_before = self.registers;
        let mut memory_reads = Vec::new();
        let mut memory_writes = Vec::new();
        let mut intermediate_values = Vec::new();

        match instruction.opcode {
            Opcode::Add => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                intermediate_values.push(val1);
                intermediate_values.push(val2);
                let result = val1.wrapping_add(val2);
                intermediate_values.push(result);
                self.registers[instruction.rd as usize] = result;
            }
            Opcode::Sub => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                intermediate_values.push(val1);
                intermediate_values.push(val2);
                let result = val1.wrapping_sub(val2);
                intermediate_values.push(result);
                self.registers[instruction.rd as usize] = result;
            }
            Opcode::Mul => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                intermediate_values.push(val1);
                intermediate_values.push(val2);
                let result = val1.wrapping_mul(val2);
                intermediate_values.push(result);
                self.registers[instruction.rd as usize] = result;
            }
            Opcode::Div => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                if val2 == 0 {
                    return Err("Division by zero");
                }
                intermediate_values.push(val1);
                intermediate_values.push(val2);
                let result = val1 / val2;
                intermediate_values.push(result);
                self.registers[instruction.rd as usize] = result;
            }
            Opcode::Load => {
                let base_addr = self.registers[instruction.rs1 as usize];
                let addr = base_addr.wrapping_add(instruction.imm as u32);
                intermediate_values.push(base_addr);
                intermediate_values.push(instruction.imm as u32);
                intermediate_values.push(addr);
                let value = self.memory.read_word(addr)?;
                memory_reads.push((addr, value));
                intermediate_values.push(value);
                self.registers[instruction.rd as usize] = value;
            }
            Opcode::Store => {
                let base_addr = self.registers[instruction.rs1 as usize];
                let addr = base_addr.wrapping_add(instruction.imm as u32);
                let value = self.registers[instruction.rs2 as usize];
                intermediate_values.push(base_addr);
                intermediate_values.push(instruction.imm as u32);
                intermediate_values.push(addr);
                intermediate_values.push(value);
                let old_value = self.memory.read_word(addr).unwrap_or(0);
                self.memory.write_word(addr, value)?;
                memory_writes.push((addr, old_value, value));
            }
            Opcode::Branch => {
                let val1 = self.registers[instruction.rs1 as usize];
                let val2 = self.registers[instruction.rs2 as usize];
                intermediate_values.push(val1);
                intermediate_values.push(val2);
                let is_equal = val1 == val2;
                intermediate_values.push(if is_equal { 1 } else { 0 });
                if is_equal {
                    self.pc = self.pc.wrapping_add(instruction.imm as u32);
                    let pc_after = self.pc;
                    return Ok(ExecutionStep {
                        pc_before,
                        pc_after,
                        registers_before,
                        registers_after: self.registers,
                        instruction,
                        memory_reads,
                        memory_writes,
                        intermediate_values,
                    });
                }
            }
            Opcode::Jump => {
                self.pc = self.pc.wrapping_add(instruction.imm as u32);
                let pc_after = self.pc;
                return Ok(ExecutionStep {
                    pc_before,
                    pc_after,
                    registers_before,
                    registers_after: self.registers,
                    instruction,
                    memory_reads,
                    memory_writes,
                    intermediate_values,
                });
            }
            Opcode::Nop => {}
        }

        self.registers[0] = 0;
        self.pc = self.pc.wrapping_add(4);
        
        Ok(ExecutionStep {
            pc_before,
            pc_after: self.pc,
            registers_before,
            registers_after: self.registers,
            instruction,
            memory_reads,
            memory_writes,
            intermediate_values,
        })
    }

    pub fn run(&mut self, max_steps: usize) -> Result<(), &'static str> {
        for _ in 0..max_steps {
            let instruction_word = self.memory.read_word(self.pc)?;
            let instruction = super::decode_instruction(instruction_word)?;
            self.execute_instruction(instruction)?;
        }
        Ok(())
    }

    pub fn run_with_trace(&mut self, max_steps: usize) -> Result<Vec<ExecutionStep>, &'static str> {
        let mut trace = Vec::new();
        
        for _ in 0..max_steps {
            let instruction_word = self.memory.read_word(self.pc)?;
            let instruction = super::decode_instruction(instruction_word)?;
            let step = self.execute_with_trace(instruction)?;
            trace.push(step);
        }
        
        Ok(trace)
    }
}