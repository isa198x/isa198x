//! Motorola MC6809–MC6809E Programming Manual (1981), Appendix A TFR/EXG
//! operations and register codes; Appendix C Table C-1 and Appendix D Table
//! D-1 for timing. These two detailed tables agree on TFR=6 and EXG=8 cycles.
//! Appendix F's opcode map instead prints TFR=7; this API follows C/D, not
//! a fabricated runtime range combining contradictory source entries.
//! [Cross-reference](https://www.maddes.net/m6809pm/appendix_c.htm),
//! [programming aid](https://www.maddes.net/m6809pm/appendix_d.htm),
//! [operations](https://www.maddes.net/m6809pm/appendix_a.htm).

use crate::mos6809::{Insn, Kind, transfer_reg_name};

/// CC is preserved or loaded wholesale from another byte register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TransferConditionCodes {
    /// All CC bits retain their original values, including CC-to-CC operations.
    Unchanged,
    /// All CC bits are copied from this register's incoming value, not computed
    /// as arithmetic flags. The register is `"a"`, `"b"`, or `"dp"`.
    LoadedFrom(&'static str),
}

/// Complete nominal timing and CC effects of a documented TFR/EXG postbyte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct TransferEffects {
    /// Full instruction cycles, excluding external stalls.
    pub cycles: u8,
    /// Width of each participating register in bytes (not total bytes moved).
    pub register_bytes: u8,
    /// CC bits copied into another register: all eight or none. Mere
    /// preservation, including CC-to-CC, is not counted as an input dependency.
    pub condition_codes_read: u8,
    /// Whether CC retains its value or is loaded from another register.
    pub condition_codes: TransferConditionCodes,
}

impl Insn {
    /// Resolve TFR/EXG timing and CC effects for the encoded register pair.
    /// Only documented, equal-width pairs are accepted; mixed-width or
    /// reserved register codes return `None`, as do other instruction kinds.
    ///
    /// ```
    /// use isa198x::mos6809::{lookup, timing::TransferConditionCodes};
    /// let e = lookup("exg").expect("EXG").transfer_effects(0xa8).expect("CC,A");
    /// assert_eq!(e.cycles, 8);
    /// assert_eq!(e.condition_codes_read, 0xff);
    /// assert_eq!(e.condition_codes, TransferConditionCodes::LoadedFrom("a"));
    /// ```
    #[must_use]
    pub fn transfer_effects(&self, postbyte: u8) -> Option<TransferEffects> {
        if self.undocumented {
            return None;
        }
        let (cycles, exchange) = match self.kind {
            Kind::Transfer(0x1f) => (6, false),
            Kind::Transfer(0x1e) => (8, true),
            _ => return None,
        };
        let source = postbyte >> 4;
        let destination = postbyte & 0xf;
        let source_name = transfer_reg_name(source)?;
        let destination_name = transfer_reg_name(destination)?;
        if (source ^ destination) & 8 != 0 {
            return None;
        }
        let distinct = source != destination;
        let condition_codes = if distinct && destination == 0xa {
            TransferConditionCodes::LoadedFrom(source_name)
        } else if distinct && exchange && source == 0xa {
            TransferConditionCodes::LoadedFrom(destination_name)
        } else {
            TransferConditionCodes::Unchanged
        };
        let reads_cc = distinct && (source == 0xa || (exchange && destination == 0xa));
        Some(TransferEffects {
            cycles,
            register_bytes: if source & 8 == 0 { 2 } else { 1 },
            condition_codes_read: if reads_cc { 0xff } else { 0 },
            condition_codes,
        })
    }
}
