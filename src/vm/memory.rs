use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Memory {
    memory: HashMap<u32, u8>,
    size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self {
            memory: HashMap::new(),
            size,
        }
    }

    pub fn read_byte(&self, addr: u32) -> Result<u8, &'static str> {
        if addr as usize >= self.size {
            return Err("Memory address out of bounds");
        }
        Ok(self.memory.get(&addr).copied().unwrap_or(0))
    }

    pub fn write_byte(&mut self, addr: u32, value: u8) -> Result<(), &'static str> {
        if addr as usize >= self.size {
            return Err("Memory address out of bounds");
        }
        self.memory.insert(addr, value);
        Ok(())
    }

    pub fn read_word(&self, addr: u32) -> Result<u32, &'static str> {
        if addr % 4 != 0 {
            return Err("Unaligned memory access");
        }
        
        let mut word = 0u32;
        for i in 0..4 {
            let byte = self.read_byte(addr + i)?;
            word |= (byte as u32) << (i * 8);
        }
        Ok(word)
    }

    pub fn write_word(&mut self, addr: u32, value: u32) -> Result<(), &'static str> {
        if addr % 4 != 0 {
            return Err("Unaligned memory access");
        }
        
        for i in 0..4 {
            let byte = ((value >> (i * 8)) & 0xff) as u8;
            self.write_byte(addr + i, byte)?;
        }
        Ok(())
    }

    pub fn load_program(&mut self, program: &[u8], start_addr: u32) -> Result<(), &'static str> {
        for (i, &byte) in program.iter().enumerate() {
            self.write_byte(start_addr + i as u32, byte)?;
        }
        Ok(())
    }
}