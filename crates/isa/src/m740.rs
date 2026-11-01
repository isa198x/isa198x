//! Mitsubishi/Renesas 740 family (M740) instruction-set extension over the NMOS
//! 6502.
//!
//! The 740 family's core runs the 6502's instructions and adds bit operations
//! (`SEB`/`CLB` and `BBS`/`BBC` on a bit of A or of a zero-page byte),
//! `LDM`, `COM`, `TST`, `RRF`, the T flag (`CLT`/`SET`), `BRA`, `INC A`/`DEC A`,
//! zero-page indirect `JSR`/`JMP`, special-page `JSR`, and the product-dependent
//! `STP`, `WIT`, `MUL` and `DIV`. This extension carries only what
//! [`crate::mos6502::SET`] lacks.
//!
//! Each bit operation names its bit in the mnemonic, `SEB3` or `BBS0`, as the
//! HuC6280's `RMB`/`BBR` do, so every form is a fixed-slot [`Form`]. Three
//! encodings differ from what a 6502 or 65C02 reader would expect, and are
//! taken from the manual:
//!
//! - `DEC A` is `$1A` and `INC A` is `$3A`, the reverse of the 65C02;
//! - `LDM #nn,zz` lays the value down before the address (`immediate,zeropage`);
//! - an accumulator bit branch is two bytes, and its displacement counts from
//!   the byte after it, as any branch's does.
//!
//! Provenance: the Renesas *740 Family Software Manual* (Rev.2.00, 2006) and
//! Mitsubishi's first edition (1997), whose opcode map and instruction pages
//! are held in `reference/by-topic/cpu-m740/`. ca65 V2.19 disagrees with the
//! manual on the three encodings above; MAME's M740 core agrees with it.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const IMM8: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 1,
};
const DP: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 1,
};
const REL: Operand = Operand {
    kind: OperandKind::RelativePc,
    bytes: 1,
};
const NONE: &[Operand] = &[];
const ONE_DP: &[Operand] = &[DP];
const ONE_REL: &[Operand] = &[REL];
/// A zero-page bit branch: the byte to test, then the displacement.
const DP_REL: &[Operand] = &[DP, REL];
/// `LDM #nn,zz`: the value, then the zero-page address.
const IMM_DP: &[Operand] = &[IMM8, DP];

pub const SET: InstructionSet = InstructionSet {
    cpu: "Mitsubishi 740 family (extension)",
    endianness: Endianness::Little,
    instructions: INSTRUCTIONS,
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
const INSTRUCTIONS: &[Instruction] = &[
    inst!("JSR", "Jump to subroutine", [form(&[0x02], "(indirect)", ONE_DP), form(&[0x22], "special-page", ONE_DP)]),
    inst!("BBS0", "Branch on bit set", [form(&[0x03], "accumulator,relative", ONE_REL), form(&[0x07], "zeropage,relative", DP_REL)]),
    inst!("SEB0", "Set bit", [form(&[0x0B], "accumulator", NONE), form(&[0x0F], "zeropage", ONE_DP)]),
    inst!("CLT", "Clear T flag", [form(&[0x12], "implied", NONE)]),
    inst!("BBC0", "Branch on bit clear", [form(&[0x13], "accumulator,relative", ONE_REL), form(&[0x17], "zeropage,relative", DP_REL)]),
    inst!("DEC", "Decrement", [form(&[0x1A], "accumulator", NONE)]),
    inst!("CLB0", "Clear bit", [form(&[0x1B], "accumulator", NONE), form(&[0x1F], "zeropage", ONE_DP)]),
    inst!("BBS1", "Branch on bit set", [form(&[0x23], "accumulator,relative", ONE_REL), form(&[0x27], "zeropage,relative", DP_REL)]),
    inst!("SEB1", "Set bit", [form(&[0x2B], "accumulator", NONE), form(&[0x2F], "zeropage", ONE_DP)]),
    inst!("SET", "Set T flag", [form(&[0x32], "implied", NONE)]),
    inst!("BBC1", "Branch on bit clear", [form(&[0x33], "accumulator,relative", ONE_REL), form(&[0x37], "zeropage,relative", DP_REL)]),
    inst!("INC", "Increment", [form(&[0x3A], "accumulator", NONE)]),
    inst!("CLB1", "Clear bit", [form(&[0x3B], "accumulator", NONE), form(&[0x3F], "zeropage", ONE_DP)]),
    inst!("LDM", "Load immediate into memory", [form(&[0x3C], "immediate,zeropage", IMM_DP)]),
    inst!("STP", "Stop the clock", [form(&[0x42], "implied", NONE)]),
    inst!("BBS2", "Branch on bit set", [form(&[0x43], "accumulator,relative", ONE_REL), form(&[0x47], "zeropage,relative", DP_REL)]),
    inst!("COM", "Complement memory", [form(&[0x44], "zeropage", ONE_DP)]),
    inst!("SEB2", "Set bit", [form(&[0x4B], "accumulator", NONE), form(&[0x4F], "zeropage", ONE_DP)]),
    inst!("BBC2", "Branch on bit clear", [form(&[0x53], "accumulator,relative", ONE_REL), form(&[0x57], "zeropage,relative", DP_REL)]),
    inst!("CLB2", "Clear bit", [form(&[0x5B], "accumulator", NONE), form(&[0x5F], "zeropage", ONE_DP)]),
    inst!("MUL", "Multiply", [form(&[0x62], "zeropage,x", ONE_DP)]),
    inst!("BBS3", "Branch on bit set", [form(&[0x63], "accumulator,relative", ONE_REL), form(&[0x67], "zeropage,relative", DP_REL)]),
    inst!("TST", "Test memory for zero or negative", [form(&[0x64], "zeropage", ONE_DP)]),
    inst!("SEB3", "Set bit", [form(&[0x6B], "accumulator", NONE), form(&[0x6F], "zeropage", ONE_DP)]),
    inst!("BBC3", "Branch on bit clear", [form(&[0x73], "accumulator,relative", ONE_REL), form(&[0x77], "zeropage,relative", DP_REL)]),
    inst!("CLB3", "Clear bit", [form(&[0x7B], "accumulator", NONE), form(&[0x7F], "zeropage", ONE_DP)]),
    inst!("BRA", "Branch always", [form(&[0x80], "relative", ONE_REL)]),
    inst!("RRF", "Rotate right by four bits", [form(&[0x82], "zeropage", ONE_DP)]),
    inst!("BBS4", "Branch on bit set", [form(&[0x83], "accumulator,relative", ONE_REL), form(&[0x87], "zeropage,relative", DP_REL)]),
    inst!("SEB4", "Set bit", [form(&[0x8B], "accumulator", NONE), form(&[0x8F], "zeropage", ONE_DP)]),
    inst!("BBC4", "Branch on bit clear", [form(&[0x93], "accumulator,relative", ONE_REL), form(&[0x97], "zeropage,relative", DP_REL)]),
    inst!("CLB4", "Clear bit", [form(&[0x9B], "accumulator", NONE), form(&[0x9F], "zeropage", ONE_DP)]),
    inst!("BBS5", "Branch on bit set", [form(&[0xA3], "accumulator,relative", ONE_REL), form(&[0xA7], "zeropage,relative", DP_REL)]),
    inst!("SEB5", "Set bit", [form(&[0xAB], "accumulator", NONE), form(&[0xAF], "zeropage", ONE_DP)]),
    inst!("JMP", "Jump", [form(&[0xB2], "(indirect)", ONE_DP)]),
    inst!("BBC5", "Branch on bit clear", [form(&[0xB3], "accumulator,relative", ONE_REL), form(&[0xB7], "zeropage,relative", DP_REL)]),
    inst!("CLB5", "Clear bit", [form(&[0xBB], "accumulator", NONE), form(&[0xBF], "zeropage", ONE_DP)]),
    inst!("WIT", "Wait for an interrupt", [form(&[0xC2], "implied", NONE)]),
    inst!("BBS6", "Branch on bit set", [form(&[0xC3], "accumulator,relative", ONE_REL), form(&[0xC7], "zeropage,relative", DP_REL)]),
    inst!("SEB6", "Set bit", [form(&[0xCB], "accumulator", NONE), form(&[0xCF], "zeropage", ONE_DP)]),
    inst!("BBC6", "Branch on bit clear", [form(&[0xD3], "accumulator,relative", ONE_REL), form(&[0xD7], "zeropage,relative", DP_REL)]),
    inst!("CLB6", "Clear bit", [form(&[0xDB], "accumulator", NONE), form(&[0xDF], "zeropage", ONE_DP)]),
    inst!("DIV", "Divide", [form(&[0xE2], "zeropage,x", ONE_DP)]),
    inst!("BBS7", "Branch on bit set", [form(&[0xE3], "accumulator,relative", ONE_REL), form(&[0xE7], "zeropage,relative", DP_REL)]),
    inst!("SEB7", "Set bit", [form(&[0xEB], "accumulator", NONE), form(&[0xEF], "zeropage", ONE_DP)]),
    inst!("BBC7", "Branch on bit clear", [form(&[0xF3], "accumulator,relative", ONE_REL), form(&[0xF7], "zeropage,relative", DP_REL)]),
    inst!("CLB7", "Clear bit", [form(&[0xFB], "accumulator", NONE), form(&[0xFF], "zeropage", ONE_DP)]),
];

#[cfg(test)]
mod tests {
    use super::*;

    /// Every code the manual's map assigns, between this extension and the
    /// 6502's set, and no code twice.
    #[test]
    fn the_map_is_covered_once() {
        let mut seen = std::collections::BTreeSet::new();
        for set in [&crate::mos6502::SET, &SET] {
            for instruction in set.instructions {
                for form in instruction.forms {
                    assert!(seen.insert(form.opcode[0]), "{:02X} twice", form.opcode[0]);
                }
            }
        }
        assert_eq!(seen.len(), 231);
    }

    #[test]
    fn the_manual_decides_where_a_65c02_reader_would_not() {
        assert_eq!(
            SET.find_form("DEC", "accumulator").expect("DEC A").opcode,
            &[0x1A]
        );
        assert_eq!(
            SET.find_form("INC", "accumulator").expect("INC A").opcode,
            &[0x3A]
        );
        let ldm = SET.find_form("LDM", "immediate,zeropage").expect("LDM");
        assert_eq!(ldm.opcode, &[0x3C]);
        assert_eq!(ldm.operands, IMM_DP);
        assert_eq!(
            SET.find_form("BBS0", "accumulator,relative")
                .expect("BBS0 A")
                .opcode,
            &[0x03]
        );
        assert_eq!(
            SET.find_form("CLB7", "zeropage").expect("CLB7 zz").opcode,
            &[0xFF]
        );
        assert_eq!(
            SET.find_form("JSR", "special-page")
                .expect("JSR \\$FFnn")
                .opcode,
            &[0x22]
        );
    }
}
