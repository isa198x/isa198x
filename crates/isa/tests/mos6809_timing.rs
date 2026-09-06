//! Motorola MC6809E (1984), Table 2 and Figure 18 note 4; programming
//! manual (1981), Appendix A, the four stack-operation entries.
use isa198x::mos6809::{
    lookup,
    timing::{StackConditionCodes, indexed_cost},
};

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
