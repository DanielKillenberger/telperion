---
title: A new HabitParams row changes the specimen snapshot layout; bump its schema
date: "2026-09-26"
track: bug
category: data
module: crates/telperion-core/src/pipeline/branching/specimen/snapshot.rs
tags: [catalogue, snapshot, bincode, schema, fn-61]
problem_type: data
symptoms: Old schema-3 snapshots would decode against a layout with two extra habit fields
root_cause: Snapshots are positional bincode; serde(default) only covers JSON
resolution_type: fix
---

## Problem
fn-61 added two rows to `HabitParams`. The catalogue, wire, browser metadata and generated files all followed, but three consumers outside the catalogue did not: the specimen snapshot (positional bincode, header `TLPS\x03`) kept its schema number while its layout changed, the catalogue's wire-rank test still expected `0..249`, and `harness/specimen.test.ts` pinned the schema. A row that reads the crown's height also used `planning.height`, which on the growth path sits a twig's reach inside the envelope.

## What Didn't Work
Updating only the row-count assertion and relying on `serde(default)`: defaults cover missing JSON keys, never an inserted field in a bincode sequence.

## Solution
Bump the snapshot header and doc (`pipeline/branching/specimen/snapshot.rs`), the browser `SpecimenSnapshot` schema (`src/browser/specimen.ts`), `harness/parity.test.ts`, `harness/specimen.test.ts`, the schema test in `suite/generation_limits.rs` and README; update both catalogue count tests (`every_wire_row_has_one_entry`, `every_row_has_its_own_rank_on_the_wire`). Measure crown height against `self.envelope.height`, never `planning`.

## Prevention
When adding a field to any serde struct inside `Specimen`, grep `TLPS`, `schema: ` and `toBe(<n>)` and bump together; run the whole `catalogue::tests` module, not just one count test.
