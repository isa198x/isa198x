//! Motorola MC6809–MC6809E Programming Manual (1981), Appendix F Table F-1
//! (base timings), Appendix A (each instruction's operation and CC entry).
//! Indexed surcharges come from the parent module's manufacturer Table 2.
//! [Timing](https://www.maddes.net/m6809pm/appendix_f.htm),
//! [flag effects](https://www.maddes.net/m6809pm/appendix_a.htm).

use super::{ConditionCodeEffects, indexed_cost};
use crate::mos6809::{Insn, Kind, lookup};

/// Complete operand-resolved nominal timing and flags of a memory form.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct MemoryEffects {
    /// Full instruction cycles, including the selected indexed surcharge.
    /// Excludes external stalls and, for JSR, execution of the called routine.
    pub cycles: u8,
    /// CC inputs and outcomes, not the values of the resulting flags.
    pub condition_codes: ConditionCodeEffects,
}

impl Insn {
    /// Resolve an ordinary memory form's timing and condition-code effects.
    ///
    /// Covers loads/stores, arithmetic/logic, read-modify-write/test, JMP/JSR,
    /// and LEA. Modes are the names used by [`crate::mos6809::rows`]. Pass
    /// `Some(postbyte)` for indexed mode, and `None` for every other mode.
    /// Returns `None` for missing/extra postbytes, undocumented postbytes,
    /// unsupported modes/opcodes, and ANDCC/ORCC/CWAI (not covered here).
    /// The displacement or effective address value does not affect timing.
    ///
    /// ```
    /// use isa198x::mos6809::lookup;
    /// let ldy = lookup("ldy").expect("LDY");
    /// let e = ldy.memory_effects("indexed", Some(0x99)).expect("LDY [n,X]");
    /// assert_eq!(e.cycles, 13); // six base + seven for 16-bit indirect offset
    /// assert_eq!(e.condition_codes.calculated, 0x0c); // N, Z
    /// assert_eq!(e.condition_codes.cleared, 0x02); // V
    /// assert!(ldy.memory_effects("indexed", None).is_none());
    /// ```
    #[must_use]
    pub fn memory_effects(&self, mode: &str, postbyte: Option<u8>) -> Option<MemoryEffects> {
        if self.undocumented {
            return None;
        }
        let (slot, opcode) = memory_opcode(&self.kind, mode)?;
        // Insn/Kind are public. A known mnemonic must not lend its timing to
        // an unrelated encoding supplied by a caller.
        let canonical = lookup(self.mnemonic)?;
        if opcode != memory_opcode(&canonical.kind, mode)?.1 {
            return None;
        }
        let extra = match (mode, postbyte) {
            ("indexed", Some(byte)) => indexed_cost(byte)?.cycles,
            ("indexed", None) | (_, Some(_)) => return None,
            (_, None) => 0,
        };
        let (base, condition_codes) = facts(self.mnemonic)?;
        let base = base[slot];
        if base == 0 {
            return None; // An absent table entry is never a complete timing.
        }
        Some(MemoryEffects {
            cycles: base + extra,
            condition_codes,
        })
    }
}

fn memory_opcode(kind: &Kind, mode: &str) -> Option<(usize, &'static [u8])> {
    let Kind::Mem {
        imm,
        direct,
        indexed,
        extended,
        ..
    } = kind
    else {
        return None;
    };
    let (slot, opcode) = match mode {
        "immediate" => (0, *imm),
        "direct" => (1, *direct),
        "indexed" => (2, *indexed),
        "extended" => (3, *extended),
        _ => return None,
    };
    (!opcode.is_empty()).then_some((slot, opcode))
}

// Arrays are immediate/direct/indexed/extended base cycles. A zero marks an
// absent mode, which memory_opcode rejects before this table is consulted.
pub(super) fn facts(name: &str) -> Option<([u8; 4], ConditionCodeEffects)> {
    let nz_clear_v = cc(0, 0x0c, 0x02, 0, 0);
    let arithmetic16 = cc(0, 0x0f, 0, 0, 0);
    let none = cc(0, 0, 0, 0, 0);
    Some(match name {
        "lda" | "ldb" | "anda" | "andb" | "bita" | "bitb" | "ora" | "orb" | "eora" | "eorb" => {
            ([2, 4, 4, 5], nz_clear_v)
        }
        "sta" | "stb" => ([0, 4, 4, 5], nz_clear_v),
        "ldd" | "ldx" | "ldu" => ([3, 5, 5, 6], nz_clear_v),
        "std" | "stx" | "stu" => ([0, 5, 5, 6], nz_clear_v),
        "ldy" | "lds" => ([4, 6, 6, 7], nz_clear_v),
        "sty" | "sts" => ([0, 6, 6, 7], nz_clear_v),
        "adda" | "addb" => ([2, 4, 4, 5], cc(0, 0x2f, 0, 0, 0)),
        "adca" | "adcb" => ([2, 4, 4, 5], cc(0x01, 0x2f, 0, 0, 0)),
        "suba" | "subb" | "cmpa" | "cmpb" => ([2, 4, 4, 5], cc(0, 0x0f, 0, 0, 0x20)),
        "sbca" | "sbcb" => ([2, 4, 4, 5], cc(0x01, 0x0f, 0, 0, 0x20)),
        "addd" | "subd" | "cmpx" => ([4, 6, 6, 7], arithmetic16),
        "cmpd" | "cmpy" | "cmpu" | "cmps" => ([5, 7, 7, 8], arithmetic16),
        "clr" => ([0, 6, 6, 7], cc(0, 0, 0x0b, 0x04, 0)),
        "inc" | "dec" => ([0, 6, 6, 7], cc(0, 0x0e, 0, 0, 0)),
        "tst" => ([0, 6, 6, 7], nz_clear_v),
        "com" => ([0, 6, 6, 7], cc(0, 0x0c, 0x02, 0x01, 0)),
        "neg" | "asl" | "lsl" => ([0, 6, 6, 7], cc(0, 0x0f, 0, 0, 0x20)),
        "lsr" => ([0, 6, 6, 7], cc(0, 0x05, 0x08, 0, 0)),
        "ror" => ([0, 6, 6, 7], cc(0x01, 0x0d, 0, 0, 0)),
        "asr" => ([0, 6, 6, 7], cc(0, 0x0d, 0, 0, 0x20)),
        "rol" => ([0, 6, 6, 7], cc(0x01, 0x0f, 0, 0, 0)),
        "jmp" => ([0, 3, 3, 4], none),
        "jsr" => ([0, 7, 7, 8], none),
        "leax" | "leay" => ([0, 0, 4, 0], cc(0, 0x04, 0, 0, 0)),
        "leas" | "leau" => ([0, 0, 4, 0], none),
        _ => return None,
    })
}

const fn cc(read: u8, calculated: u8, cleared: u8, set: u8, undefined: u8) -> ConditionCodeEffects {
    ConditionCodeEffects {
        read,
        calculated,
        cleared,
        set,
        undefined,
    }
}
