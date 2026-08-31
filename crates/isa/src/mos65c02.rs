//! CMOS 65C02 instruction-set additions over the documented NMOS 6502.
//!
//! This is an additive [`InstructionSet`]: consumers layer it on
//! [`crate::mos6502::SET`]. It contains the 27 newly documented opcode forms
//! common to the plain 65C02 profile. [`ROCKWELL_SET`] adds the 32 bit-operation
//! forms, and [`WDC_SET`] adds `WAI`/`STP`. The descendant sets are cumulative
//! so each remains one extension over [`crate::mos6502::SET`].
//!
//! **Provenance.** The encodings are distilled from *Programming the 65816,
//! including the 6502, 65C02 and 65802* in the primary reference library and
//! agree with the independently modelled 65C02 baseline in Hudson Soft's
//! HuC6280 Software Manual. Representative forms are differentially checked
//! against ACME.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const IMM8: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 1,
};
const DP: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 1,
};
const ABS: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 2,
};
const REL: Operand = Operand {
    kind: OperandKind::RelativePc,
    bytes: 1,
};
const NONE: &[Operand] = &[];
const ONE_IMM8: &[Operand] = &[IMM8];
const ONE_DP: &[Operand] = &[DP];
const ONE_ABS: &[Operand] = &[ABS];
const ONE_REL: &[Operand] = &[REL];

pub const SET: InstructionSet = InstructionSet {
    cpu: "CMOS 65C02 (extension)",
    endianness: Endianness::Little,
    instructions: &CMOS_INSTRUCTIONS,
};

/// Rockwell R65C02: plain CMOS additions plus 32 bit-operation forms.
pub const ROCKWELL_SET: InstructionSet = InstructionSet {
    cpu: "Rockwell R65C02 (extension)",
    endianness: Endianness::Little,
    instructions: &ROCKWELL_INSTRUCTIONS,
};

/// WDC W65C02: Rockwell profile plus `WAI` and `STP`.
pub const WDC_SET: InstructionSet = InstructionSet {
    cpu: "WDC W65C02 (extension)",
    endianness: Endianness::Little,
    instructions: &WDC_INSTRUCTIONS,
};

const fn form(
    opcode: &'static [u8],
    mode: &'static str,
    operands: &'static [Operand],
    cycles: Cycles,
    flags: &'static str,
) -> Form {
    Form {
        opcode,
        mode,
        operands,
        suffix: &[],
        cycles,
        flags,
        undocumented: false,
    }
}

macro_rules! inst {
    ($mnemonic:literal, $summary:literal, [ $($form:expr),* $(,)? ]) => {
        Instruction { mnemonic: $mnemonic, summary: $summary, forms: &[ $($form),* ] }
    };
}

macro_rules! bit_zp {
    ($mnemonic:literal, $op:literal) => {
        inst!(
            $mnemonic,
            "Reset/set memory bit",
            [form(&[$op], "zeropage", ONE_DP, Cycles::fixed(5), "")]
        )
    };
}

macro_rules! bit_branch {
    ($mnemonic:literal, $op:literal) => {
        inst!(
            $mnemonic,
            "Branch on memory bit",
            [form(
                &[$op],
                "zeropage,relative",
                &[DP, REL],
                Cycles::branch(5),
                ""
            )]
        )
    };
}

const EMPTY: Instruction = Instruction {
    mnemonic: "",
    summary: "",
    forms: &[],
};

const fn join<const A: usize, const B: usize, const N: usize>(
    left: [Instruction; A],
    right: [Instruction; B],
) -> [Instruction; N] {
    assert!(N == A + B);
    let mut joined = [EMPTY; N];
    let mut i = 0;
    while i < A {
        joined[i] = left[i];
        i += 1;
    }
    let mut j = 0;
    while j < B {
        joined[A + j] = right[j];
        j += 1;
    }
    joined
}

#[rustfmt::skip]
pub(crate) const CMOS_INSTRUCTIONS: [Instruction; 20] = [
    inst!("PHX", "Push X", [form(&[0xDA], "implied", NONE, Cycles::fixed(3), "")]),
    inst!("PHY", "Push Y", [form(&[0x5A], "implied", NONE, Cycles::fixed(3), "")]),
    inst!("PLX", "Pull X", [form(&[0xFA], "implied", NONE, Cycles::fixed(4), "NZ")]),
    inst!("PLY", "Pull Y", [form(&[0x7A], "implied", NONE, Cycles::fixed(4), "NZ")]),
    inst!("INC", "Increment", [form(&[0x1A], "accumulator", NONE, Cycles::fixed(2), "NZ")]),
    inst!("DEC", "Decrement", [form(&[0x3A], "accumulator", NONE, Cycles::fixed(2), "NZ")]),
    inst!("BRA", "Branch always", [form(&[0x80], "relative", ONE_REL, Cycles::branch(3), "")]),
    inst!("STZ", "Store zero", [
        form(&[0x64], "zeropage", ONE_DP, Cycles::fixed(3), ""),
        form(&[0x74], "zeropage,x", ONE_DP, Cycles::fixed(4), ""),
        form(&[0x9C], "absolute", ONE_ABS, Cycles::fixed(4), ""),
        form(&[0x9E], "absolute,x", ONE_ABS, Cycles::fixed(5), ""),
    ]),
    inst!("TRB", "Test and reset bits", [
        form(&[0x14], "zeropage", ONE_DP, Cycles::fixed(5), "Z"),
        form(&[0x1C], "absolute", ONE_ABS, Cycles::fixed(6), "Z"),
    ]),
    inst!("TSB", "Test and set bits", [
        form(&[0x04], "zeropage", ONE_DP, Cycles::fixed(5), "Z"),
        form(&[0x0C], "absolute", ONE_ABS, Cycles::fixed(6), "Z"),
    ]),
    inst!("BIT", "Bit test", [
        form(&[0x89], "immediate", ONE_IMM8, Cycles::fixed(2), "Z"),
        form(&[0x34], "zeropage,x", ONE_DP, Cycles::fixed(4), "NZV"),
        form(&[0x3C], "absolute,x", ONE_ABS, Cycles::fixed(4), "NZV"),
    ]),
    inst!("JMP", "Jump", [form(&[0x7C], "(absolute,x)", ONE_ABS, Cycles::fixed(6), "")]),
    inst!("ORA", "OR accumulator", [form(&[0x12], "(indirect)", ONE_DP, Cycles::fixed(5), "NZ")]),
    inst!("AND", "AND accumulator", [form(&[0x32], "(indirect)", ONE_DP, Cycles::fixed(5), "NZ")]),
    inst!("EOR", "Exclusive-OR accumulator", [form(&[0x52], "(indirect)", ONE_DP, Cycles::fixed(5), "NZ")]),
    inst!("ADC", "Add with carry", [form(&[0x72], "(indirect)", ONE_DP, Cycles::fixed(5), "NZCV")]),
    inst!("STA", "Store accumulator", [form(&[0x92], "(indirect)", ONE_DP, Cycles::fixed(5), "")]),
    inst!("LDA", "Load accumulator", [form(&[0xB2], "(indirect)", ONE_DP, Cycles::fixed(5), "NZ")]),
    inst!("CMP", "Compare accumulator", [form(&[0xD2], "(indirect)", ONE_DP, Cycles::fixed(5), "NZC")]),
    inst!("SBC", "Subtract with carry", [form(&[0xF2], "(indirect)", ONE_DP, Cycles::fixed(5), "NZCV")]),
];

#[rustfmt::skip]
pub(crate) const ROCKWELL_ADDITIONS: [Instruction; 32] = [
    bit_zp!("RMB0", 0x07), bit_zp!("RMB1", 0x17), bit_zp!("RMB2", 0x27), bit_zp!("RMB3", 0x37),
    bit_zp!("RMB4", 0x47), bit_zp!("RMB5", 0x57), bit_zp!("RMB6", 0x67), bit_zp!("RMB7", 0x77),
    bit_zp!("SMB0", 0x87), bit_zp!("SMB1", 0x97), bit_zp!("SMB2", 0xA7), bit_zp!("SMB3", 0xB7),
    bit_zp!("SMB4", 0xC7), bit_zp!("SMB5", 0xD7), bit_zp!("SMB6", 0xE7), bit_zp!("SMB7", 0xF7),
    bit_branch!("BBR0", 0x0F), bit_branch!("BBR1", 0x1F), bit_branch!("BBR2", 0x2F), bit_branch!("BBR3", 0x3F),
    bit_branch!("BBR4", 0x4F), bit_branch!("BBR5", 0x5F), bit_branch!("BBR6", 0x6F), bit_branch!("BBR7", 0x7F),
    bit_branch!("BBS0", 0x8F), bit_branch!("BBS1", 0x9F), bit_branch!("BBS2", 0xAF), bit_branch!("BBS3", 0xBF),
    bit_branch!("BBS4", 0xCF), bit_branch!("BBS5", 0xDF), bit_branch!("BBS6", 0xEF), bit_branch!("BBS7", 0xFF),
];

const ROCKWELL_INSTRUCTIONS: [Instruction; 52] = join(CMOS_INSTRUCTIONS, ROCKWELL_ADDITIONS);

const WDC_ADDITIONS: [Instruction; 2] = [
    inst!(
        "WAI",
        "Wait for interrupt",
        [form(&[0xCB], "implied", NONE, Cycles::fixed(3), "")]
    ),
    inst!(
        "STP",
        "Stop the clock",
        [form(&[0xDB], "implied", NONE, Cycles::fixed(3), "")]
    ),
];

const WDC_INSTRUCTIONS: [Instruction; 54] = join(ROCKWELL_INSTRUCTIONS, WDC_ADDITIONS);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_exactly_the_plain_cmos_additions() {
        assert_eq!(
            SET.instructions
                .iter()
                .map(|insn| insn.forms.len())
                .sum::<usize>(),
            27
        );
        assert_eq!(
            SET.find_form("BRA", "relative").expect("BRA").opcode,
            &[0x80]
        );
        assert_eq!(
            SET.find_form("LDA", "(indirect)").expect("LDA (dp)").opcode,
            &[0xB2]
        );
        assert_eq!(
            SET.find_form("JMP", "(absolute,x)")
                .expect("JMP (abs,x)")
                .opcode,
            &[0x7C]
        );
        assert!(SET.find_form("BBR0", "zeropage,relative").is_none());
        assert!(SET.find_form("WAI", "implied").is_none());
    }

    #[test]
    fn descendant_profiles_are_cumulative() {
        assert_eq!(
            ROCKWELL_SET
                .instructions
                .iter()
                .map(|i| i.forms.len())
                .sum::<usize>(),
            59
        );
        assert_eq!(
            WDC_SET
                .instructions
                .iter()
                .map(|i| i.forms.len())
                .sum::<usize>(),
            61
        );
        assert_eq!(
            ROCKWELL_SET
                .find_form("BBR4", "zeropage,relative")
                .expect("BBR4")
                .opcode,
            &[0x4F]
        );
        assert!(ROCKWELL_SET.find_form("WAI", "implied").is_none());
        assert_eq!(
            WDC_SET.find_form("WAI", "implied").expect("WAI").opcode,
            &[0xCB]
        );
        assert_eq!(
            WDC_SET.find_form("STP", "implied").expect("STP").opcode,
            &[0xDB]
        );
    }
}
