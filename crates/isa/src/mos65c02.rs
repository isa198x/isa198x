//! CMOS 65C02 instruction-set additions over the documented NMOS 6502.
//!
//! This is an additive [`InstructionSet`]: consumers layer it on
//! [`crate::mos6502::SET`]. It contains the 27 newly documented opcode forms
//! common to the plain 65C02 profile. Rockwell bit operations and WDC's later
//! `WAI`/`STP` additions belong to their narrower descendant profiles.
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
    instructions: INSTRUCTIONS,
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

#[rustfmt::skip]
const INSTRUCTIONS: &[Instruction] = &[
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
}
