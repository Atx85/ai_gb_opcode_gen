use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg8 { A, B, C, D, E, H, L }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg16 { BC, DE, HL, SP, AF }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagCondition { NZ, Z, NC, C }

#[derive(Deserialize, Debug)]
pub struct OpcodeTable {
    pub unprefixed: HashMap<String, OpcodeInfo>,
    pub cbprefixed: HashMap<String, OpcodeInfo>
}   

#[derive(Deserialize, Debug)]
pub struct OpcodeInfo {
    pub mnemonic: String,
    pub bytes: u8,
    pub cycles: Vec<u32>,
    pub operands: Vec<OperandInfo>,
    pub immediate: bool,
    pub flags: FlagEffects,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlagEffects {
    #[serde(rename = "Z")]
    pub z: FlagEffect,
    #[serde(rename = "N")]
    pub n: FlagEffect,
    #[serde(rename = "H")]
    pub h: FlagEffect,
    #[serde(rename = "C")]
    pub c: FlagEffect,
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagEffect {
    #[serde(rename = "-")]
    Unchanged,
    #[serde(rename = "0")]
    Clear,
    #[serde(rename = "1")]
    Set,
    #[serde(alias = "Z", alias = "N", alias = "H", alias = "C")]
    Computed,
}

#[derive(Deserialize, Debug)]
pub struct OperandInfo {
    pub name: String,
    #[serde(default)]
    pub bytes: Option<u8>,
    pub immediate: bool,
    #[serde(default)]
    pub increment: Option<bool>,
    #[serde(default)]
    pub decrement: Option<bool>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Reg8(Reg8),
    Reg16(Reg16),
    IndirectHL { inc: bool, dec: bool },
    IndirectReg16(Reg16),
    IndirectC,
    Imm8(u8),
    Imm16(u16),
    Rel8(i8),
    Condition(FlagCondition),
    Bit(u8),
    RstTarget(u8),
}

impl OperandInfo {
    pub fn to_enum(&self) -> Operand {
        match self.name.as_str() {
            "A" => Operand::Reg8(Reg8::A),
            "B" => Operand::Reg8(Reg8::B),
            "C" if self.immediate => Operand::Reg8(Reg8::C),
            "D" => Operand::Reg8(Reg8::D),
            "E" => Operand::Reg8(Reg8::E),
            "H" => Operand::Reg8(Reg8::H),
            "L" => Operand::Reg8(Reg8::L),

            "BC" if self.immediate => Operand::Reg16(Reg16::BC),
            "DE" if self.immediate => Operand::Reg16(Reg16::DE),
            "HL" if self.immediate => Operand::Reg16(Reg16::HL),
            "SP" if self.immediate => Operand::Reg16(Reg16::SP),
            "AF" if self.immediate => Operand::Reg16(Reg16::AF),

            "HL" if !self.immediate => {
                Operand::IndirectHL {
                    inc: self.increment.unwrap_or(false),
                    dec: self.decrement.unwrap_or(false),
                }
            }
            "BC" => Operand::IndirectReg16(Reg16::BC),
            "DE" => Operand::IndirectReg16(Reg16::DE),
            "C" => Operand::IndirectC,

            "n8" | "a8" => Operand::Imm8(0),
            "n16" | "a16" => Operand::Imm16(0),
            "e8" => Operand::Rel8(0),

            "NZ" => Operand::Condition(FlagCondition::NZ),
            "Z" => Operand::Condition(FlagCondition::Z),
            "NC" => Operand::Condition(FlagCondition::NC),
            
            bit_str if bit_str.len() == 1 && bit_str.chars().all(|c| c.is_ascii_digit()) => {
                Operand::Bit(bit_str.parse().unwrap())
            }

            rst if rst.starts_with('$') => {
                let val = u8::from_str_radix(&rst[1..], 16).unwrap();
                Operand::RstTarget(val)
            }

            _ => panic!("Unknown or unhandled operand name: {}", self.name),
        }
    }
}

fn main() {
    let path = "./Opcodes.json";
    let data = fs::read_to_string(path).expect("Unable to read file");
    let table: OpcodeTable = serde_json::from_str(&data).expect("Unable to parse");

    let mut code = String::new();
    code.push_str("// ==========================================\n");
    code.push_str("// AUTO-GENERATED CODE - DO NOT EDIT MANUALLY\n");
    code.push_str("// ==========================================\n\n");

    // Copy your required enums into the generated file context
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum Reg8 { A, B, C, D, E, H, L }\n\n");
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum Reg16 { BC, DE, HL, SP, AF }\n\n");
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum FlagCondition { NZ, Z, NC, C }\n\n");

    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum FlagEffect { Unchanged, Clear, Set, Computed }\n\n");
    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub struct FlagEffects { pub z: FlagEffect, pub n: FlagEffect, pub h: FlagEffect, pub c: FlagEffect }\n\n");

    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum Operand {\n");
    code.push_str("    Reg8(Reg8),\n    Reg16(Reg16),\n");
    code.push_str("    IndirectHL { inc: bool, dec: bool },\n");
    code.push_str("    IndirectReg16(Reg16),\n    IndirectC,\n");
    code.push_str("    Imm8(u8), Imm16(u16), Rel8(i8),\n");
    code.push_str("    Condition(FlagCondition),\n");
    code.push_str("    Bit(u8), RstTarget(u8),\n");
    code.push_str("}\n\n");

    // Collect and generate the Mnemonic enum
    let mut mnemonics: Vec<String> = table.unprefixed.values()
        .chain(table.cbprefixed.values())
        .map(|op| op.mnemonic.clone())
        .collect();
    mnemonics.sort();
    mnemonics.dedup();

    code.push_str("#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n");
    code.push_str("pub enum Mnemonic {\n");
    for m in &mnemonics {
        let safe_name = if m.starts_with("ILLEGAL") { m.replace('$', "X") } else { m.clone() };
        code.push_str(&format!("    {},\n", safe_name));
    }
    code.push_str("}\n\n");

    // Define the static metadata layout struct
    code.push_str("pub struct OpcodeInfoGen {\n");
    code.push_str("    pub mnemonic: Mnemonic,\n");
    code.push_str("    pub bytes: u8,\n");
    code.push_str("    pub cycles: &'static [u32],\n");
    code.push_str("    pub operands: &'static [Operand],\n");
    code.push_str("    pub flags: FlagEffects,\n");
    code.push_str("}\n\n");

    // Helper closure to write out flat 256-element opcode arrays
    let mut write_opcode_array = |code_str: &mut String, map: &HashMap<String, OpcodeInfo>, name: &str| {
        code_str.push_str(&format!("pub const {}: [OpcodeInfoGen; 256] = [\n", name));
        for i in 0..=255 {
            let key = format!("0x{:02X}", i);
            if let Some(info) = map.get(&key) {
                let safe_mnemonic = if info.mnemonic.starts_with("ILLEGAL") {
                    info.mnemonic.replace('$', "X")
                } else {
                    info.mnemonic.clone()
                };

                let cycles_str = format!("&{:?}", info.cycles);
                let flags_str = format!(
                    "FlagEffects {{ z: FlagEffect::{:?}, n: FlagEffect::{:?}, h: FlagEffect::{:?}, c: FlagEffect::{:?} }}",
                    info.flags.z, info.flags.n, info.flags.h, info.flags.c
                );

                // Convert each OperandInfo into its Rust enum representation textually
                let mut operands_tokens = String::from("&[");
                for op in &info.operands {
                    let enum_val = op.to_enum();
                    match enum_val {
                        Operand::Reg8(r) => operands_tokens.push_str(&format!("Operand::Reg8(Reg8::{:?}), ", r)),
                        Operand::Reg16(r) => operands_tokens.push_str(&format!("Operand::Reg16(Reg16::{:?}), ", r)),
                        Operand::IndirectHL { inc, dec } => operands_tokens.push_str(&format!("Operand::IndirectHL {{ inc: {}, dec: {} }}, ", inc, dec)),
                        Operand::IndirectReg16(r) => operands_tokens.push_str(&format!("Operand::IndirectReg16(Reg16::{:?}), ", r)),
                        Operand::IndirectC => operands_tokens.push_str("Operand::IndirectC, "),
                        Operand::Imm8(v) => operands_tokens.push_str(&format!("Operand::Imm8({}), ", v)),
                        Operand::Imm16(v) => operands_tokens.push_str(&format!("Operand::Imm16({}), ", v)),
                        Operand::Rel8(v) => operands_tokens.push_str(&format!("Operand::Rel8({}), ", v)),
                        Operand::Condition(c) => operands_tokens.push_str(&format!("Operand::Condition(FlagCondition::{:?}), ", c)),
                        Operand::Bit(b) => operands_tokens.push_str(&format!("Operand::Bit({}), ", b)),
                        Operand::RstTarget(t) => operands_tokens.push_str(&format!("Operand::RstTarget({}), ", t)),
                    }
                }
                operands_tokens.push_str("]");

                code_str.push_str(&format!(
                    "    /* {} */ OpcodeInfoGen {{ mnemonic: Mnemonic::{}, bytes: {}, cycles: {}, operands: {}, flags: {} }},\n",
                    key, safe_mnemonic, info.bytes, cycles_str, operands_tokens, flags_str
                ));
            } else {
                code_str.push_str(&format!(
                    "    /* {} */ OpcodeInfoGen {{ mnemonic: Mnemonic::NOP, bytes: 1, cycles: &[4], operands: &[], flags: FlagEffects {{ z: FlagEffect::Unchanged, n: FlagEffect::Unchanged, h: FlagEffect::Unchanged, c: FlagEffect::Unchanged }} }},\n",
                    key
                ));
            }
        }
        code_str.push_str("];\n\n");
    };

    write_opcode_array(&mut code, &table.unprefixed, "UNPREFIXED_OPCODES");
    write_opcode_array(&mut code, &table.cbprefixed, "CB_PREFIXED_OPCODES");

    fs::write("./opcodes_gen.rs", code).expect("Failed to write opcodes_gen.rs");
    println!("Successfully generated opcodes_gen.rs with full type safety!");
}