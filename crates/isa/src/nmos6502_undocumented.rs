//! Undocumented NMOS 6502/6510 instruction-set extension.
//!
//! This is a separately selectable extension: documented 6502 source must not
//! acquire these mnemonics merely because the silicon decodes their bytes.
//! Encodings and timings come from the umbrella reference's complete opcode
//! table and 6510 synthesis:
//! `reference/by-topic/cpu-6502/cpu-6502-opcode-table.md` and
//! `reference/by-topic/cpu-6510/cpu-6510-reference.md`.

use crate::{Cycles, Endianness, Form, Instruction, InstructionSet, Operand, OperandKind};

const IMM8: Operand = Operand {
    kind: OperandKind::Immediate,
    bytes: 1,
};
const ADDR8: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 1,
};
const ADDR16: Operand = Operand {
    kind: OperandKind::Address,
    bytes: 2,
};
const NONE: &[Operand] = &[];
const ONE_IMM: &[Operand] = &[IMM8];
const ONE_ZP: &[Operand] = &[ADDR8];
const ONE_ABS: &[Operand] = &[ADDR16];

/// The NMOS-only forms ACME exposes under its `6510`/`nmos6502` target.
pub const SET: InstructionSet = InstructionSet {
    cpu: "NMOS 6502 undocumented extension",
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
        undocumented: true,
    }
}

const fn documented_form(opcode: &'static [u8], mode: &'static str) -> Form {
    Form {
        opcode,
        mode,
        operands: NONE,
        suffix: &[],
        cycles: Cycles::fixed(2),
        flags: "",
        undocumented: false,
    }
}

macro_rules! inst {
    ($mnemonic:literal, $summary:literal, [ $($form:expr),* $(,)? ]) => {
        Instruction { mnemonic: $mnemonic, summary: $summary, forms: &[ $($form),* ] }
    };
}

macro_rules! rmw {
    ($mnemonic:literal, $summary:literal, $base:literal, $flags:literal) => {
        inst!(
            $mnemonic,
            $summary,
            [
                form(
                    &[$base + 0x04],
                    "zeropage",
                    ONE_ZP,
                    Cycles::fixed(5),
                    $flags
                ),
                form(
                    &[$base + 0x14],
                    "zeropage,x",
                    ONE_ZP,
                    Cycles::fixed(6),
                    $flags
                ),
                form(&[$base], "(indirect,x)", ONE_ZP, Cycles::fixed(8), $flags),
                form(
                    &[$base + 0x10],
                    "(indirect),y",
                    ONE_ZP,
                    Cycles::fixed(8),
                    $flags
                ),
                form(
                    &[$base + 0x0C],
                    "absolute",
                    ONE_ABS,
                    Cycles::fixed(6),
                    $flags
                ),
                form(
                    &[$base + 0x1C],
                    "absolute,x",
                    ONE_ABS,
                    Cycles::fixed(7),
                    $flags
                ),
                form(
                    &[$base + 0x18],
                    "absolute,y",
                    ONE_ABS,
                    Cycles::fixed(7),
                    $flags
                ),
            ]
        )
    };
}

#[rustfmt::skip]
const INSTRUCTIONS: &[Instruction] = &[
    rmw!("SLO", "Shift left then OR accumulator",       0x03, "NZC"),
    rmw!("RLA", "Rotate left then AND accumulator",     0x23, "NZC"),
    rmw!("SRE", "Shift right then exclusive OR",        0x43, "NZC"),
    rmw!("RRA", "Rotate right then add with carry",      0x63, "NZCV"),
    rmw!("DCP", "Decrement then compare",                0xC3, "NZC"),
    rmw!("ISC", "Increment then subtract with carry",    0xE3, "NZCV"),

    inst!("SAX", "Store accumulator AND X", [
        form(&[0x87], "zeropage",     ONE_ZP,  Cycles::fixed(3), ""),
        form(&[0x97], "zeropage,y",   ONE_ZP,  Cycles::fixed(4), ""),
        form(&[0x83], "(indirect,x)", ONE_ZP,  Cycles::fixed(6), ""),
        form(&[0x8F], "absolute",     ONE_ABS, Cycles::fixed(4), ""),
    ]),
    inst!("LAX", "Load accumulator and X", [
        form(&[0xA7], "zeropage",     ONE_ZP,  Cycles::fixed(3),         "NZ"),
        form(&[0xB7], "zeropage,y",   ONE_ZP,  Cycles::fixed(4),         "NZ"),
        form(&[0xA3], "(indirect,x)", ONE_ZP,  Cycles::fixed(6),         "NZ"),
        form(&[0xB3], "(indirect),y", ONE_ZP,  Cycles::page_crossing(5), "NZ"),
        form(&[0xAF], "absolute",     ONE_ABS, Cycles::fixed(4),         "NZ"),
        form(&[0xBF], "absolute,y",   ONE_ABS, Cycles::page_crossing(4), "NZ"),
    ]),
    inst!("LAS", "Load accumulator, X and stack pointer", [
        form(&[0xBB], "absolute,y", ONE_ABS, Cycles::page_crossing(4), "NZ"),
    ]),
    inst!("TAS", "Transfer accumulator AND X to stack and memory", [
        form(&[0x9B], "absolute,y", ONE_ABS, Cycles::fixed(5), ""),
    ]),
    inst!("SHA", "Store accumulator AND X AND high byte", [
        form(&[0x9F], "absolute,y",   ONE_ABS, Cycles::fixed(5), ""),
        form(&[0x93], "(indirect),y", ONE_ZP,  Cycles::fixed(6), ""),
    ]),
    inst!("SHX", "Store X AND high byte", [
        form(&[0x9E], "absolute,y", ONE_ABS, Cycles::fixed(5), ""),
    ]),
    inst!("SHY", "Store Y AND high byte", [
        form(&[0x9C], "absolute,x", ONE_ABS, Cycles::fixed(5), ""),
    ]),
    inst!("ANC", "AND immediate and copy negative to carry", [
        form(&[0x0B], "immediate", ONE_IMM, Cycles::fixed(2), "NZC"),
    ]),
    inst!("ASR", "AND immediate then shift right", [
        form(&[0x4B], "immediate", ONE_IMM, Cycles::fixed(2), "NZC"),
    ]),
    inst!("ARR", "AND immediate then rotate right", [
        form(&[0x6B], "immediate", ONE_IMM, Cycles::fixed(2), "NZCV"),
    ]),
    inst!("SBX", "Subtract immediate from accumulator AND X", [
        form(&[0xCB], "immediate", ONE_IMM, Cycles::fixed(2), "NZC"),
    ]),
    inst!("ANE", "Unstable accumulator AND X immediate", [
        form(&[0x8B], "immediate", ONE_IMM, Cycles::fixed(2), "NZ"),
    ]),
    inst!("LXA", "Unstable load accumulator and X immediate", [
        form(&[0xAB], "immediate", ONE_IMM, Cycles::fixed(2), "NZ"),
    ]),
    inst!("DOP", "Two-byte no operation", [
        form(&[0x80], "implied",   NONE,    Cycles::fixed(2), ""),
        form(&[0x80], "immediate", ONE_IMM, Cycles::fixed(2), ""),
        form(&[0x04], "zeropage",  ONE_ZP,  Cycles::fixed(3), ""),
        form(&[0x14], "zeropage,x",ONE_ZP,  Cycles::fixed(4), ""),
    ]),
    inst!("TOP", "Three-byte no operation", [
        form(&[0x0C], "implied",    NONE,    Cycles::fixed(4),         ""),
        form(&[0x0C], "absolute",  ONE_ABS, Cycles::fixed(4),         ""),
        form(&[0x1C], "absolute,x",ONE_ABS, Cycles::page_crossing(4), ""),
    ]),
    // ACME permits the DOP/TOP forms under the ordinary NOP spelling. The
    // documented implied NOP is repeated so selecting this extension does not
    // hide it during mnemonic-level mode resolution.
    inst!("NOP", "No operation", [
        documented_form(&[0xEA], "implied"),
        form(&[0x80], "immediate", ONE_IMM, Cycles::fixed(2), ""),
        form(&[0x04], "zeropage", ONE_ZP, Cycles::fixed(3), ""),
        form(&[0x14], "zeropage,x", ONE_ZP, Cycles::fixed(4), ""),
        form(&[0x0C], "absolute", ONE_ABS, Cycles::fixed(4), ""),
        form(&[0x1C], "absolute,x", ONE_ABS, Cycles::page_crossing(4), ""),
    ]),
    inst!("JAM", "Halt until reset", [
        form(&[0x02], "implied", NONE, Cycles::fixed(2), ""),
    ]),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_form_is_marked_undocumented() {
        assert_eq!(SET.rows().count(), 78);
        assert!(
            SET.rows()
                .filter(|row| !(row.mnemonic == "NOP" && row.mode == "implied"))
                .all(|row| row.undocumented)
        );
    }

    #[test]
    fn mnemonic_modes_are_unique() {
        let mut seen = std::collections::BTreeSet::new();
        for row in SET.rows() {
            assert!(
                seen.insert((row.mnemonic, row.mode)),
                "duplicate mnemonic and mode"
            );
        }
    }

    #[test]
    fn acme_probe_encodings_are_present() {
        assert_eq!(
            SET.find_form("LAX", "zeropage").expect("lax zp").opcode,
            &[0xA7]
        );
        assert_eq!(
            SET.find_form("SRE", "zeropage").expect("sre zp").opcode,
            &[0x47]
        );
    }

    #[test]
    fn extension_only_repeats_nop_for_its_extra_modes() {
        assert!(!SET.has_mnemonic("LDA"));
        assert_eq!(
            SET.find_form("NOP", "implied")
                .expect("documented nop")
                .opcode,
            &[0xEA]
        );
    }
}
