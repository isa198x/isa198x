//! Motorola 1981 manual Appendix A (TFR/EXG) and C/D timing tables.
use isa198x::mos6809::{Insn, Kind, SET, lookup, timing::TransferConditionCodes};

#[test]
fn all_postbytes_match_the_documented_register_pair_matrix() {
    let registers = [
        (0, "d", 2),
        (1, "x", 2),
        (2, "y", 2),
        (3, "u", 2),
        (4, "s", 2),
        (5, "pc", 2),
        (8, "a", 1),
        (9, "b", 1),
        (10, "cc", 1),
        (11, "dp", 1),
    ];
    for (name, cycles) in [("tfr", 6), ("exg", 8)] {
        let insn = lookup(name).expect("transfer instruction");
        let mut valid = 0;
        for postbyte in 0..=255u8 {
            let source = registers.iter().find(|r| r.0 == postbyte / 16);
            let destination = registers.iter().find(|r| r.0 == postbyte % 16);
            let expected = source.zip(destination).filter(|(s, d)| s.2 == d.2);
            let actual = insn.transfer_effects(postbyte);
            assert_eq!(
                actual.is_some(),
                expected.is_some(),
                "{name} ${postbyte:02x}"
            );
            let Some((s, d)) = expected else {
                continue;
            };
            valid += 1;
            let e = actual.expect("documented same-width pair");
            assert_eq!((e.cycles, e.register_bytes), (cycles, s.2));
            let distinct = s.1 != d.1;
            let expected_cc = if distinct && d.1 == "cc" {
                TransferConditionCodes::LoadedFrom(s.1)
            } else if distinct && name == "exg" && s.1 == "cc" {
                TransferConditionCodes::LoadedFrom(d.1)
            } else {
                TransferConditionCodes::Unchanged
            };
            assert_eq!(e.condition_codes, expected_cc, "{name} ${postbyte:02x}");
            let read = distinct && (s.1 == "cc" || (name == "exg" && d.1 == "cc"));
            assert_eq!(e.condition_codes_read, if read { 255 } else { 0 });
        }
        assert_eq!(valid, 52); // six word registers squared + four byte registers squared
    }
}

#[test]
fn only_documented_transfer_kinds_report_effects() {
    for insn in SET {
        assert_eq!(
            insn.transfer_effects(0x01).is_some(),
            matches!(insn.mnemonic, "tfr" | "exg")
        );
    }
    for (opcode, undocumented) in [(0x12, false), (0x1f, true), (0x1e, true)] {
        let insn = Insn {
            mnemonic: "tfr",
            kind: Kind::Transfer(opcode),
            undocumented,
        };
        assert!(insn.transfer_effects(0x01).is_none());
    }
}
