# Keep CPU families as modules until dependency granularity has a consumer

**Status:** Active

**Decided:** 2026-08-28

## Decision

Isa198x initially publishes two crates: `isa198x`, containing every supported
CPU family as modules, and `isa198x-disasm`, containing the corresponding
spec-driven disassemblers.

Repository extraction and per-CPU crate extraction solve different problems.
The new repository gives the shared layer neutral ownership, independent
versions, crates.io delivery and a release cadence that does not follow the
Asm198x binary. Per-CPU crates would reduce the dependency and compilation
surface for a consumer that needs only one CPU, but no current consumer has
shown that cost.

Split `isa198x` into a core plus per-CPU crates only when either:

1. a consumer needs one CPU in isolation and the combined crate creates a
   material build, binary-size or policy cost; or
2. a CPU family needs an independently evolving API or dependency surface.

Until then, modules preserve the seam without multiplying public packages,
versions, release notes and Trusted Publisher configuration.
