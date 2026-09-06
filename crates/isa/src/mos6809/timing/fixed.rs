//! Motorola MC6809–MC6809E Programming Manual (1981): Appendix F Table F-1
//! for cycles, Appendix A for ANDCC, ORCC, ABX, NOP, RTS, MUL, SEX, DAA,
//! and the accumulator forms of the read-modify-write/test operations.
//! [Timing](https://www.maddes.net/m6809pm/appendix_f.htm),
//! [operations and flags](https://www.maddes.net/m6809pm/appendix_a.htm).

use super::{ConditionCodeEffects, memory};
use crate::mos6809::{Insn, Kind};

/// Complete fixed nominal instruction timing and condition-code effects.
/// External stalls are excluded. This type cannot describe interrupt waits
/// or runtime-dependent return-from-interrupt timing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct FixedEffects {
    /// Complete nominal cycle count, not a lower bound or indexed base cost.
    pub cycles: u8,
    /// Flag input dependencies and output categories.
    pub condition_codes: ConditionCodeEffects,
}

impl Insn {
    /// Resolve ANDCC/ORCC for the encoded immediate mask, including all CC bits.
    /// Masked-off ANDCC bits are cleared; masked-on ORCC bits are set; all
    /// others are preserved. Preservation alone is not a `read` dependency.
    /// Other instructions, including CWAI, return `None`.
    ///
    /// ```
    /// use isa198x::mos6809::lookup;
    /// let e = lookup("andcc").expect("ANDCC")
    ///     .immediate_cc_effects(0xef).expect("immediate CC operation");
    /// assert_eq!(e.cycles, 3);
    /// assert_eq!(e.condition_codes.cleared, 0x10); // I
    /// assert_eq!(e.condition_codes.preserved(), 0xef);
    /// ```
    #[must_use]
    pub fn immediate_cc_effects(&self, mask: u8) -> Option<FixedEffects> {
        if self.undocumented {
            return None;
        }
        let Kind::Mem { imm, width: 1, .. } = self.kind else {
            return None;
        };
        let (cleared, set) = match imm {
            [0x1c] => (!mask, 0),
            [0x1a] => (0, mask),
            _ => return None,
        };
        Some(FixedEffects {
            cycles: 3,
            condition_codes: cc(0, 0, cleared, set, 0),
        })
    }

    /// Resolve the covered fixed-cost inherent instructions: accumulator
    /// read-modify-write/test forms, NOP, ABX, RTS, MUL, SEX and DAA.
    ///
    /// Returns `None` for other kinds and undocumented encodings. SYNC, RTI
    /// and SWI/SWI2/SWI3 are outside this API's current coverage; no wait or
    /// runtime-dependent cost is substituted with a fixed lower bound.
    #[must_use]
    pub fn fixed_inherent_effects(&self) -> Option<FixedEffects> {
        if self.undocumented {
            return None;
        }
        let Kind::Inherent(opcode) = self.kind else {
            return None;
        };
        let (cycles, condition_codes) = match opcode {
            [0x12] => (2, cc(0, 0, 0, 0, 0)),          // NOP
            [0x3a] => (3, cc(0, 0, 0, 0, 0)),          // ABX
            [0x39] => (5, cc(0, 0, 0, 0, 0)),          // RTS
            [0x3d] => (11, cc(0, 0x05, 0, 0, 0)),      // MUL: Z and C (B bit 7)
            [0x1d] => (2, cc(0, 0x0c, 0, 0, 0)),       // SEX: N/Z; V preserved
            [0x19] => (2, cc(0x21, 0x0d, 0, 0, 0x02)), // DAA: H/C input, V undefined
            [op @ 0x40..=0x5f] => {
                // Appendix A gives one flag definition for memory and both
                // accumulator forms. Reuse that fact, not the memory timing.
                let name = match op & 0x0f {
                    0x0 => "neg",
                    0x3 => "com",
                    0x4 => "lsr",
                    0x6 => "ror",
                    0x7 => "asr",
                    0x8 => "asl",
                    0x9 => "rol",
                    0xa => "dec",
                    0xc => "inc",
                    0xd => "tst",
                    0xf => "clr",
                    _ => return None,
                };
                (2, memory::facts(name)?.1)
            }
            _ => return None,
        };
        Some(FixedEffects {
            cycles,
            condition_codes,
        })
    }
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
