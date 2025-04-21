use ethnum::{I256, U256};
use std::collections::HashMap;

use crate::opcode::Opcode;

pub struct EVM {
    stack: Vec<U256>,
    pc: usize,
    bytecode: Vec<u8>,
    opcode_map: HashMap<u8, Opcode>,
}

impl EVM {
    pub fn new(hex_input: &str) -> Result<Self, &'static str> {
        // Remove "0x" prefix if present and validate hex string
        let cleaned_input = hex_input.strip_prefix("0x").unwrap_or(hex_input);
        if !cleaned_input.chars().all(|c| c.is_ascii_hexdigit()) || cleaned_input.len() % 2 != 0 {
            return Err("Invalid hex string");
        }

        // Convert hex string to Vec<u8>
        let bytecode = (0..cleaned_input.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&cleaned_input[i..i + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| "Failed to parse hex string")?;

        // Add Opcode to hashmap
        let mut opcode_map = HashMap::new();
        opcode_map.insert(0x00, Opcode::STOP);
        opcode_map.insert(0x01, Opcode::ADD);
        opcode_map.insert(0x02, Opcode::MUL);
        opcode_map.insert(0x03, Opcode::SUB);
        opcode_map.insert(0x04, Opcode::DIV);
        opcode_map.insert(0x05, Opcode::SDIV);
        opcode_map.insert(0x06, Opcode::MOD);
        opcode_map.insert(0x07, Opcode::SMOD);
        opcode_map.insert(0x08, Opcode::ADDMOD);
        opcode_map.insert(0x09, Opcode::MULMOD);
        opcode_map.insert(0x0A, Opcode::EXP);
        opcode_map.insert(0x0B, Opcode::SIGNEXTEND);
        opcode_map.insert(0x60, Opcode::PUSH1);
        opcode_map.insert(0x61, Opcode::PUSH2);
        opcode_map.insert(0x62, Opcode::PUSH3);
        opcode_map.insert(0x7F, Opcode::PUSH32);

        Ok(EVM {
            stack: Vec::new(),
            pc: 0,
            bytecode,
            opcode_map,
        })
    }

    fn get_bytes(&self, bytecode: Vec<u8>, from: usize, to: usize) -> Result<U256, &'static str> {
        let value = &bytecode[from..to];
        let hex_string: String = value.iter().map(|b| format!("{:02x}", b)).collect();
        let with_prefix = format!("0x{}", hex_string);
        match U256::from_str_hex(with_prefix.as_str()) {
            Ok(v) => return Ok(v),
            Err(_) => {
                return Err("Parse int error");
            }
        };
    }

    pub fn execute(&mut self) -> Result<(), &'static str> {
        while self.pc < self.bytecode.len() {
            let opcode_byte = self.bytecode[self.pc];
            let opcode = self.opcode_map.get(&opcode_byte).ok_or("Unknown opcode")?;

            match opcode {
                Opcode::STOP => {
                    return Ok(());
                }
                Opcode::ADD => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_add(b));
                    self.pc += 1;
                }
                Opcode::MUL => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_mul(b));
                    self.pc += 1;
                }
                Opcode::SUB => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_sub(b));
                    self.pc += 1;
                }
                Opcode::DIV => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let result = if b == 0 {
                        U256::ZERO
                    } else {
                        a.wrapping_div(b)
                    };
                    self.stack.push(result);

                    self.pc += 1;
                }
                Opcode::SDIV => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap().as_i256();
                    let b = self.stack.pop().unwrap().as_i256();
                    let result = if b == I256::ZERO {
                        U256::ZERO
                    } else {
                        (a / b).as_u256()
                    };
                    self.stack.push(result);
                    self.pc += 1;
                }
                Opcode::MOD => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    self.stack.push(a.wrapping_rem(b));
                    self.pc += 1;
                }
                Opcode::SMOD => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap().as_i256();
                    let b = self.stack.pop().unwrap().as_i256();
                    self.stack.push((a.wrapping_rem(b)).as_u256());
                    self.pc += 1;
                }
                Opcode::ADDMOD => {
                    if self.stack.len() < 3 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let n = self.stack.pop().unwrap();
                    self.stack.push((a.wrapping_add(b)).wrapping_rem(n));
                    self.pc += 1;
                }
                Opcode::MULMOD => {
                    if self.stack.len() < 3 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();
                    let n = self.stack.pop().unwrap();
                    self.stack.push((a.wrapping_mul(b)).wrapping_rem(n));
                    self.pc += 1;
                }
                Opcode::EXP => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let a = self.stack.pop().unwrap();
                    let b = self.stack.pop().unwrap();

                    self.stack.push(a.wrapping_pow(b.as_u32()));
                    self.pc += 1;
                }
                Opcode::SIGNEXTEND => {
                    if self.stack.len() < 2 {
                        return Err("Stack underflow");
                    }
                    let bytes = self.stack.pop().unwrap();
                    let value = self.stack.pop().unwrap();
                    let b = bytes.as_u64();

                    if b >= 31 {
                        self.stack.push(value);
                    } else {
                        let sign_bit_pos = (b + 1) * 8 - 1;

                        let mask = (U256::from(1 as u8) << ((b + 1) * 8)) - U256::from(1 as u8);
                        let truncated = value & mask;
                        let is_negative =
                            (value & (U256::from(1 as u8) << sign_bit_pos)) != U256::ZERO;

                        if is_negative {
                            let sign_extension = U256::MAX << (sign_bit_pos + 1);
                            self.stack.push(truncated | sign_extension);
                        } else {
                            self.stack.push(truncated);
                        }
                    }
                    self.pc += 1;
                }
                Opcode::PUSH1 => {
                    if self.pc + 1 >= self.bytecode.len() {
                        return Err("Incomplete PUSH1 data");
                    }
                    let value = U256::from(self.bytecode[self.pc + 1]);
                    self.stack.push(value);
                    self.pc += 2;
                }
                Opcode::PUSH2 => {
                    if self.pc + 2 >= self.bytecode.len() {
                        return Err("Incomplete PUSH1 data");
                    }

                    let value =
                        match self.get_bytes(self.bytecode.clone(), self.pc + 1, self.pc + 3) {
                            Ok(v) => v,
                            Err(_) => {
                                return Err("Erro PUSH2 data");
                            }
                        };
                    self.stack.push(value);
                    self.pc += 3;
                }
                Opcode::PUSH3 => {
                    if self.pc + 3 >= self.bytecode.len() {
                        return Err("Incomplete PUSH1 data");
                    }

                    let value =
                        match self.get_bytes(self.bytecode.clone(), self.pc + 1, self.pc + 4) {
                            Ok(v) => v,
                            Err(_) => {
                                return Err("Erro PUSH2 data");
                            }
                        };
                    self.stack.push(value);
                    self.pc += 4;
                }

                Opcode::PUSH32 => {
                    if self.pc + 32 >= self.bytecode.len() {
                        return Err("Incomplete PUSH1 data");
                    }

                    let value =
                        match self.get_bytes(self.bytecode.clone(), self.pc + 1, self.pc + 33) {
                            Ok(v) => v,
                            Err(_) => {
                                return Err("Erro PUSH2 data");
                            }
                        };
                    self.stack.push(value);
                    self.pc += 33;
                }
            }
        }
        Ok(())
    }

    pub fn get_stack(&self) -> &Vec<U256> {
        &self.stack
    }
}
