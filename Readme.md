# Game Boy Emulator - CPU Opcode Metadata Generator

This project builds an extremely efficient, type-safe Game Boy emulator in Rust. Rather than manually typing out or parsing JSON at runtime, this project uses a code-generation script to translate raw opcode definitions into native Rust enums and static lookup tables.

## 📥 Data Source

The instruction set architecture (ISA) metadata used to generate these structures is sourced directly from the [gb-opcodes](https://gbdev.io/gb-opcodes/Opcodes.json) repository maintained by the GBDev community:

* **Source File:** `Opcodes.json`
* **Repository URL:** [https://github.com/gbdev/gb-opcodes](https://github.com/gbdev/gb-opcodes)
* **License/Usage:** Provided according to standard open-source emulation development guidelines.

---

## 🛠️ How It Works

1. **`Opcodes.json`** is read at build/generation time.
2. The Rust generation script maps raw string-based operands, mnemonics, and flag effects into strongly-typed Rust enums and structs (`Reg8`, `Reg16`, `Operand`, `Mnemonic`, `FlagEffect`, and `FlagEffects`).
3. It writes out **`opcodes_gen.rs`**, which contains flat, zero-overhead $O(1)$ static arrays (`UNPREFIXED_OPCODES` and `CB_PREFIXED_OPCODES`) for all 512 Game Boy instructions. Each opcode includes the effect on Z, N, H, and C: unchanged, cleared, set, or calculated by the instruction.

## 🚀 Usage

Include the generated file in your emulator project:

```rust
mod opcodes_gen;
use opcodes_gen::{UNPREFIXED_OPCODES, Mnemonic};

fn main() {
    // Access static metadata instantly with zero runtime overhead
    let op = &UNPREFIXED_OPCODES[0x16];
    println!("Opcode 0x16 Mnemonic: {:?}", op.mnemonic);
}