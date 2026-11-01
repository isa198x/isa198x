//! Steve Wozniak's SWEET 16, a 16-bit "metaprocessor" interpreted by 6502 code
//! in the Apple II's Integer BASIC ROM.
//!
//! SWEET 16 has sixteen 16-bit registers, `R0` to `R15`. A register
//! instruction is one byte, its operation in the high nibble (`$1`–`$F`) and
//! its register in the low one, so each is one fixed-slot [`Form`] per
//! register, its mode naming the register (`r5`) or the register used as a
//! pointer (`@r5`), as the 8048's forms do. `SET Rn` takes a two-byte constant,
//! low byte first. The non-register instructions are `$00`–`$0C`; the branches
//! among them take a one-byte displacement counted from the byte after it.
//! `$0D`–`$0F` are unassigned.
//!
//! This is a set of its own, not an extension of the 6502's: SWEET 16 code is
//! data the 6502 interprets.
//!
//! Provenance: Wozniak's opcode summary and per-instruction encodings from
//! "SWEET 16: The 6502 Dream Machine" (BYTE, November 1977), with his
//! interpreter source, as held in `reference/by-topic/cpu-sweet16/`.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const IMM16: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 2,
};
const REL: Operand = Operand {
    kind: OperandKind::RelativePc,
    bytes: 1,
};
const NONE: &[Operand] = &[];
const ONE_REL: &[Operand] = &[REL];
/// `SET Rn,constant`: the constant, low byte first.
const R_IMM16: &[Operand] = &[IMM16];

pub const SET: InstructionSet = InstructionSet {
    cpu: "SWEET 16",
    endianness: Endianness::Little,
    instructions: INSTRUCTIONS,
};

const fn form(opcode: &'static [u8], mode: &'static str, operands: &'static [Operand]) -> Form {
    Form {
        opcode,
        mode,
        operands,
        suffix: &[],
        cycles: Cycles::fixed(1),
        flags: "",
        undocumented: false,
    }
}

macro_rules! inst {
    ($m:literal, $s:literal, [ $($f:expr),* $(,)? ]) => {
        Instruction { mnemonic: $m, summary: $s, forms: &[ $($f),* ] }
    };
}

#[rustfmt::skip]
const INSTRUCTIONS: &[Instruction] = &[
    inst!("SET", "Set register to a constant", [
        form(&[0x10], "r0,immediate", R_IMM16),
        form(&[0x11], "r1,immediate", R_IMM16),
        form(&[0x12], "r2,immediate", R_IMM16),
        form(&[0x13], "r3,immediate", R_IMM16),
        form(&[0x14], "r4,immediate", R_IMM16),
        form(&[0x15], "r5,immediate", R_IMM16),
        form(&[0x16], "r6,immediate", R_IMM16),
        form(&[0x17], "r7,immediate", R_IMM16),
        form(&[0x18], "r8,immediate", R_IMM16),
        form(&[0x19], "r9,immediate", R_IMM16),
        form(&[0x1A], "r10,immediate", R_IMM16),
        form(&[0x1B], "r11,immediate", R_IMM16),
        form(&[0x1C], "r12,immediate", R_IMM16),
        form(&[0x1D], "r13,immediate", R_IMM16),
        form(&[0x1E], "r14,immediate", R_IMM16),
        form(&[0x1F], "r15,immediate", R_IMM16),
    ]),
    inst!("LD", "Load R0, directly or indirectly", [
        form(&[0x20], "r0", NONE),
        form(&[0x21], "r1", NONE),
        form(&[0x22], "r2", NONE),
        form(&[0x23], "r3", NONE),
        form(&[0x24], "r4", NONE),
        form(&[0x25], "r5", NONE),
        form(&[0x26], "r6", NONE),
        form(&[0x27], "r7", NONE),
        form(&[0x28], "r8", NONE),
        form(&[0x29], "r9", NONE),
        form(&[0x2A], "r10", NONE),
        form(&[0x2B], "r11", NONE),
        form(&[0x2C], "r12", NONE),
        form(&[0x2D], "r13", NONE),
        form(&[0x2E], "r14", NONE),
        form(&[0x2F], "r15", NONE),
        form(&[0x40], "@r0", NONE),
        form(&[0x41], "@r1", NONE),
        form(&[0x42], "@r2", NONE),
        form(&[0x43], "@r3", NONE),
        form(&[0x44], "@r4", NONE),
        form(&[0x45], "@r5", NONE),
        form(&[0x46], "@r6", NONE),
        form(&[0x47], "@r7", NONE),
        form(&[0x48], "@r8", NONE),
        form(&[0x49], "@r9", NONE),
        form(&[0x4A], "@r10", NONE),
        form(&[0x4B], "@r11", NONE),
        form(&[0x4C], "@r12", NONE),
        form(&[0x4D], "@r13", NONE),
        form(&[0x4E], "@r14", NONE),
        form(&[0x4F], "@r15", NONE),
    ]),
    inst!("ST", "Store R0, directly or indirectly", [
        form(&[0x30], "r0", NONE),
        form(&[0x31], "r1", NONE),
        form(&[0x32], "r2", NONE),
        form(&[0x33], "r3", NONE),
        form(&[0x34], "r4", NONE),
        form(&[0x35], "r5", NONE),
        form(&[0x36], "r6", NONE),
        form(&[0x37], "r7", NONE),
        form(&[0x38], "r8", NONE),
        form(&[0x39], "r9", NONE),
        form(&[0x3A], "r10", NONE),
        form(&[0x3B], "r11", NONE),
        form(&[0x3C], "r12", NONE),
        form(&[0x3D], "r13", NONE),
        form(&[0x3E], "r14", NONE),
        form(&[0x3F], "r15", NONE),
        form(&[0x50], "@r0", NONE),
        form(&[0x51], "@r1", NONE),
        form(&[0x52], "@r2", NONE),
        form(&[0x53], "@r3", NONE),
        form(&[0x54], "@r4", NONE),
        form(&[0x55], "@r5", NONE),
        form(&[0x56], "@r6", NONE),
        form(&[0x57], "@r7", NONE),
        form(&[0x58], "@r8", NONE),
        form(&[0x59], "@r9", NONE),
        form(&[0x5A], "@r10", NONE),
        form(&[0x5B], "@r11", NONE),
        form(&[0x5C], "@r12", NONE),
        form(&[0x5D], "@r13", NONE),
        form(&[0x5E], "@r14", NONE),
        form(&[0x5F], "@r15", NONE),
    ]),
    inst!("LDD", "Load double indirect, then increment twice", [
        form(&[0x60], "@r0", NONE),
        form(&[0x61], "@r1", NONE),
        form(&[0x62], "@r2", NONE),
        form(&[0x63], "@r3", NONE),
        form(&[0x64], "@r4", NONE),
        form(&[0x65], "@r5", NONE),
        form(&[0x66], "@r6", NONE),
        form(&[0x67], "@r7", NONE),
        form(&[0x68], "@r8", NONE),
        form(&[0x69], "@r9", NONE),
        form(&[0x6A], "@r10", NONE),
        form(&[0x6B], "@r11", NONE),
        form(&[0x6C], "@r12", NONE),
        form(&[0x6D], "@r13", NONE),
        form(&[0x6E], "@r14", NONE),
        form(&[0x6F], "@r15", NONE),
    ]),
    inst!("STD", "Store double indirect, then increment twice", [
        form(&[0x70], "@r0", NONE),
        form(&[0x71], "@r1", NONE),
        form(&[0x72], "@r2", NONE),
        form(&[0x73], "@r3", NONE),
        form(&[0x74], "@r4", NONE),
        form(&[0x75], "@r5", NONE),
        form(&[0x76], "@r6", NONE),
        form(&[0x77], "@r7", NONE),
        form(&[0x78], "@r8", NONE),
        form(&[0x79], "@r9", NONE),
        form(&[0x7A], "@r10", NONE),
        form(&[0x7B], "@r11", NONE),
        form(&[0x7C], "@r12", NONE),
        form(&[0x7D], "@r13", NONE),
        form(&[0x7E], "@r14", NONE),
        form(&[0x7F], "@r15", NONE),
    ]),
    inst!("POP", "Decrement, then load R0 indirect", [
        form(&[0x80], "@r0", NONE),
        form(&[0x81], "@r1", NONE),
        form(&[0x82], "@r2", NONE),
        form(&[0x83], "@r3", NONE),
        form(&[0x84], "@r4", NONE),
        form(&[0x85], "@r5", NONE),
        form(&[0x86], "@r6", NONE),
        form(&[0x87], "@r7", NONE),
        form(&[0x88], "@r8", NONE),
        form(&[0x89], "@r9", NONE),
        form(&[0x8A], "@r10", NONE),
        form(&[0x8B], "@r11", NONE),
        form(&[0x8C], "@r12", NONE),
        form(&[0x8D], "@r13", NONE),
        form(&[0x8E], "@r14", NONE),
        form(&[0x8F], "@r15", NONE),
    ]),
    inst!("STP", "Decrement, then store R0 indirect", [
        form(&[0x90], "@r0", NONE),
        form(&[0x91], "@r1", NONE),
        form(&[0x92], "@r2", NONE),
        form(&[0x93], "@r3", NONE),
        form(&[0x94], "@r4", NONE),
        form(&[0x95], "@r5", NONE),
        form(&[0x96], "@r6", NONE),
        form(&[0x97], "@r7", NONE),
        form(&[0x98], "@r8", NONE),
        form(&[0x99], "@r9", NONE),
        form(&[0x9A], "@r10", NONE),
        form(&[0x9B], "@r11", NONE),
        form(&[0x9C], "@r12", NONE),
        form(&[0x9D], "@r13", NONE),
        form(&[0x9E], "@r14", NONE),
        form(&[0x9F], "@r15", NONE),
    ]),
    inst!("ADD", "Add a register to R0", [
        form(&[0xA0], "r0", NONE),
        form(&[0xA1], "r1", NONE),
        form(&[0xA2], "r2", NONE),
        form(&[0xA3], "r3", NONE),
        form(&[0xA4], "r4", NONE),
        form(&[0xA5], "r5", NONE),
        form(&[0xA6], "r6", NONE),
        form(&[0xA7], "r7", NONE),
        form(&[0xA8], "r8", NONE),
        form(&[0xA9], "r9", NONE),
        form(&[0xAA], "r10", NONE),
        form(&[0xAB], "r11", NONE),
        form(&[0xAC], "r12", NONE),
        form(&[0xAD], "r13", NONE),
        form(&[0xAE], "r14", NONE),
        form(&[0xAF], "r15", NONE),
    ]),
    inst!("SUB", "Subtract a register from R0", [
        form(&[0xB0], "r0", NONE),
        form(&[0xB1], "r1", NONE),
        form(&[0xB2], "r2", NONE),
        form(&[0xB3], "r3", NONE),
        form(&[0xB4], "r4", NONE),
        form(&[0xB5], "r5", NONE),
        form(&[0xB6], "r6", NONE),
        form(&[0xB7], "r7", NONE),
        form(&[0xB8], "r8", NONE),
        form(&[0xB9], "r9", NONE),
        form(&[0xBA], "r10", NONE),
        form(&[0xBB], "r11", NONE),
        form(&[0xBC], "r12", NONE),
        form(&[0xBD], "r13", NONE),
        form(&[0xBE], "r14", NONE),
        form(&[0xBF], "r15", NONE),
    ]),
    inst!("POPD", "Decrement twice, then load double indirect", [
        form(&[0xC0], "@r0", NONE),
        form(&[0xC1], "@r1", NONE),
        form(&[0xC2], "@r2", NONE),
        form(&[0xC3], "@r3", NONE),
        form(&[0xC4], "@r4", NONE),
        form(&[0xC5], "@r5", NONE),
        form(&[0xC6], "@r6", NONE),
        form(&[0xC7], "@r7", NONE),
        form(&[0xC8], "@r8", NONE),
        form(&[0xC9], "@r9", NONE),
        form(&[0xCA], "@r10", NONE),
        form(&[0xCB], "@r11", NONE),
        form(&[0xCC], "@r12", NONE),
        form(&[0xCD], "@r13", NONE),
        form(&[0xCE], "@r14", NONE),
        form(&[0xCF], "@r15", NONE),
    ]),
    inst!("CPR", "Compare R0 with a register", [
        form(&[0xD0], "r0", NONE),
        form(&[0xD1], "r1", NONE),
        form(&[0xD2], "r2", NONE),
        form(&[0xD3], "r3", NONE),
        form(&[0xD4], "r4", NONE),
        form(&[0xD5], "r5", NONE),
        form(&[0xD6], "r6", NONE),
        form(&[0xD7], "r7", NONE),
        form(&[0xD8], "r8", NONE),
        form(&[0xD9], "r9", NONE),
        form(&[0xDA], "r10", NONE),
        form(&[0xDB], "r11", NONE),
        form(&[0xDC], "r12", NONE),
        form(&[0xDD], "r13", NONE),
        form(&[0xDE], "r14", NONE),
        form(&[0xDF], "r15", NONE),
    ]),
    inst!("INR", "Increment a register", [
        form(&[0xE0], "r0", NONE),
        form(&[0xE1], "r1", NONE),
        form(&[0xE2], "r2", NONE),
        form(&[0xE3], "r3", NONE),
        form(&[0xE4], "r4", NONE),
        form(&[0xE5], "r5", NONE),
        form(&[0xE6], "r6", NONE),
        form(&[0xE7], "r7", NONE),
        form(&[0xE8], "r8", NONE),
        form(&[0xE9], "r9", NONE),
        form(&[0xEA], "r10", NONE),
        form(&[0xEB], "r11", NONE),
        form(&[0xEC], "r12", NONE),
        form(&[0xED], "r13", NONE),
        form(&[0xEE], "r14", NONE),
        form(&[0xEF], "r15", NONE),
    ]),
    inst!("DCR", "Decrement a register", [
        form(&[0xF0], "r0", NONE),
        form(&[0xF1], "r1", NONE),
        form(&[0xF2], "r2", NONE),
        form(&[0xF3], "r3", NONE),
        form(&[0xF4], "r4", NONE),
        form(&[0xF5], "r5", NONE),
        form(&[0xF6], "r6", NONE),
        form(&[0xF7], "r7", NONE),
        form(&[0xF8], "r8", NONE),
        form(&[0xF9], "r9", NONE),
        form(&[0xFA], "r10", NONE),
        form(&[0xFB], "r11", NONE),
        form(&[0xFC], "r12", NONE),
        form(&[0xFD], "r13", NONE),
        form(&[0xFE], "r14", NONE),
        form(&[0xFF], "r15", NONE),
    ]),
    inst!("RTN", "Return to 6502 code", [form(&[0x00], "implied", NONE)]),
    inst!("BR", "Branch always", [form(&[0x01], "relative", ONE_REL)]),
    inst!("BNC", "Branch if no carry", [form(&[0x02], "relative", ONE_REL)]),
    inst!("BC", "Branch if carry", [form(&[0x03], "relative", ONE_REL)]),
    inst!("BP", "Branch if plus", [form(&[0x04], "relative", ONE_REL)]),
    inst!("BM", "Branch if minus", [form(&[0x05], "relative", ONE_REL)]),
    inst!("BZ", "Branch if zero", [form(&[0x06], "relative", ONE_REL)]),
    inst!("BNZ", "Branch if not zero", [form(&[0x07], "relative", ONE_REL)]),
    inst!("BM1", "Branch if minus one", [form(&[0x08], "relative", ONE_REL)]),
    inst!("BNM1", "Branch if not minus one", [form(&[0x09], "relative", ONE_REL)]),
    inst!("BK", "Break to the 6502", [form(&[0x0A], "implied", NONE)]),
    inst!("RS", "Return from a SWEET 16 subroutine", [form(&[0x0B], "implied", NONE)]),
    inst!("BS", "Branch to a SWEET 16 subroutine", [form(&[0x0C], "relative", ONE_REL)]),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Every code from `$00` to `$FF` but the three unassigned, once each.
    #[test]
    fn every_assigned_code_once() {
        let mut seen = std::collections::BTreeSet::new();
        for instruction in INSTRUCTIONS {
            for form in instruction.forms {
                assert!(seen.insert(form.opcode[0]), "{:02X} twice", form.opcode[0]);
            }
        }
        assert_eq!(seen.len(), 256 - 3);
        assert!(![0x0D, 0x0E, 0x0F].iter().any(|code| seen.contains(code)));
    }

    #[test]
    fn woz_s_examples() {
        // `15 34 A0  SET R5 $A034`.
        let set = SET.find_form("SET", "r5,immediate").expect("SET R5");
        assert_eq!(set.opcode, &[0x15]);
        assert_eq!(set.operands, R_IMM16);
        assert_eq!(SET.find_form("LD", "@r5").expect("LD @R5").opcode, &[0x45]);
        assert_eq!(SET.find_form("LD", "r5").expect("LD R5").opcode, &[0x25]);
        assert_eq!(
            SET.find_form("POPD", "@r12").expect("POPD @R12").opcode,
            &[0xCC]
        );
        assert_eq!(SET.find_form("BS", "relative").expect("BS").opcode, &[0x0C]);
    }
}
