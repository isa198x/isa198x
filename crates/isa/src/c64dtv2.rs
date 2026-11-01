//! C64DTV revision 2 CPU extension over the documented NMOS 6502.
//!
//! ACME's profile consists of the NMOS undocumented forms also implemented by
//! DTV2 (all of its `6510` mnemonic families except `ANC`) plus the DTV-specific
//! `BRA`, `SAC`, and `SIR`. It is cumulative so consumers need only this one
//! extension beside [`crate::mos6502::SET`].
//!
//! **Provenance.** The DTV instruction meanings and encodings come from the
//! *C64DTV Programming Guide* in `reference/by-topic/cpu-c64dtv/`; the exposed
//! mnemonic boundary is checked against ACME and cc65's independently authored
//! `6502dtv-opcodes.s` opcode-space test.
//!
//! **`ANC` is disputed, so there are two sets.** [`SET`] is ACME's profile,
//! which dropped `ANC` in ACME 0.95.6 on a reported test that `$0B` and `$2B`
//! do not work on a DTV2. [`CA65_SET`] adds `ANC #imm` at `$0B`, as cc65 does:
//! ca65 V2.19's manual lists `ANC` among the DTV's supported undocumented
//! instructions, its `6502dtv-opcodes.s` assembles `anc` as `$0B`, and VICE's
//! DTV core (`6510dtvcore.c`) executes `$0B` and `$2B` as `ANC`. The
//! programming guide does not cover the undocumented instructions, so no
//! primary source settles it. Each assembler profile takes the set its
//! reference tool assembles.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const IMM8: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 1,
};
const REL: Operand = Operand {
    kind: OperandKind::RelativePc,
    bytes: 1,
};
const ONE_IMM: &[Operand] = &[IMM8];
const ONE_REL: &[Operand] = &[REL];

pub const SET: InstructionSet = InstructionSet {
    cpu: "C64DTV revision 2 (extension)",
    endianness: Endianness::Little,
    instructions: &INSTRUCTIONS,
};

/// [`SET`] with `ANC #imm` at `$0B`, as cc65's ca65 assembles the DTV. See
/// the module notes for why the two differ.
pub const CA65_SET: InstructionSet = InstructionSet {
    cpu: "C64DTV revision 2, as ca65 reads it (extension)",
    endianness: Endianness::Little,
    instructions: &CA65_INSTRUCTIONS,
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

const ADDITIONS: [Instruction; 3] = [
    Instruction {
        mnemonic: "BRA",
        summary: "Branch always",
        forms: &[form(&[0x12], "relative", ONE_REL)],
    },
    Instruction {
        mnemonic: "SAC",
        summary: "Set accumulator mapping",
        forms: &[form(&[0x32], "immediate", ONE_IMM)],
    },
    Instruction {
        mnemonic: "SIR",
        summary: "Set index-register mapping",
        forms: &[form(&[0x42], "immediate", ONE_IMM)],
    },
];

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

const INSTRUCTIONS: [Instruction; 25] = join(
    crate::nmos6502_undocumented::DTV_SHARED_INSTRUCTIONS,
    ADDITIONS,
);

const CA65_INSTRUCTIONS: [Instruction; 26] =
    join(INSTRUCTIONS, crate::nmos6502_undocumented::ANC_ADDITION);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_the_acme_profile_boundary() {
        assert_eq!(SET.rows().count(), 80);
        assert_eq!(
            SET.find_form("BRA", "relative").expect("BRA").opcode,
            &[0x12]
        );
        assert_eq!(
            SET.find_form("SAC", "immediate").expect("SAC").opcode,
            &[0x32]
        );
        assert_eq!(
            SET.find_form("SIR", "immediate").expect("SIR").opcode,
            &[0x42]
        );
        assert!(SET.find_form("SLO", "zeropage").is_some());
        assert!(SET.find_form("ANC", "immediate").is_none());
    }

    #[test]
    fn the_ca65_reading_adds_only_anc() {
        assert_eq!(CA65_SET.rows().count(), SET.rows().count() + 1);
        assert_eq!(
            CA65_SET.find_form("ANC", "immediate").expect("ANC").opcode,
            &[0x0B]
        );
        for row in SET.rows() {
            assert!(CA65_SET.rows().any(|other| other == row), "{row:?}");
        }
    }
}
