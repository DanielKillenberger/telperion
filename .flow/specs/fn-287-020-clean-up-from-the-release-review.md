# 0.2.0 clean-up from the release review

## Goal & Context

Astra's release review (2026-10-16, `.flow/evidence/fn-237-release-the-tree-space-engine-replaces/ASTRA-RELEASE-REVIEW.md`, at F4 8dab10634), asked for by the owner ("figure out what can be cut/simplified or what we're doing stupidly"), and its incorporation into the release plan ("astra review is in, incorporate into the release plan"). These are the review's checked cuts and cheap simplifications before 0.2.0; none changes an approved tree.

## Requirements

- **R1 (no oracle in production):** the resident position programme's four pipelines (`emit`, `admit`, `radii`, `finish`; `generation.rs:206`, `io.rs:167`) are no longer created at production generator start; the old comparison route stays under `cfg(test)` where it supplies an oracle (`route_tests.rs`, `lobed_tests.rs:27`, `tests.rs:436`). CPU delivery and the shared leaf-placement programme stay. Proof: production initialisation creates no position program (a test), both profiles compile.
- **R2 (dead experiment):** `telperion-space`'s `closed_form/envelope.rs` and its tests (406 lines, no consumer; `rg 'StandIns|closed_form::envelope|super::envelope'`) are deleted with the `dead_code` allow; shared event/count machinery, `along.rs` and `Count::wood` (`kept/path.rs:286`) stay.
- **R3 (true metadata):** `docs/pipeline.md` describes the floating-point cycle and the fn-257 field as built; `telperion-space/src/lib.rs`'s "nothing is wired" note, `package.json`'s description, and the catalogue's `FLOWERS` dormancy text (`catalogue.rs:279`: flowering is live on Telperion and Laurelin) say what is true; derived parameter docs regenerated; no dial eligibility changes.
- **R4 (one gate resource declaration):** fn-271 R4's stale 20 GB is replaced by one measured `telperion-heavy` invocation for the gate, with its revision, concurrency and scope, and hosts and reviewers use it (fn-259 FRICTION.md: a 30-minute wait on 20 GB; the gate passed at 12 GB, two threads).
- **R5:** fn-186 (the no-geometry job tests the build without geometry) lands: the self dev-dependency without default features, geometry-only tests gated, both configurations' feature trees and test lists checked.

## Boundaries

Release item (fn-237). After fn-257 merges (R3's field description, and the shared generation files). The review's post-release items are fn-288..fn-290, fn-285 and fn-242.
