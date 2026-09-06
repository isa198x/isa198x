//! Operand-resolved 6809 timing, not a claim of full instruction coverage.
//!
//! Source: Motorola, *MC6809E HMOS 8-Bit Microprocessor* (1984), Table 2
//! (indexed addressing) and Figure 18 note 4 (push/pull timing).
//! [Manufacturer datasheet scan](https://datasheets.pl/elementy_czynne/IC/MC/MC6809E-4.pdf).
//! Stack condition-code effects: Motorola, *MC6809–MC6809E Programming Manual*
//! (1981), Appendix A, PSHS/PSHU/PULS/PULU operation and condition-code entries.
//!
//! Indexed costs are additions to an instruction's indexed base cost, not
//! complete instruction timings. They depend on the encoded postbyte, not
//! the value of the effective address: no page-cross penalty is involved.
//! Undocumented postbytes return `None`, never a fabricated zero.
//! Costs are nominal processor cycles, excluding external stalls.

/// Extra work and extension bytes selected by an indexed postbyte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct IndexedCost {
    /// Cycles added to the instruction's indexed base timing.
    pub cycles: u8,
    /// Bytes after the postbyte (excludes the opcode and postbyte themselves).
    pub extension_bytes: u8,
}

/// Resolve Table 2 for an encoded postbyte. `None` means Motorola does not
/// document that encoding; it does not mean a zero-cost addressing mode.
/// PCR register-field bits are don't-care as specified by `1xx01100/01`;
/// extended indirect, in contrast, is documented only as `$9F`.
///
/// ```
/// use isa198x::mos6809::timing::indexed_cost;
/// let cost = indexed_cost(0x99).expect("documented 16-bit indirect offset");
/// assert_eq!((cost.cycles, cost.extension_bytes), (7, 2));
/// assert!(indexed_cost(0x90).is_none()); // indirect single increment is undefined
/// ```
#[must_use]
pub const fn indexed_cost(postbyte: u8) -> Option<IndexedCost> {
    let (cycles, extension_bytes) = if postbyte & 0x80 == 0 {
        (1, 0) // signed five-bit displacement, including the encoded zero
    } else {
        match postbyte & 0x1f {
            0x00 | 0x02 => (2, 0),
            0x01 | 0x03 => (3, 0),
            0x04 => (0, 0),
            0x05 | 0x06 => (1, 0),
            0x08 | 0x0c => (1, 1),
            0x09 => (4, 2),
            0x0b => (4, 0),
            0x0d => (5, 2),
            0x11 | 0x13 => (6, 0),
            0x14 => (3, 0),
            0x15 | 0x16 => (4, 0),
            0x18 | 0x1c => (4, 1),
            0x19 => (7, 2),
            0x1b => (7, 0),
            0x1d => (8, 2),
            0x1f if postbyte == 0x9f => (5, 2),
            _ => return None,
        }
    };
    Some(IndexedCost {
        cycles,
        extension_bytes,
    })
}

/// A stack instruction either preserves CC or loads its complete saved value.
/// Restoring CC is not an arithmetic flag calculation or a known set/clear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum StackConditionCodes {
    /// No condition-code bit is modified.
    Unchanged,
    /// All condition-code bits are loaded from the saved byte on the stack.
    Restored,
}

/// Exact operand-resolved timing and CC effect of PSHS/PSHU/PULS/PULU.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct StackEffects {
    /// Number of data bytes pushed/pulled, excluding instruction bytes.
    pub transferred_bytes: u8,
    /// Complete instruction cost: five plus one per transferred byte.
    pub cycles: u8,
    /// Whether the instruction preserves CC or restores its stacked value.
    pub condition_codes: StackConditionCodes,
}

impl super::Insn {
    /// Resolve a stack instruction's register-mask-dependent timing and flags.
    /// All 256 masks are defined, including the empty set. Other instruction
    /// kinds return `None`; no timing is inferred for them from their operands.
    #[must_use]
    pub fn stack_effects(&self, mask: u8) -> Option<StackEffects> {
        let super::Kind::Stack { opcode, .. } = self.kind else {
            return None;
        };
        if !matches!(opcode, 0x34..=0x37) {
            return None;
        }
        // CC/A/B/DP are bytes; X/Y/other-stack/PC are words.
        let transferred_bytes =
            (mask & 0x0f).count_ones() as u8 + 2 * (mask & 0xf0).count_ones() as u8;
        let condition_codes = if matches!(opcode, 0x35 | 0x37) && mask & 1 != 0 {
            StackConditionCodes::Restored
        } else {
            StackConditionCodes::Unchanged
        };
        Some(StackEffects {
            transferred_bytes,
            cycles: 5 + transferred_bytes,
            condition_codes,
        })
    }
}
