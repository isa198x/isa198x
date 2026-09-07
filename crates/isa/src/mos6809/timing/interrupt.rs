//! Motorola MC6809–MC6809E Programming Manual (1981), Appendix A operations
//! for SWI/SWI2/SWI3, RTI, SYNC and CWAI; Appendix D Table D-1 for timing.
//! D-1 explicitly gives SYNC >=4 and CWAI >=20, unlike the abbreviated
//! opcode maps. Appendix F's SYNC=2 conflicts with D-1 and C-1 (4); the
//! detailed programming aid's lower bound is used here, not a fixed cost.
//! [Operations](https://www.maddes.net/m6809pm/appendix_a.htm),
//! [timing](https://www.maddes.net/m6809pm/appendix_d.htm).

use super::ConditionCodeEffects;
use crate::mos6809::{Insn, Kind};

/// Nominal timing, excluding external stalls and execution of handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InterruptTiming {
    /// Complete fixed software-interrupt entry cost.
    Fixed(u8),
    /// RTI costs six cycles for a partial saved state, fifteen for an entire
    /// state. The selector is E in the restored CC byte, not live CC on entry.
    ReturnFromInterrupt,
    /// An interrupt wait has a documented minimum but no finite maximum.
    Wait {
        /// Manufacturer's lower bound, never a cycle-budget ceiling.
        minimum: u8,
    },
}

impl InterruptTiming {
    /// Nominal lower and upper bounds. `None` is an unbounded wait, not a
    /// missing table entry or permission to substitute the minimum.
    #[must_use]
    pub const fn bounds(self) -> (u8, Option<u8>) {
        match self {
            Self::Fixed(cycles) => (cycles, Some(cycles)),
            Self::ReturnFromInterrupt => (6, Some(15)),
            Self::Wait { minimum } => (minimum, None),
        }
    }

    /// Resolve RTI using the CC byte read from the saved stack frame.
    /// Returns `None` for other timing kinds; cannot resolve an interrupt wait.
    #[must_use]
    pub const fn rti_cycles(self, restored_cc: u8) -> Option<u8> {
        match self {
            Self::ReturnFromInterrupt => Some(if restored_cc & 0x80 == 0 { 6 } else { 15 }),
            _ => None,
        }
    }
}

/// The phase at which an interrupt instruction's CC effects are known.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InterruptConditionCodes {
    /// SWI-family effects at handler entry, before executing handler code.
    AtHandlerEntry(ConditionCodeEffects),
    /// SYNC/CWAI effects before waiting. Changes made by a subsequent hardware
    /// interrupt or its handler are separate and are not claimed here.
    BeforeWait(ConditionCodeEffects),
    /// RTI loads the complete saved CC byte; no bit is an arithmetic result.
    Restored,
}

/// Timing and phase-specific CC effects of the six interrupt instructions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct InterruptEffects {
    /// Fixed, restored-state-dependent, or unbounded-wait timing.
    pub timing: InterruptTiming,
    /// CC changes at the stated phase, relative to the instruction's input CC.
    pub condition_codes: InterruptConditionCodes,
    /// Transformation of input CC into the saved byte, if this instruction
    /// pushes one. SWI sets E before saving CC, then sets live I/F afterward.
    /// `None` means no CC push by this instruction (RTI and SYNC); it says
    /// nothing about a later hardware interrupt awakened by SYNC.
    pub stacked_condition_codes: Option<ConditionCodeEffects>,
}

impl Insn {
    /// Resolve SWI/SWI2/SWI3, RTI, SYNC or CWAI. Pass the immediate mask only
    /// for CWAI; missing or extra masks return `None`, as do other opcodes.
    ///
    /// ```
    /// use isa198x::mos6809::lookup;
    /// let wait = lookup("cwai").expect("CWAI").interrupt_effects(Some(0xef))
    ///     .expect("CWAI immediate");
    /// assert_eq!(wait.timing.bounds(), (20, None)); // no finite upper bound
    /// let rti = lookup("rti").expect("RTI").interrupt_effects(None).expect("RTI");
    /// assert_eq!(rti.timing.bounds(), (6, Some(15)));
    /// assert_eq!(rti.timing.rti_cycles(0x80), Some(15)); // saved E, not live E
    /// ```
    #[must_use]
    pub fn interrupt_effects(&self, mask: Option<u8>) -> Option<InterruptEffects> {
        if self.undocumented {
            return None;
        }
        use InterruptConditionCodes::{AtHandlerEntry, BeforeWait, Restored};
        use InterruptTiming::{Fixed, ReturnFromInterrupt, Wait};
        let (timing, condition_codes, stacked_condition_codes) = match (&self.kind, mask) {
            (Kind::Inherent([0x3f]), None) => {
                (Fixed(19), AtHandlerEntry(cc(0, 0xd0)), Some(cc(0, 0x80)))
            }
            (Kind::Inherent([0x10, 0x3f] | [0x11, 0x3f]), None) => {
                (Fixed(20), AtHandlerEntry(cc(0, 0x80)), Some(cc(0, 0x80)))
            }
            (Kind::Inherent([0x3b]), None) => (ReturnFromInterrupt, Restored, None),
            (Kind::Inherent([0x13]), None) => (Wait { minimum: 4 }, BeforeWait(cc(0, 0)), None),
            (
                Kind::Mem {
                    imm: [0x3c],
                    width: 1,
                    ..
                },
                Some(mask),
            ) => {
                // AND immediate, then set E regardless of the mask's E bit.
                let saved = cc(!mask & 0x7f, 0x80);
                (Wait { minimum: 20 }, BeforeWait(saved), Some(saved))
            }
            _ => return None,
        };
        Some(InterruptEffects {
            timing,
            condition_codes,
            stacked_condition_codes,
        })
    }
}

const fn cc(cleared: u8, set: u8) -> ConditionCodeEffects {
    ConditionCodeEffects {
        read: 0,
        calculated: 0,
        cleared,
        set,
        undefined: 0,
    }
}
