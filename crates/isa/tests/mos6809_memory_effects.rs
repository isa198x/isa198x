//! Expectations transcribed by instruction family from Motorola's 1981
//! programming manual, Appendix F Table F-1 and Appendix A CC entries.
use isa198x::mos6809::{Insn, Kind, SET, lookup, rows, timing::indexed_cost};

#[test]
fn every_ordinary_memory_row_matches_the_manufacturer_tables() {
    // immediate/direct/indexed/extended; zero means no form, not zero cycles.
    // CC order E F H I N Z V C: '-' preserved, '*' calculated, '?' undefined.
    let families = [
        (
            "lda ldb anda andb bita bitb ora orb eora eorb",
            [2, 4, 4, 5],
            0,
            "----**0-",
        ),
        ("sta stb", [0, 4, 4, 5], 0, "----**0-"),
        ("ldd ldx ldu", [3, 5, 5, 6], 0, "----**0-"),
        ("std stx stu", [0, 5, 5, 6], 0, "----**0-"),
        ("ldy lds", [4, 6, 6, 7], 0, "----**0-"),
        ("sty sts", [0, 6, 6, 7], 0, "----**0-"),
        ("adda addb", [2, 4, 4, 5], 0, "--*-****"),
        ("adca adcb", [2, 4, 4, 5], 1, "--*-****"),
        ("suba subb cmpa cmpb", [2, 4, 4, 5], 0, "--?-****"),
        ("sbca sbcb", [2, 4, 4, 5], 1, "--?-****"),
        ("addd subd cmpx", [4, 6, 6, 7], 0, "----****"),
        ("cmpd cmpy cmpu cmps", [5, 7, 7, 8], 0, "----****"),
        ("clr", [0, 6, 6, 7], 0, "----0100"),
        ("inc dec", [0, 6, 6, 7], 0, "----***-"),
        ("tst", [0, 6, 6, 7], 0, "----**0-"),
        ("com", [0, 6, 6, 7], 0, "----**01"),
        ("neg asl lsl", [0, 6, 6, 7], 0, "--?-****"),
        ("lsr", [0, 6, 6, 7], 0, "----0*-*"),
        ("ror", [0, 6, 6, 7], 1, "----**-*"),
        ("asr", [0, 6, 6, 7], 0, "--?-**-*"),
        ("rol", [0, 6, 6, 7], 1, "----****"),
        ("jmp", [0, 3, 3, 4], 0, "--------"),
        ("jsr", [0, 7, 7, 8], 0, "--------"),
        ("leax leay", [0, 0, 4, 0], 0, "-----*--"),
        ("leas leau", [0, 0, 4, 0], 0, "--------"),
    ];
    let mut expected_rows = Vec::new();
    for (names, timings, read, flags) in families {
        for name in names.split_whitespace() {
            let insn = lookup(name).expect("documented mnemonic");
            for (mode, base) in ["immediate", "direct", "indexed", "extended"]
                .into_iter()
                .zip(timings)
            {
                let postbytes: Vec<_> = if mode == "indexed" {
                    (0..=255).map(Some).collect()
                } else {
                    vec![None]
                };
                if base != 0 {
                    expected_rows.push((name, mode));
                }
                for postbyte in postbytes {
                    let extra = postbyte.map_or(Some(0), |b| indexed_cost(b).map(|c| c.cycles));
                    let actual = insn.memory_effects(mode, postbyte);
                    if base == 0 || extra.is_none() {
                        assert!(actual.is_none(), "{name} {mode} {postbyte:?}");
                        continue;
                    }
                    let e = actual.expect("documented form and postbyte");
                    assert_eq!(
                        e.cycles,
                        base + extra.expect("valid postbyte"),
                        "{name} {mode}"
                    );
                    assert_eq!(e.condition_codes.read, read, "{name} {mode}");
                    let cc = e.condition_codes;
                    for (bit, effect) in flags.bytes().rev().enumerate() {
                        let mask = 1 << bit;
                        let outcomes = [
                            (b'-', cc.preserved()),
                            (b'*', cc.calculated),
                            (b'0', cc.cleared),
                            (b'1', cc.set),
                            (b'?', cc.undefined),
                        ];
                        for (category, bits) in outcomes {
                            assert_eq!(bits & mask != 0, category == effect, "{name} CC bit {bit}");
                        }
                    }
                }
            }
        }
    }
    assert_eq!(expected_rows.len(), 195);
    let actual_rows: Vec<_> = rows()
        .filter(|row| {
            lookup(row.mnemonic)
                .expect("row")
                .memory_effects(&row.mode, (row.mode == "indexed").then_some(0x84))
                .is_some()
        })
        .collect();
    assert_eq!(actual_rows.len(), expected_rows.len());
    for row in actual_rows {
        assert!(expected_rows.contains(&(row.mnemonic, row.mode.as_ref())));
    }
    // No new ordinary memory instruction can silently fall through the table.
    for insn in SET.iter().filter(|i| matches!(i.kind, Kind::Mem { .. })) {
        assert_eq!(
            expected_rows.iter().any(|(name, _)| *name == insn.mnemonic),
            !matches!(insn.mnemonic, "andcc" | "orcc" | "cwai"),
            "{}",
            insn.mnemonic
        );
    }
}

#[test]
fn incomplete_or_mismatched_inputs_never_produce_complete_timings() {
    for insn in SET {
        assert!(insn.memory_effects("indexed", None).is_none());
        for mode in [
            "",
            "relative",
            "relative long",
            "inherent",
            "register pair",
            "register set",
        ] {
            assert!(insn.memory_effects(mode, None).is_none());
        }
        for mode in ["immediate", "direct", "extended"] {
            assert!(insn.memory_effects(mode, Some(0x84)).is_none());
        }
    }
    let forged = Insn {
        mnemonic: "lda",
        kind: Kind::Mem {
            imm: &[0x12],
            direct: &[],
            indexed: &[],
            extended: &[],
            width: 1,
        },
        undocumented: false,
    };
    assert!(forged.memory_effects("immediate", None).is_none());
    let undocumented = Insn {
        undocumented: true,
        kind: Kind::Mem {
            imm: &[0x86],
            direct: &[],
            indexed: &[],
            extended: &[],
            width: 1,
        },
        ..forged
    };
    assert!(undocumented.memory_effects("immediate", None).is_none());
    for name in [
        "andcc", "orcc", "cwai", "rti", "sync", "reset", "hcf", "rhf",
    ] {
        assert!(
            lookup(name)
                .expect("instruction")
                .memory_effects("immediate", None)
                .is_none()
        );
    }
}
