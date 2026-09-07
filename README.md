# Isa198x

Isa198x is the neutral executable instruction-set layer for the 198x family.
It contains dependency-free declarative ISA specifications and the
spec-driven disassemblers built on them.

| Crate | Role |
|---|---|
| `isa198x` | Instruction encoding truth, organised as one module per CPU family. |
| `isa198x-disasm` | Disassemblers depending only on `isa198x` and the standard library. |

The crates began in the Asm198x workspace. They moved here with their git
history when crates.io publication made Asm198x's ownership and raw-SHA
release coupling the wrong boundary. Their versions restart at `0.1.0` because
the previous `0.0.x` numbers were Asm198x workspace checkpoints, not releases
of these libraries.

CPU modules remain in one `isa198x` crate. A per-CPU crate split is reserved
for a measured need: a consumer requiring one isolated ISA, or material build
and packaging cost from the combined crate.

## 6809 timing and flags

`mos6809::timing::indexed_cost(postbyte)` reports the indexed-addressing
surcharge and extension-byte count for documented encodings.
`mos6809::Insn::stack_effects(mask)` reports complete push/pull timing and
whether the condition-code register is restored.
`mos6809::Insn::branch_effects(mode)` reports complete short/long branch
timing and condition-code read/write masks, including BRA/BRN/BSR and aliases.
`mos6809::Insn::memory_effects(mode, postbyte)` resolves complete timing for
ordinary memory forms, including indexed costs, and distinguishes calculated,
set, cleared, undefined, and preserved flags. Indexed forms require a documented
postbyte; a missing postbyte never produces a misleading base-only total.
`mos6809::Insn::immediate_cc_effects(mask)` resolves ANDCC/ORCC flags for the
encoded mask. `mos6809::Insn::fixed_inherent_effects()` covers accumulator
operations, NOP, ABX, RTS, MUL, SEX and DAA, with complete fixed costs.
`mos6809::Insn::transfer_effects(postbyte)` resolves TFR/EXG for documented
same-width register pairs, distinguishing CC preservation from loading CC
from another register. Its source notes record the conflicting TFR timing
entries and the selected C/D tables.
These follow Motorola's manufacturer tables, cited in the module.

These are the first pieces of the 6809 timing backfill, not full instruction
timing coverage. Interrupt-related instructions (SYNC, RTI, SWI/SWI2/SWI3,
CWAI) remain outside these tables; consumers must not
infer their effects from absent data. Asm198x's
6809 cycle coverage remains unchanged until those tables and capture exist.
