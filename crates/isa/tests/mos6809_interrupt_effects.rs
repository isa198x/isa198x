//! Motorola 1981 programming manual, Appendix A operations and D-1 timings.
use isa198x::mos6809::{
    Insn, Kind, SET, lookup, rows,
    timing::{ConditionCodeEffects, InterruptConditionCodes, InterruptTiming},
};

fn apply(cc: ConditionCodeEffects, incoming: u8) -> u8 {
    assert_eq!((cc.read, cc.calculated, cc.undefined), (0, 0, 0));
    assert_eq!(cc.cleared & cc.set, 0);
    (incoming & cc.preserved()) | cc.set
}

#[test]
fn software_interrupts_save_cc_before_changing_live_interrupt_masks() {
    for (name, cycles, live_set) in [("swi", 19, 0xd0), ("swi2", 20, 0x80), ("swi3", 20, 0x80)] {
        let e = lookup(name)
            .expect("SWI family")
            .interrupt_effects(None)
            .expect("interrupt");
        assert_eq!(e.timing, InterruptTiming::Fixed(cycles));
        assert_eq!(e.timing.bounds(), (cycles, Some(cycles)));
        assert!(e.timing.rti_cycles(0x80).is_none());
        let InterruptConditionCodes::AtHandlerEntry(live) = e.condition_codes else {
            panic!("expected handler entry");
        };
        let stacked = e.stacked_condition_codes.expect("saved CC");
        for incoming in 0..=255u8 {
            assert_eq!(apply(stacked, incoming), incoming | 0x80);
            assert_eq!(apply(live, incoming), incoming | live_set);
        }
    }
}

#[test]
fn rti_is_bounded_and_selected_by_the_restored_e_bit() {
    let e = lookup("rti")
        .expect("RTI")
        .interrupt_effects(None)
        .expect("interrupt");
    assert_eq!(e.timing.bounds(), (6, Some(15)));
    assert_eq!(e.condition_codes, InterruptConditionCodes::Restored);
    assert!(e.stacked_condition_codes.is_none());
    for saved_cc in 0..=255u8 {
        assert_eq!(
            e.timing.rti_cycles(saved_cc),
            Some(if saved_cc < 128 { 6 } else { 15 })
        );
    }
}

#[test]
fn waits_have_no_finite_ceiling_and_cwai_sets_e_after_masking() {
    let sync = lookup("sync")
        .expect("SYNC")
        .interrupt_effects(None)
        .expect("interrupt");
    assert_eq!(sync.timing.bounds(), (4, None));
    assert!(sync.stacked_condition_codes.is_none());
    let InterruptConditionCodes::BeforeWait(sync_cc) = sync.condition_codes else {
        panic!("expected pre-wait phase");
    };
    assert_eq!(sync_cc.preserved(), 255);
    for mask in 0..=255u8 {
        let e = lookup("cwai")
            .expect("CWAI")
            .interrupt_effects(Some(mask))
            .expect("mask");
        assert_eq!(e.timing.bounds(), (20, None));
        let InterruptConditionCodes::BeforeWait(live) = e.condition_codes else {
            panic!("expected pre-wait phase");
        };
        let stacked = e.stacked_condition_codes.expect("CWAI saved CC");
        for incoming in 0..=255u8 {
            let expected = (incoming & mask) | 0x80;
            assert_eq!(apply(live, incoming), expected);
            assert_eq!(apply(stacked, incoming), expected);
            assert!(e.timing.rti_cycles(incoming).is_none());
            assert!(sync.timing.rti_cycles(incoming).is_none());
        }
    }
}

#[test]
fn missing_extra_and_unsupported_operands_are_rejected() {
    for insn in SET {
        assert_eq!(
            insn.interrupt_effects(None).is_some(),
            matches!(insn.mnemonic, "swi" | "swi2" | "swi3" | "rti" | "sync")
        );
        assert_eq!(
            insn.interrupt_effects(Some(0)).is_some(),
            insn.mnemonic == "cwai"
        );
    }
    for (opcode, undocumented) in [
        (&[0x3f][..], true),
        (&[0x10, 0x3b][..], false),
        (&[0x3c][..], false),
    ] {
        let insn = Insn {
            mnemonic: "swi",
            kind: Kind::Inherent(opcode),
            undocumented,
        };
        assert!(insn.interrupt_effects(None).is_none());
    }
}

#[test]
fn every_documented_row_has_exactly_one_timing_effects_path() {
    let mut documented = 0;
    for row in rows() {
        let insn = lookup(row.mnemonic).expect("declared instruction");
        let mode = row.mode.as_ref();
        let paths = [
            insn.memory_effects(mode, (mode == "indexed").then_some(0x84))
                .is_some(),
            insn.branch_effects(mode).is_some(),
            mode == "register set" && insn.stack_effects(0x02).is_some(),
            mode == "register pair" && insn.transfer_effects(0x01).is_some(),
            mode == "immediate" && insn.immediate_cc_effects(0xff).is_some(),
            mode == "inherent" && insn.fixed_inherent_effects().is_some(),
            matches!(mode, "immediate" | "inherent")
                && insn
                    .interrupt_effects((mode == "immediate").then_some(0xff))
                    .is_some(),
        ];
        assert_eq!(
            paths.into_iter().filter(|covered| *covered).count(),
            usize::from(!row.undocumented),
            "{} {mode}",
            row.mnemonic
        );
        documented += usize::from(!row.undocumented);
    }
    assert_eq!(documented, 277);
}
