//! Motorola MC6809E (1984), Table 2 and Figure 18 note 4; programming
//! manual (1981), Appendix A, the four stack-operation entries.
use isa198x::mos6809::{
    Insn, Kind, SET, lookup, rows,
    timing::{StackConditionCodes, indexed_cost},
};

/// Motorola programming manual (1981), Appendix F Table F-1, and
/// Appendix A's branch operation/CC entries. Expectations are mnemonic-based
/// so this also checks the production opcode-to-condition mapping.
#[test]
fn every_branch_row_has_complete_timing_and_flag_dependencies() {
    let branches = [
        ("bra", 3, 5, 0, 0),
        ("brn", 3, 5, 0, 0),
        ("bsr", 7, 9, 0, 0),
        ("bhi", 3, 5, 1, 0x05),
        ("bls", 3, 5, 1, 0x05),
        ("bcc", 3, 5, 1, 0x01),
        ("bhs", 3, 5, 1, 0x01),
        ("bcs", 3, 5, 1, 0x01),
        ("blo", 3, 5, 1, 0x01),
        ("bne", 3, 5, 1, 0x04),
        ("beq", 3, 5, 1, 0x04),
        ("bvc", 3, 5, 1, 0x02),
        ("bvs", 3, 5, 1, 0x02),
        ("bpl", 3, 5, 1, 0x08),
        ("bmi", 3, 5, 1, 0x08),
        ("bge", 3, 5, 1, 0x0a),
        ("blt", 3, 5, 1, 0x0a),
        ("bgt", 3, 5, 1, 0x0e),
        ("ble", 3, 5, 1, 0x0e),
    ];
    assert_eq!(
        SET.iter()
            .filter(|i| matches!(i.kind, Kind::Branch { .. }))
            .count(),
        branches.len(),
        "new branches must join the manufacturer expectations"
    );
    for (name, short, long, extra, read) in branches {
        let insn = lookup(name).expect("documented branch");
        for (mode, base, taken) in [("relative", short, 0), ("relative long", long, extra)] {
            let e = insn.branch_effects(mode).expect("documented branch row");
            assert_eq!(
                (e.cycles.base, e.cycles.branch_taken),
                (base, taken),
                "{name} {mode}"
            );
            assert_eq!(e.cycles.page_cross, 0, "{name} {mode}");
            assert_eq!(e.condition_codes_read, read, "{name} {mode}");
            assert_eq!(e.condition_codes_written, 0, "{name} {mode}");
        }
        for mode in [
            "",
            "inherent",
            "immediate",
            "direct",
            "indexed",
            "extended",
            "register pair",
            "register set",
        ] {
            assert!(insn.branch_effects(mode).is_none(), "{name} {mode}");
        }
    }
    let mut covered = 0;
    for row in rows() {
        let insn = lookup(row.mnemonic).expect("declared row");
        let effects = insn.branch_effects(&row.mode);
        assert_eq!(effects.is_some(), matches!(insn.kind, Kind::Branch { .. }));
        covered += usize::from(effects.is_some());
    }
    assert_eq!(covered, 38);
}

#[test]
fn missing_or_undocumented_branch_encodings_are_not_zero_cost() {
    for insn in SET
        .iter()
        .filter(|i| !matches!(i.kind, Kind::Branch { .. }))
    {
        assert!(insn.branch_effects("relative").is_none());
        assert!(insn.branch_effects("relative long").is_none());
    }
    // Insn and Kind are public: do not assume every constructed instance came
    // from SET, or that a plausible mnemonic makes its bytes documented.
    for (short, long, undocumented) in [
        (&[][..], &[][..], false),
        (&[0x20][..], &[0x16][..], true),
        (&[0x10, 0x26][..], &[0x26][..], false),
        (&[0x12][..], &[0x10, 0x20][..], false),
    ] {
        let insn = Insn {
            mnemonic: "bra",
            kind: Kind::Branch { short, long },
            undocumented,
        };
        assert!(insn.branch_effects("relative").is_none());
        assert!(insn.branch_effects("relative long").is_none());
    }
}

#[test]
fn every_indexed_postbyte_matches_the_manufacturer_matrix() {
    // (base encoding, cycle addition, bytes after postbyte).
    // The four RR values apply to register indexing; xx is explicitly ignored
    // for PCR. Extended indirect has no RR field and is checked separately.
    let rows = [
        (0x84, 0, 0),
        (0x88, 1, 1),
        (0x89, 4, 2),
        (0x86, 1, 0),
        (0x85, 1, 0),
        (0x8b, 4, 0),
        (0x80, 2, 0),
        (0x81, 3, 0),
        (0x82, 2, 0),
        (0x83, 3, 0),
        (0x8c, 1, 1),
        (0x8d, 5, 2),
        (0x94, 3, 0),
        (0x98, 4, 1),
        (0x99, 7, 2),
        (0x96, 4, 0),
        (0x95, 4, 0),
        (0x9b, 7, 0),
        (0x91, 6, 0),
        (0x93, 6, 0),
        (0x9c, 4, 1),
        (0x9d, 8, 2),
    ];
    let mut expected = [None; 256];
    expected[..128].fill(Some((1, 0)));
    for (base, cycles, bytes) in rows {
        for reg in [0x00, 0x20, 0x40, 0x60] {
            expected[base | reg] = Some((cycles, bytes));
        }
    }
    expected[0x9f] = Some((5, 2));
    assert_eq!(expected.iter().flatten().count(), 217);
    for postbyte in 0..=255u8 {
        assert_eq!(
            indexed_cost(postbyte).map(|c| (c.cycles, c.extension_bytes)),
            expected[postbyte as usize],
            "postbyte ${postbyte:02X}"
        );
    }
}

#[test]
fn encoded_zero_displacement_is_not_the_zero_offset_mode() {
    assert_eq!(indexed_cost(0).expect("five bit zero").cycles, 1);
    assert_eq!(indexed_cost(0x84).expect("no offset").cycles, 0);
    assert_eq!(indexed_cost(0x89).expect("register offset").cycles, 4);
    assert_eq!(indexed_cost(0x8d).expect("PCR offset").cycles, 5);
    for undocumented in [0x87, 0x8f, 0x90, 0x92, 0xbf, 0xdf, 0xff] {
        assert!(indexed_cost(undocumented).is_none());
    }
}

#[test]
fn every_stack_mask_has_exact_byte_cost_and_condition_code_effect() {
    for (name, pull) in [
        ("pshs", false),
        ("pshu", false),
        ("puls", true),
        ("pulu", true),
    ] {
        let insn = lookup(name).expect("stack instruction");
        for mask in 0..=255u8 {
            let e = insn
                .stack_effects(mask)
                .expect("all register masks defined");
            let bytes: u8 = [1, 1, 1, 1, 2, 2, 2, 2]
                .iter()
                .enumerate()
                .filter(|(bit, _)| mask & (1 << bit) != 0)
                .map(|(_, &width)| width)
                .sum();
            assert_eq!(e.transferred_bytes, bytes, "{name} ${mask:02X}");
            assert_eq!(e.cycles, 5 + bytes, "{name} ${mask:02X}");
            assert_eq!(
                e.condition_codes,
                if pull && mask & 1 != 0 {
                    StackConditionCodes::Restored
                } else {
                    StackConditionCodes::Unchanged
                }
            );
        }
        assert_eq!(insn.stack_effects(0).expect("empty mask").cycles, 5);
        assert_eq!(insn.stack_effects(255).expect("full mask").cycles, 17);
    }
    assert!(lookup("rts").expect("RTS").stack_effects(255).is_none());
}
