//! Motorola 1981 programming manual, Appendix A and Appendix F Table F-1.
use isa198x::mos6809::{Insn, Kind, SET, lookup};

#[test]
fn every_cc_mask_matches_the_operation_for_every_incoming_cc() {
    for name in ["andcc", "orcc"] {
        let insn = lookup(name).expect("CC instruction");
        for mask in 0..=255u8 {
            let e = insn.immediate_cc_effects(mask).expect("all masks valid");
            assert_eq!(e.cycles, 3);
            let cc = e.condition_codes;
            assert_eq!((cc.read, cc.calculated, cc.undefined), (0, 0, 0));
            assert_eq!(cc.cleared & cc.set, 0);
            for incoming in 0..=255u8 {
                let expected = if name == "andcc" {
                    incoming & mask
                } else {
                    incoming | mask
                };
                let actual = (incoming & cc.preserved()) | cc.set;
                assert_eq!(actual, expected, "{name} #{mask:02x}, CC={incoming:02x}");
            }
        }
    }
    for insn in SET {
        assert_eq!(
            insn.immediate_cc_effects(0xff).is_some(),
            matches!(insn.mnemonic, "andcc" | "orcc")
        );
    }
}

#[test]
fn accumulator_forms_share_flags_but_not_memory_cycle_costs() {
    for name in [
        "neg", "com", "lsr", "ror", "asr", "asl", "lsl", "rol", "dec", "inc", "tst", "clr",
    ] {
        let memory = lookup(name)
            .expect("memory operation")
            .memory_effects("direct", None)
            .expect("direct");
        assert_eq!(memory.cycles, 6);
        for register in ['a', 'b'] {
            let mnemonic = format!("{name}{register}");
            let e = lookup(&mnemonic)
                .expect("accumulator operation")
                .fixed_inherent_effects()
                .expect("fixed form");
            assert_eq!(e.cycles, 2, "{mnemonic}");
            assert_eq!(e.condition_codes, memory.condition_codes, "{mnemonic}");
        }
    }
}

#[test]
fn miscellaneous_inherent_timings_and_flags_match_the_manual() {
    for (name, cycles, read, calculated, undefined) in [
        ("nop", 2, 0, 0, 0),
        ("abx", 3, 0, 0, 0),
        ("rts", 5, 0, 0, 0),
        ("mul", 11, 0, 0x05, 0),
        ("sex", 2, 0, 0x0c, 0),
        ("daa", 2, 0x21, 0x0d, 0x02),
    ] {
        let e = lookup(name)
            .expect("instruction")
            .fixed_inherent_effects()
            .expect("fixed timing");
        assert_eq!(e.cycles, cycles, "{name}");
        let cc = e.condition_codes;
        assert_eq!(
            (cc.read, cc.calculated, cc.undefined),
            (read, calculated, undefined),
            "{name}"
        );
        assert_eq!((cc.cleared, cc.set), (0, 0));
        assert_eq!(cc.preserved(), !(calculated | undefined));
    }
    let mut covered = 0;
    for insn in SET {
        let expected = matches!(insn.kind, Kind::Inherent(_))
            && !matches!(
                insn.mnemonic,
                "sync" | "rti" | "swi" | "swi2" | "swi3" | "reset" | "hcf" | "rhf"
            );
        assert_eq!(
            insn.fixed_inherent_effects().is_some(),
            expected,
            "{}",
            insn.mnemonic
        );
        covered += usize::from(expected);
    }
    assert_eq!(covered, 30);
}

#[test]
fn unsupported_opcode_shapes_and_undocumented_inputs_stay_unknown() {
    for opcode in [
        &[][..],
        &[0x41][..],
        &[0x51][..],
        &[0x10, 0x12][..],
        &[0x12, 0][..],
    ] {
        let insn = Insn {
            mnemonic: "nop",
            kind: Kind::Inherent(opcode),
            undocumented: false,
        };
        assert!(insn.fixed_inherent_effects().is_none());
    }
    let undocumented = Insn {
        mnemonic: "nop",
        kind: Kind::Inherent(&[0x12]),
        undocumented: true,
    };
    assert!(undocumented.fixed_inherent_effects().is_none());
    for (imm, width, undocumented) in [
        (&[0x1c][..], 2, false),
        (&[0x1c][..], 1, true),
        (&[0x3c][..], 1, false),
        (&[0x10, 0x1c][..], 1, false),
    ] {
        let insn = Insn {
            mnemonic: "andcc",
            kind: Kind::Mem {
                imm,
                direct: &[],
                indexed: &[],
                extended: &[],
                width,
            },
            undocumented,
        };
        assert!(insn.immediate_cc_effects(0).is_none());
    }
}
