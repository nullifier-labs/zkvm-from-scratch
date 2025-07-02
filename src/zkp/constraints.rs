use super::{ExecutionTrace, TraceStep};
use crate::vm::Opcode;

#[derive(Debug, Clone)]
pub struct ConstraintSystem {
    pub constraints: Vec<Constraint>,
    pub witness_columns: Vec<String>,
    pub public_columns: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum Constraint {
    Equality {
        left: Expression,
        right: Expression,
    },
    RangeCheck {
        value: Expression,
        max_bits: u32,
    },
    MemoryConsistency {
        addr: Expression,
        value_read: Expression,
        value_written: Expression,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Constant(u32),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
}

impl Default for ConstraintSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl ConstraintSystem {
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            witness_columns: Vec::new(),
            public_columns: Vec::new(),
        }
    }

    pub fn add_witness_column(&mut self, name: String) {
        self.witness_columns.push(name);
    }

    pub fn add_public_column(&mut self, name: String) {
        self.public_columns.push(name);
    }

    pub fn add_constraint(&mut self, constraint: Constraint) {
        self.constraints.push(constraint);
    }

    pub fn generate_constraints_for_trace(&mut self, trace: &ExecutionTrace) {
        // Add standard witness columns
        self.add_witness_column("step".to_string());
        self.add_witness_column("pc_before".to_string());
        self.add_witness_column("pc_after".to_string());

        for i in 0..32 {
            self.add_witness_column(format!("reg_{i}_before"));
            self.add_witness_column(format!("reg_{i}_after"));
        }

        // Add public columns for initial and final state
        self.add_public_column("initial_pc".to_string());
        self.add_public_column("final_pc".to_string());

        for (step_idx, step) in trace.steps.iter().enumerate() {
            self.generate_constraints_for_step(step, step_idx);
        }
    }

    fn generate_constraints_for_step(&mut self, step: &TraceStep, step_idx: usize) {
        let _step_var = Expression::Variable(format!("step_{step_idx}"));
        let pc_before = Expression::Variable(format!("pc_before_{step_idx}"));
        let pc_after = Expression::Variable(format!("pc_after_{step_idx}"));

        // PC progression constraint for most instructions
        match step.instruction.opcode {
            Opcode::Branch | Opcode::Jump => {
                // Branch/Jump instructions handle PC differently
            }
            _ => {
                // Normal instructions: pc_after = pc_before + 4
                self.add_constraint(Constraint::Equality {
                    left: pc_after.clone(),
                    right: Expression::Add(
                        Box::new(pc_before.clone()),
                        Box::new(Expression::Constant(4)),
                    ),
                });
            }
        }

        // Instruction-specific constraints
        match step.instruction.opcode {
            Opcode::Add => {
                let rs1_val = Expression::Variable(format!(
                    "reg_{}_before_{}",
                    step.instruction.rs1, step_idx
                ));
                let rs2_val = Expression::Variable(format!(
                    "reg_{}_before_{}",
                    step.instruction.rs2, step_idx
                ));
                let rd_val =
                    Expression::Variable(format!("reg_{}_after_{}", step.instruction.rd, step_idx));

                // rd = rs1 + rs2
                self.add_constraint(Constraint::Equality {
                    left: rd_val,
                    right: Expression::Add(Box::new(rs1_val), Box::new(rs2_val)),
                });
            }
            Opcode::Sub => {
                let rs1_val = Expression::Variable(format!(
                    "reg_{}_before_{}",
                    step.instruction.rs1, step_idx
                ));
                let rs2_val = Expression::Variable(format!(
                    "reg_{}_before_{}",
                    step.instruction.rs2, step_idx
                ));
                let rd_val =
                    Expression::Variable(format!("reg_{}_after_{}", step.instruction.rd, step_idx));

                // rd = rs1 - rs2
                self.add_constraint(Constraint::Equality {
                    left: rd_val,
                    right: Expression::Sub(Box::new(rs1_val), Box::new(rs2_val)),
                });
            }
            Opcode::Mul => {
                let rs1_val = Expression::Variable(format!(
                    "reg_{}_before_{}",
                    step.instruction.rs1, step_idx
                ));
                let rs2_val = Expression::Variable(format!(
                    "reg_{}_before_{}",
                    step.instruction.rs2, step_idx
                ));
                let rd_val =
                    Expression::Variable(format!("reg_{}_after_{}", step.instruction.rd, step_idx));

                // rd = rs1 * rs2
                self.add_constraint(Constraint::Equality {
                    left: rd_val,
                    right: Expression::Mul(Box::new(rs1_val), Box::new(rs2_val)),
                });
            }
            Opcode::Load | Opcode::Store => {
                // Memory access constraints
                for mem_access in &step.memory_accesses {
                    self.add_constraint(Constraint::MemoryConsistency {
                        addr: Expression::Constant(mem_access.addr),
                        value_read: Expression::Constant(mem_access.value_before),
                        value_written: Expression::Constant(mem_access.value_after),
                    });
                }
            }
            _ => {}
        }

        // Register 0 is always 0 constraint
        self.add_constraint(Constraint::Equality {
            left: Expression::Variable(format!("reg_0_after_{step_idx}")),
            right: Expression::Constant(0),
        });

        // Range checks for register values (32-bit)
        for i in 0..32 {
            self.add_constraint(Constraint::RangeCheck {
                value: Expression::Variable(format!("reg_{i}_after_{step_idx}")),
                max_bits: 32,
            });
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    pub fn evaluate_expression(
        &self,
        expr: &Expression,
        witness: &std::collections::HashMap<String, u32>,
    ) -> Option<u32> {
        match expr {
            Expression::Constant(val) => Some(*val),
            Expression::Variable(name) => witness.get(name).copied(),
            Expression::Add(left, right) => {
                let left_val = self.evaluate_expression(left, witness)?;
                let right_val = self.evaluate_expression(right, witness)?;
                Some(left_val.wrapping_add(right_val))
            }
            Expression::Mul(left, right) => {
                let left_val = self.evaluate_expression(left, witness)?;
                let right_val = self.evaluate_expression(right, witness)?;
                Some(left_val.wrapping_mul(right_val))
            }
            Expression::Sub(left, right) => {
                let left_val = self.evaluate_expression(left, witness)?;
                let right_val = self.evaluate_expression(right, witness)?;
                Some(left_val.wrapping_sub(right_val))
            }
        }
    }

    pub fn verify_constraints(&self, witness: &std::collections::HashMap<String, u32>) -> bool {
        for constraint in &self.constraints {
            if !self.verify_constraint(constraint, witness) {
                return false;
            }
        }
        true
    }

    pub fn verify_constraint(
        &self,
        constraint: &Constraint,
        witness: &std::collections::HashMap<String, u32>,
    ) -> bool {
        match constraint {
            Constraint::Equality { left, right } => {
                let left_val = self.evaluate_expression(left, witness);
                let right_val = self.evaluate_expression(right, witness);
                left_val == right_val && left_val.is_some()
            }
            Constraint::RangeCheck { value, max_bits } => {
                if let Some(val) = self.evaluate_expression(value, witness) {
                    if *max_bits >= 32 {
                        true // u32 values are always within 32-bit range
                    } else {
                        val < (1u32 << max_bits)
                    }
                } else {
                    false
                }
            }
            Constraint::MemoryConsistency { .. } => {
                // For now, assume memory consistency is always satisfied
                // In a real implementation, this would check memory operation ordering
                true
            }
        }
    }
}
