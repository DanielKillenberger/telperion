---
title: A new catalogue row moves the wire tests and the generated tables together
date: "2026-09-26"
track: bug
category: data
module: crates/telperion-core/src/catalogue/tests.rs
tags: [catalogue, wire, generated, parameters, fn-61, fn-293]
problem_type: data
symptoms: A row added to a parameter group passes its own test but fails the catalogue's wire tests, the reference test or the Wasm build
root_cause: The catalogue's row count, the wire ranks and three generated tables are each held to the catalogue separately
resolution_type: fix
---

## Problem
fn-61 added two rows to `HabitParams`. The catalogue and wire followed, but the consumers held to the catalogue separately did not. At the time a third was the growth path's specimen snapshot (positional bincode); the growth path and its snapshot were removed (fn-181, 2026-10-02), so no snapshot schema exists to bump today (rewritten 2026-10-19, fn-293).

## What holds now
A new row moves, together:
- the catalogue's wire tests: `every_wire_row_has_one_entry` and `every_row_has_its_own_rank_on_the_wire` (`crates/telperion-core/src/catalogue/tests.rs`);
- the generated tables: `docs/parameters.md` and the browser's parameter metadata (`src/browser/parameters.generated.ts`, held by `crates/telperion-core/tests/parameter_reference.rs`), and `src/browser/presets.generated.ts` (held by `scripts/build-wasm.mjs`); rewrite them with `TELPERION_WRITE_REFERENCE=1` and commit them;
- the catalogue digests in `crates/telperion-core/tests/catalogue_identity.rs` when the row changes a family, walk, override or refusal text.

## Prevention
Run the whole `catalogue::tests` module and `parameter_reference`, not one count test, and regenerate the tables rather than editing them by hand. A row that reads the crown's height measures it against the grown tree's envelope, never a planning value.
