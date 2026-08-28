# Isa198x

> Read [`PRINCIPLES.md`](PRINCIPLES.md) first.

Isa198x owns the 198x family's executable instruction-set specifications and
spec-driven disassemblers. It is neutral infrastructure consumed by Asm198x
and Emu198x.

## Boundaries

- `crates/isa` publishes as `isa198x` and stays dependency-free.
- `crates/isa-disasm` publishes as `isa198x-disasm` and depends only on
  `isa198x` plus the standard library.
- Hardware facts cite the umbrella `reference/` and `syntheses/` layers.
- Do not split one crate per CPU until an isolated consumer or measured build
  cost makes the additional release surface worthwhile.
- Cross-project ownership decisions live in `../../decisions/`; Isa198x-only
  decisions live in `decisions/`.
