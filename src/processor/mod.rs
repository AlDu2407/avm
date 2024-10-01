use std::{collections::HashMap, fmt::Display};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Register {
    IP,
    ACC,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

impl Register {
    pub fn values() -> Vec<Register> {
        vec![
            Register::IP,
            Register::ACC,
            Register::R1,
            Register::R2,
            Register::R3,
            Register::R4,
            Register::R5,
            Register::R6,
            Register::R7,
            Register::R8,
        ]
    }

    pub fn register_amount() -> usize {
        Self::values().len()
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::IP => write!(f, "[IP]"),
            Register::ACC => write!(f, "[ACC]"),
            Register::R1 => write!(f, "[R1]"),
            Register::R2 => write!(f, "[R2]"),
            Register::R3 => write!(f, "[R3]"),
            Register::R4 => write!(f, "[R4]"),
            Register::R5 => write!(f, "[R5]"),
            Register::R6 => write!(f, "[R6]"),
            Register::R7 => write!(f, "[R7]"),
            Register::R8 => write!(f, "[R8]"),
        }
    }
}

#[allow(dead_code)]
pub struct Cpu {
    memory: Vec<u8>,
    registers: HashMap<Register, u16>,
}

impl Cpu {
    pub fn with_memory_capacity(capacity_in_bytes: usize) -> Self {
        let mut registers = HashMap::with_capacity(Register::register_amount());
        for register in Register::values() {
            registers.insert(register, 0_u16);
        }

        Self {
            memory: Vec::with_capacity(capacity_in_bytes),
            registers,
        }
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPU: {{ ")?;
        for (key, value) in &self.registers {
            write!(f, "{} -> {}; ", key, value)?;
        }
        write!(f, "}}")
    }
}

pub enum Instruction {
    MovR1,
    MovR2,
    AddR1R2,
}

impl TryFrom<u8> for Instruction {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x10 => Ok(Instruction::MovR1),
            0x11 => Ok(Instruction::MovR2),
            0x12 => Ok(Instruction::AddR1R2),
            _ => Err(format!(
                "ERROR: Unknown byte representation of Instruction provided {}",
                value
            )),
        }
    }
}
