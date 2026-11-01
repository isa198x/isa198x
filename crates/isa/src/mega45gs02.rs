//! MEGA65 45GS02 instruction-set extension over the NMOS 6502.
//!
//! The 45GS02 is the CSG 4510 ([`crate::csg65ce02::CSG4502_SET`]) with two
//! prefixes. `$42 $42` (two `NEG`s) makes an instruction act on Q, the 32-bit
//! pseudo register formed by A, X, Y and Z. `$EA` (`NOP`/`EOM`) before a
//! base-page indirect Z-indexed instruction makes it read a 32-bit pointer,
//! written `[$nn],z`. The two combine as `$42 $42 $EA`. `SET` is cumulative: the
//! 4510's instructions and the 75 forms the prefixes make, so consumers need
//! only this one extension beside [`crate::mos6502::SET`].
//!
//! Each form is one fixed-slot [`Form`] whose opcode is the prefix bytes and
//! the 6502 opcode together. The table's 39 reserved `$42 $42` slots are not
//! instructions and are absent.
//!
//! Provenance: the MEGA65 User Guide's 45GS02 opcode table and addressing-mode
//! notes, distilled in `reference/by-topic/cpu-45gs02/`; every form is
//! cross-checked against ca65 V2.19.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const DP: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 1,
};
const ABS: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 2,
};
const NONE: &[Operand] = &[];
const ONE_DP: &[Operand] = &[DP];
const ONE_ABS: &[Operand] = &[ABS];

pub const SET: InstructionSet = InstructionSet {
    cpu: "MEGA65 45GS02 (extension)",
    endianness: Endianness::Little,
    instructions: &INSTRUCTIONS,
};

const fn form(opcode: &'static [u8], mode: &'static str, operands: &'static [Operand]) -> Form {
    Form {
        opcode,
        mode,
        operands,
        suffix: &[],
        cycles: Cycles::fixed(2),
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
const ADDITIONS: [Instruction; 24] = [
    inst!("ORQ", "OR Q", [form(&[0x42, 0x42, 0x05], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x0D], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x12], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0x12], "[indirect]", ONE_DP)]),
    inst!("ASLQ", "Shift Q left", [form(&[0x42, 0x42, 0x06], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x0A], "accumulator", NONE), form(&[0x42, 0x42, 0x0E], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x16], "zeropage,x", ONE_DP), form(&[0x42, 0x42, 0x1E], "absolute,x", ONE_ABS)]),
    inst!("ORA", "OR accumulator", [form(&[0xEA, 0x12], "[indirect],z", ONE_DP)]),
    inst!("INQ", "Increment Q", [form(&[0x42, 0x42, 0x1A], "accumulator", NONE), form(&[0x42, 0x42, 0xE6], "zeropage", ONE_DP), form(&[0x42, 0x42, 0xEE], "absolute", ONE_ABS), form(&[0x42, 0x42, 0xF6], "zeropage,x", ONE_DP), form(&[0x42, 0x42, 0xFE], "absolute,x", ONE_ABS)]),
    inst!("BITQ", "Test bits against Q", [form(&[0x42, 0x42, 0x24], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x2C], "absolute", ONE_ABS)]),
    inst!("ANDQ", "AND Q", [form(&[0x42, 0x42, 0x25], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x2D], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x32], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0x32], "[indirect]", ONE_DP)]),
    inst!("ROLQ", "Rotate Q left", [form(&[0x42, 0x42, 0x26], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x2A], "accumulator", NONE), form(&[0x42, 0x42, 0x2E], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x36], "zeropage,x", ONE_DP), form(&[0x42, 0x42, 0x3E], "absolute,x", ONE_ABS)]),
    inst!("AND", "AND accumulator", [form(&[0xEA, 0x32], "[indirect],z", ONE_DP)]),
    inst!("DEQ", "Decrement Q", [form(&[0x42, 0x42, 0x3A], "accumulator", NONE), form(&[0x42, 0x42, 0xC6], "zeropage", ONE_DP), form(&[0x42, 0x42, 0xCE], "absolute", ONE_ABS), form(&[0x42, 0x42, 0xD6], "zeropage,x", ONE_DP), form(&[0x42, 0x42, 0xDE], "absolute,x", ONE_ABS)]),
    inst!("ASRQ", "Arithmetic shift Q right", [form(&[0x42, 0x42, 0x43], "accumulator", NONE), form(&[0x42, 0x42, 0x44], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x54], "zeropage,x", ONE_DP)]),
    inst!("EORQ", "Exclusive-OR Q", [form(&[0x42, 0x42, 0x45], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x4D], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x52], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0x52], "[indirect]", ONE_DP)]),
    inst!("LSRQ", "Shift Q right", [form(&[0x42, 0x42, 0x46], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x4A], "accumulator", NONE), form(&[0x42, 0x42, 0x4E], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x56], "zeropage,x", ONE_DP), form(&[0x42, 0x42, 0x5E], "absolute,x", ONE_ABS)]),
    inst!("EOR", "Exclusive-OR accumulator", [form(&[0xEA, 0x52], "[indirect],z", ONE_DP)]),
    inst!("ADCQ", "Add Q with carry", [form(&[0x42, 0x42, 0x65], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x6D], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x72], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0x72], "[indirect]", ONE_DP)]),
    inst!("RORQ", "Rotate Q right", [form(&[0x42, 0x42, 0x66], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x6A], "accumulator", NONE), form(&[0x42, 0x42, 0x6E], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x76], "zeropage,x", ONE_DP), form(&[0x42, 0x42, 0x7E], "absolute,x", ONE_ABS)]),
    inst!("ADC", "Add with carry", [form(&[0xEA, 0x72], "[indirect],z", ONE_DP)]),
    inst!("STQ", "Store Q", [form(&[0x42, 0x42, 0x85], "zeropage", ONE_DP), form(&[0x42, 0x42, 0x8D], "absolute", ONE_ABS), form(&[0x42, 0x42, 0x92], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0x92], "[indirect]", ONE_DP)]),
    inst!("STA", "Store accumulator", [form(&[0xEA, 0x92], "[indirect],z", ONE_DP)]),
    inst!("LDQ", "Load Q", [form(&[0x42, 0x42, 0xA5], "zeropage", ONE_DP), form(&[0x42, 0x42, 0xAD], "absolute", ONE_ABS), form(&[0x42, 0x42, 0xB2], "(indirect),z", ONE_DP), form(&[0x42, 0x42, 0xEA, 0xB2], "[indirect],z", ONE_DP)]),
    inst!("LDA", "Load accumulator", [form(&[0xEA, 0xB2], "[indirect],z", ONE_DP)]),
    inst!("CMPQ", "Compare Q", [form(&[0x42, 0x42, 0xC5], "zeropage", ONE_DP), form(&[0x42, 0x42, 0xCD], "absolute", ONE_ABS), form(&[0x42, 0x42, 0xD2], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0xD2], "[indirect]", ONE_DP)]),
    inst!("CMP", "Compare accumulator", [form(&[0xEA, 0xD2], "[indirect],z", ONE_DP)]),
    inst!("SBCQ", "Subtract from Q with carry", [form(&[0x42, 0x42, 0xE5], "zeropage", ONE_DP), form(&[0x42, 0x42, 0xED], "absolute", ONE_ABS), form(&[0x42, 0x42, 0xF2], "(indirect)", ONE_DP), form(&[0x42, 0x42, 0xEA, 0xF2], "[indirect]", ONE_DP)]),
    inst!("SBC", "Subtract with carry", [form(&[0xEA, 0xF2], "[indirect],z", ONE_DP)]),
];

const INSTRUCTIONS: [Instruction; 115] =
    crate::csg65ce02::join(crate::csg65ce02::CSG4502_INSTRUCTIONS, ADDITIONS);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_prefixes_make_the_documented_forms() {
        let added: usize = ADDITIONS.iter().map(|i| i.forms.len()).sum();
        assert_eq!(added, 75);
        assert_eq!(
            SET.rows().count(),
            crate::csg65ce02::CSG4502_SET.rows().count() + 75
        );
        // The guide's own example: `LDQ [$12],Z` is `$42 $42 $EA $B2 $12`.
        assert_eq!(
            SET.find_form("LDQ", "[indirect],z")
                .expect("LDQ [zp],z")
                .opcode,
            &[0x42, 0x42, 0xEA, 0xB2]
        );
        assert_eq!(
            SET.find_form("STA", "[indirect],z")
                .expect("STA [zp],z")
                .opcode,
            &[0xEA, 0x92]
        );
        assert_eq!(
            SET.find_form("INQ", "accumulator").expect("INQ").opcode,
            &[0x42, 0x42, 0x1A]
        );
        // The 4510 is unchanged underneath.
        assert_eq!(
            SET.find_form("MAP", "implied").expect("MAP").opcode,
            &[0x5C]
        );
    }
}
