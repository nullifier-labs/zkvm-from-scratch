#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
    Load,
    Store,
    Branch,
    Jump,
    Nop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub imm: i32,
}

impl Instruction {
    pub fn new(opcode: Opcode, rd: u8, rs1: u8, rs2: u8, imm: i32) -> Self {
        Self {
            opcode,
            rd,
            rs1,
            rs2,
            imm,
        }
    }
}

pub fn decode_instruction(word: u32) -> Result<Instruction, &'static str> {
    let opcode_bits = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as u8;
    let rs1 = ((word >> 15) & 0x1f) as u8;
    let rs2 = ((word >> 20) & 0x1f) as u8;
    let imm = (word as i32) >> 20;

    let opcode = match opcode_bits {
        0x33 => match (word >> 25) & 0x7f {
            0x00 => Opcode::Add,
            0x20 => Opcode::Sub,
            0x01 => match (word >> 12) & 0x7 {
                0x0 => Opcode::Mul,
                0x4 => Opcode::Div,
                _ => return Err("Unknown M-extension instruction"),
            },
            _ => return Err("Unknown R-type instruction"),
        },
        0x03 => Opcode::Load,
        0x23 => Opcode::Store,
        0x63 => Opcode::Branch,
        0x6f => Opcode::Jump,
        _ => Opcode::Nop,
    };

    Ok(Instruction::new(opcode, rd, rs1, rs2, imm))
}