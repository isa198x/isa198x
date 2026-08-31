//! CSG 65CE02 and CSG 4502 instruction-set extensions over the NMOS 6502.
//!
//! The 65CE02 descends from the Rockwell 65C02 but replaces the eight plain
//! `(zp)` accumulator forms with `(zp),z`. `SET` therefore contains the
//! compatible CMOS forms, Rockwell bit operations, and all CE additions, but
//! deliberately omits those displaced forms. `CSG4502_SET` replaces reserved
//! `AUG` with `MAP` and adds the `EOM` spelling of opcode `$EA`.
//!
//! Provenance: CSG 65CE02 preliminary data sheet; ACME's independently
//! modelled processor tables; cc65's exhaustive 65CE02/4510 opcode tests.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const IMM8: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 1,
};
const IMM16: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 2,
};
const DP: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 1,
};
const ABS: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 2,
};
const REL8: Operand = Operand {
    kind: OperandKind::RelativePc,
    bytes: 1,
};
const REL16: Operand = Operand {
    kind: OperandKind::RelativePc,
    bytes: 2,
};
const NONE: &[Operand] = &[];

pub const SET: InstructionSet = InstructionSet {
    cpu: "CSG 65CE02 (extension)",
    endianness: Endianness::Little,
    instructions: &INSTRUCTIONS,
};

pub const CSG4502_SET: InstructionSet = InstructionSet {
    cpu: "CSG 4502/4510 (extension)",
    endianness: Endianness::Little,
    instructions: &CSG4502_INSTRUCTIONS,
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

macro_rules! implied {
    ($m:literal, $op:literal) => {
        inst!($m, "65CE02 operation", [form(&[$op], "implied", NONE)])
    };
}

macro_rules! long_branch {
    ($m:literal, $op:literal) => {
        inst!(
            $m,
            "Long relative branch",
            [form(&[$op], "relative16", &[REL16])]
        )
    };
}

const EMPTY: Instruction = Instruction {
    mnemonic: "",
    summary: "",
    forms: &[],
};

const fn join<const A: usize, const B: usize, const N: usize>(
    a: [Instruction; A],
    b: [Instruction; B],
) -> [Instruction; N] {
    assert!(N == A + B);
    let mut out = [EMPTY; N];
    let mut i = 0;
    while i < A {
        out[i] = a[i];
        i += 1;
    }
    let mut j = 0;
    while j < B {
        out[A + j] = b[j];
        j += 1;
    }
    out
}

// The first twelve plain-CMOS entries are compatible. The following eight
// `(indirect)` accumulator entries are intentionally displaced by `(zp),z`.
const CMOS_COMPATIBLE: [Instruction; 12] = [
    crate::mos65c02::CMOS_INSTRUCTIONS[0],
    crate::mos65c02::CMOS_INSTRUCTIONS[1],
    crate::mos65c02::CMOS_INSTRUCTIONS[2],
    crate::mos65c02::CMOS_INSTRUCTIONS[3],
    crate::mos65c02::CMOS_INSTRUCTIONS[4],
    crate::mos65c02::CMOS_INSTRUCTIONS[5],
    crate::mos65c02::CMOS_INSTRUCTIONS[6],
    crate::mos65c02::CMOS_INSTRUCTIONS[7],
    crate::mos65c02::CMOS_INSTRUCTIONS[8],
    crate::mos65c02::CMOS_INSTRUCTIONS[9],
    crate::mos65c02::CMOS_INSTRUCTIONS[10],
    crate::mos65c02::CMOS_INSTRUCTIONS[11],
];

#[rustfmt::skip]
const CE_ADDITIONS: [Instruction; 45] = [
    inst!("ORA", "OR accumulator", [form(&[0x12], "(indirect),z", &[DP])]),
    inst!("AND", "AND accumulator", [form(&[0x32], "(indirect),z", &[DP])]),
    inst!("EOR", "Exclusive-OR accumulator", [form(&[0x52], "(indirect),z", &[DP])]),
    inst!("ADC", "Add with carry", [form(&[0x72], "(indirect),z", &[DP])]),
    inst!("STA", "Store accumulator", [form(&[0x92], "(indirect),z", &[DP]), form(&[0x82], "(stack-indirect),y", &[DP])]),
    inst!("LDA", "Load accumulator", [form(&[0xB2], "(indirect),z", &[DP]), form(&[0xE2], "(stack-indirect),y", &[DP])]),
    inst!("CMP", "Compare accumulator", [form(&[0xD2], "(indirect),z", &[DP])]),
    inst!("SBC", "Subtract with carry", [form(&[0xF2], "(indirect),z", &[DP])]),
    inst!("JSR", "Jump to subroutine", [form(&[0x22], "indirect", &[ABS]), form(&[0x23], "(absolute,x)", &[ABS])]),
    inst!("STX", "Store X", [form(&[0x9B], "absolute,y", &[ABS])]),
    inst!("STY", "Store Y", [form(&[0x8B], "absolute,x", &[ABS])]),
    inst!("BRU", "Branch unconditional", [form(&[0x80], "relative", &[REL8])]),
    long_branch!("LBPL", 0x13), long_branch!("LBMI", 0x33),
    long_branch!("LBVC", 0x53), long_branch!("LBVS", 0x73),
    long_branch!("LBCC", 0x93), long_branch!("LBCS", 0xB3),
    long_branch!("LBNE", 0xD3), long_branch!("LBEQ", 0xF3),
    long_branch!("BSR", 0x63), long_branch!("LBRU", 0x83), long_branch!("LBRA", 0x83),
    inst!("ASR", "Arithmetic shift right", [form(&[0x43], "accumulator", NONE), form(&[0x44], "zeropage", &[DP]), form(&[0x54], "zeropage,x", &[DP])]),
    inst!("ASW", "Arithmetic shift word", [form(&[0xCB], "absolute", &[ABS])]),
    inst!("CPZ", "Compare Z", [form(&[0xC2], "immediate", &[IMM8]), form(&[0xD4], "zeropage", &[DP]), form(&[0xDC], "absolute", &[ABS])]),
    inst!("DEW", "Decrement word", [form(&[0xC3], "zeropage", &[DP])]),
    inst!("INW", "Increment word", [form(&[0xE3], "zeropage", &[DP])]),
    inst!("LDZ", "Load Z", [form(&[0xA3], "immediate", &[IMM8]), form(&[0xAB], "absolute", &[ABS]), form(&[0xBB], "absolute,x", &[ABS])]),
    inst!("PHW", "Push word", [form(&[0xF4], "immediate16", &[IMM16]), form(&[0xFC], "absolute", &[ABS])]),
    inst!("ROW", "Rotate word", [form(&[0xEB], "absolute", &[ABS])]),
    inst!("RTN", "Return and adjust stack", [form(&[0x62], "immediate", &[IMM8])]),
    implied!("CLE", 0x02), implied!("SEE", 0x03), implied!("INZ", 0x1B), implied!("DEZ", 0x3B),
    implied!("NEG", 0x42), implied!("TSY", 0x0B), implied!("TYS", 0x2B), implied!("TAZ", 0x4B),
    implied!("TAB", 0x5B), implied!("TZA", 0x6B), implied!("TBA", 0x7B), implied!("PHZ", 0xDB), implied!("PLZ", 0xFB),
];

const COMMON_AND_BITS: [Instruction; 44] =
    join(CMOS_COMPATIBLE, crate::mos65c02::ROCKWELL_ADDITIONS);
const CE_CORE: [Instruction; 89] = join(COMMON_AND_BITS, CE_ADDITIONS);
const AUG: [Instruction; 1] = [implied!("AUG", 0x5C)];
const INSTRUCTIONS: [Instruction; 90] = join(CE_CORE, AUG);

const CSG4502_ADDITIONS: [Instruction; 2] = [implied!("MAP", 0x5C), implied!("EOM", 0xEA)];
const CSG4502_INSTRUCTIONS: [Instruction; 91] = join(CE_CORE, CSG4502_ADDITIONS);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_keep_the_65ce02_and_4502_boundary() {
        assert_eq!(
            SET.find_form("LDA", "(indirect),z")
                .expect("LDA (zp),z")
                .opcode,
            &[0xB2]
        );
        assert!(SET.find_form("LDA", "(indirect)").is_none());
        assert_eq!(
            SET.find_form("PHW", "immediate16")
                .expect("PHW immediate")
                .opcode,
            &[0xF4]
        );
        assert_eq!(
            SET.find_form("LBRA", "relative16").expect("LBRA").opcode,
            &[0x83]
        );
        assert!(SET.has_mnemonic("AUG"));
        assert!(!SET.has_mnemonic("MAP"));
        assert!(!CSG4502_SET.has_mnemonic("AUG"));
        assert_eq!(
            CSG4502_SET.find_form("MAP", "implied").expect("MAP").opcode,
            &[0x5C]
        );
        assert_eq!(
            CSG4502_SET.find_form("EOM", "implied").expect("EOM").opcode,
            &[0xEA]
        );
    }
}
