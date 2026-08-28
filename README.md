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
