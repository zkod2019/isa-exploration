use std::{collections::HashMap, convert::TryFrom};

use crate::inst::*;

#[derive(Debug, Clone, Copy)]
struct Registers {
    r0: u64,
    r1: u64,
    r2: u64,
    r3: u64,
    r4: u64,
    r5: u64,
    r6: u64,
    r7: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    r16: u64,
}

impl Registers {
    fn new() -> Self {
        Self {
            r0: 0,
            r1: 0,
            r2: 0,
            r3: 0,
            r4: 0,
            r5: 0,
            r6: 0,
            r7: 0,
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            r16: 0,
        }
    }

    fn get(&self, reg_id: &Register) -> u64 {
        match reg_id {
            Register::R0 => self.r0,
            Register::R1 => self.r1,
            Register::R2 => self.r2,
            Register::R3 => self.r3,
            Register::R4 => self.r4,
            Register::R5 => self.r5,
            Register::R6 => self.r6,
            Register::R7 => self.r7,
            Register::R8 => self.r8,
            Register::R9 => self.r9,
            Register::R10 => self.r10,
            Register::R11 => self.r11,
            Register::R12 => self.r12,
            Register::R13 => self.r13,
            Register::R14 => self.r14,
            Register::R15 => self.r15,
            Register::R16 => self.r16,
        }
    }

    fn set(&mut self, reg_id: &Register, new_value: u64) {
        match reg_id {
            Register::R0 => self.r0 = new_value,
            Register::R1 => self.r1 = new_value,
            Register::R2 => self.r2 = new_value,
            Register::R3 => self.r3 = new_value,
            Register::R4 => self.r4 = new_value,
            Register::R5 => self.r5 = new_value,
            Register::R6 => self.r6 = new_value,
            Register::R7 => self.r7 = new_value,
            Register::R8 => self.r8 = new_value,
            Register::R9 => self.r9 = new_value,
            Register::R10 => self.r10 = new_value,
            Register::R11 => self.r11 = new_value,
            Register::R12 => self.r12 = new_value,
            Register::R13 => self.r13 = new_value,
            Register::R14 => self.r14 = new_value,
            Register::R15 => self.r15 = new_value,
            Register::R16 => self.r16 = new_value,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VM<'a, const MEMORY_SIZE: usize> {
    registers: Registers,
    memory: [u8; MEMORY_SIZE],
    block_table: HashMap<&'a str, &'a Block>,
}

impl<'a, const MEMORY_SIZE: usize> VM<'a, MEMORY_SIZE> {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: [0; MEMORY_SIZE],
            block_table: HashMap::new(),
        }
    }

    pub fn interpret_inst(&mut self, inst: &Inst) {
        match inst {
            Inst::Dbg(reg) => println!("{}", self.registers.get(reg)),
            Inst::Rega(dst, value) => self.registers.set(dst, value.as_u64()),
            Inst::Copy(dst, src) => self.registers.set(dst, self.registers.get(src)),
            Inst::Load(dst, adr) => {
                let offset = self.registers.get(adr);
                // Find offset in memory and read 8 bytes forward
                let bytes: [u8; 8] = [
                    self.memory[usize::try_from(offset)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 1)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 2)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 3)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 4)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 5)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 6)
                        .expect("This VM can only function on 64 bit machines.")],
                    self.memory[usize::try_from(offset + 7)
                        .expect("This VM can only function on 64 bit machines.")],
                ];
                let value = u64::from_ne_bytes(bytes);
                self.registers.set(dst, value)
            }
            Inst::Store(adr, val) => {
                let value = self.registers.get(val);
                let offset = self.registers.get(adr);

                let bytes = value.to_ne_bytes();
                self.memory[usize::try_from(offset)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[0];
                self.memory[usize::try_from(offset + 1)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[1];
                self.memory[usize::try_from(offset + 2)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[2];
                self.memory[usize::try_from(offset + 3)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[3];
                self.memory[usize::try_from(offset + 4)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[4];
                self.memory[usize::try_from(offset + 5)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[5];
                self.memory[usize::try_from(offset + 6)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[6];
                self.memory[usize::try_from(offset + 7)
                    .expect("This VM can only function on 64 bit machines.")] = bytes[7];
            }
            Inst::Jump(target_label) => {
                if let Some(target_block) = self.block_table.get(target_label.0.as_str()) {
                    for inst in &target_block.insts {
                        self.interpret_inst(inst);
                    }
                } else {
                    panic!("undefined label ID in `jump`")
                }
            }
            Inst::CJump(cond, target_label) => {
                if self.registers.get(cond) == 1 {
                    if let Some(target_block) = self.block_table.get(target_label.0.as_str()) {
                        for inst in &target_block.insts {
                            self.interpret_inst(inst);
                        }
                    } else {
                        panic!("undefined target label ID in `cjump`")
                    }
                }
            }
            Inst::Branch(cond, true_label, false_label) => {
                if self.registers.get(cond) == 1 {
                    if let Some(target_block) = self.block_table.get(true_label.0.as_str()) {
                        for inst in &target_block.insts {
                            self.interpret_inst(inst);
                        }
                    } else {
                        panic!("undefined target label ID in `branch`")
                    }
                } else if let Some(target_block) = self.block_table.get(false_label.0.as_str()) {
                    for inst in &target_block.insts {
                        self.interpret_inst(inst);
                    }
                } else {
                    panic!("undefined target label ID in `branch`")
                }
            }
            Inst::SAdd(dst, lhs, rhs) => {
                let lhs_value = self.registers.get(lhs) as i64;
                let rhs_value = self.registers.get(rhs) as i64;
                let sum = lhs_value + rhs_value;
                self.registers.set(dst, sum as u64)
            }
        }
    }

    pub fn interpret(&mut self, blocks: &'a [Block]) {
        for block in blocks {
            self.block_table.insert(&block.label, block);
        }

        if let Some(main_block) = self.block_table.get("main") {
            for inst in &main_block.insts {
                self.interpret_inst(inst);
            }
        }
    }
}
