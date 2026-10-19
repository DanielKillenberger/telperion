# fn-293 R4: the carried-over contracts, reconciled

One row for each item in `PIPELINE-AUDIT.md`'s "Carried-over contracts" table and in R4's list, as they stand at task 7 (after tasks 1 to 6 and 8). The audit numbers them K1, K2 and K4 to K10; it has no K3. A row is kept, with a named reason, or removed, and the row says where. No contract a shipped consumer reads changes here (spec, Key context). AGENTS.md "Generator evolution" (owner, 2026-09-20) is abbreviated as **GE** below.

## The rows

| # | Contract | Fate | Reason and where |
|---|---|---|---|
| K1 | fn-257 R1, R1a, R1': "the field answers queries exactly as today", the cube-query contract | **Removed** (superseded) | Already marked superseded in fn-257's text, at lines 17, 18 and 38: "*(superseded by R8, 2026-10-16)*" (`b78d72f42`, host, on audit finding 1). R8 (owner, 2026-10-16) replaced the cube query with the grid, and fn-257 shipped R8. Nothing reads R1, R1a or R1' now. No edit was needed. |
| K2 | `docs/field-package.md`: the cube-query contract and "Migrating from 0.1.4" | **Contract removed; the migration note kept, with its audience named** | fn-257 R8 removed the cube query; the doc now describes the grid. The migration note is kept because fn-257 R8b asks for it ("arbitrary-cube queries go, with a migration note beside `cut`'s"). Its audience is a page on the published 0.1.4 package, such as the site (fn-257 R1b), and the note now says so (`docs/field-package.md`, "Migrating from 0.1.4"). `docs/principles.md:19` no longer describes the per-cube query; it names the device field's test. |
| K4 | `field.snapshot()`, the CPU field's index snapshot, as package API (README, `core.ts`, `bindings.mjs`) | **Removed** | fn-257 R8 deleted `FieldSnapshot` and `IndexSnapshot` (no Rust source names either). `README.md`'s 0.1.4 note lists `snapshot()` among what is gone, and `tests/browser/bindings.mjs:237` checks that the native boundary exports no `field_snapshot`. |
| K5 | `{ structure: true }`: six f64 and three u32 a node, root parent `0xffffffff`, kind structural 0 / branch 1 / twig 2 | **Kept** | A shipped consumer reads it: the main entry's `structure` output (`src/browser/core.ts:86`). `harness/parity.test.ts` pins its bytes native against Wasm for every preset (fn-198 host decision 10). The `kind` column is no longer the scaffold's. The engine's conversion derives it (`pipeline/space.rs:576-585`: structural at or above a share of the root radius, twig where a node has no child, branch otherwise), and `stage.rs::structure` writes it. Changing the layout would be an owner decision. |
| K6 | "one leaf mesh, twelve bytes a leaf - three Uint32 words" (`README.md:16-18`) | **Kept** | It is still the shipped encoding (`src/browser/core.ts:82-83`, decoded by `src/browser/leaf.ts`). fn-291's compaction is owner-justified (fn-291, 2026-10-16). The README's wood-chunk migration pointer is kept with its audience named, a consumer of 0.1.4's arrays, as fn-291 R4 asks ("its TypeScript types, README and a migration note say so"). |
| K7 | `tests/catalogue_identity.rs`: four FNV digests (four tests) "recorded on master before the catalogue (d6a3405c)", with a re-pin history | **Kept, test unchanged** | Tasks 3 and 8 relied on `every_refusal_is_the_refusal_it_was` to show that every refusal stayed the same, so the digests guard something production reads (`Family::validate`). Each re-pin names its spec. R4's list does not name K7. Whether to trim the comment changelog is under Open. |
| K8 | fn-242: "every shipped preset's field byte-identical", `telperion-field.wasm`, the "one-thread coarse field", "three published Wasm entries" | **R1 and R2 marked superseded; the entries sentence corrected** | R1 and R2 were written for the slim field module and the one-thread coarse field. fn-257 removed both, and `git ls-files` names neither. Each is now marked superseded by the spec's own re-scope (host, 2026-10-16). A lever that changes answers is judged under GE. The re-scope's "three published Wasm entries" now carries a note: the package publishes three entries (`telperion`, `telperion/field`, `telperion/field/voxelize`) over two Wasm modules. |
| K9 | `[inferred]` byte identity against the pre-engine build or the removed growth path | **Marked superseded, spec by spec (below)** | Each marked line cites GE or fn-181 and "fn-293 task 7". |
| K10 | `/canopy/spacing`, `/canopy/clump`, `/canopy/clumpSpan`: "kept on the wire for compatibility" (`docs/parameters.md`); `Family`'s scaffold rows on the wire | **Kept** | fn-237 line 74, on the owner's instruction to "incorporate into the release plan": "Not touched, as the review advises: ... the deprecated wire rows". This spec's Boundaries say the same. The scaffold's rows stay on the wire and stay bounded by `Family::validate` (fn-123 R1). Task 8 dropped only the scaffold-only preconditions (`inner_envelope`, `forks_placed`) and kept `resolved_growth`, because two refusal tests read it. |

### K9, spec by spec

| Spec | Line (at task 7) | Fate |
|---|---|---|
| fn-15 | R9 ("snapshot schema ... updated"); the cited memory entry `a-new-habitparams-row-changes-the-2026-09-26` | Marked: the specimen snapshot it names no longer exists (no `specimen/snapshot.rs`), so a new row changes no snapshot schema. The catalogue wire test and the generated tables still move with a new row. The memory entry itself is outside this task's Touches (Open). |
| fn-28 | "Determinism is untouched"; R4 (fn-11's tree byte for byte) | Marked superseded: fn-11's growth path was removed on 2026-10-02 (fn-181). The spec needs re-grounding on the engine, or closing (Open). |
| fn-41 | "Byte identity"; R2 (pinned stills byte-identical before and after the split) | **Kept, unchanged.** It is a refactor's own equivalence check, which GE keeps ("Use byte-equivalence checks where unchanged output is intended, including ... behavior-preserving optimizations"). It pins the stills before and after the refactor, not the pre-engine build. |
| fn-123 | line 25 (`maxNodes >= 1`: "four tests pin a zero ceiling on `generate` and the growth path") | Marked: fn-181 removed the growth path those tests pinned. The refusal's placement in `Family::validate` stands. Lines 26 and 27 pin no bytes and are unchanged. |
| fn-125 | "Unchanged output where intended"; R3 ("byte-identical to today's direct build"); R8 ("every round preset's wood bytes are unchanged") | Marked superseded by GE: the pre-engine direct build is gone (fn-198). |
| fn-125 | R1 (the plan layout) and "Twig runs" (`foliage/prepared.rs`) | Marked: task 5 removed `prepared.rs`'s stations from the renderer's production build with the CPU delivery. Only the renderer's tests read them, as the device's reference. |
| fn-126 | R2 (byte-identical to fn-125's); "No change to the algorithm's output. Speed only." | Marked superseded by GE. |
| fn-171 | "Plan layout" (fn-125's layout); "No change to what any tree looks like" | Marked: the layout note as for fn-125. The reference is now the engine's CPU cycle, which the device matches by lineage, not the pre-engine build. |
| fn-176 | "Plan layout"; "Full detail is the default" (byte-identical to fn-125 and fn-15); R1 (buffers byte-identical) | Marked, as for fn-125. |
| fn-240 | R2 ("bit for bit where the pin holds, or within a stated tolerance with the host's word") | Marked: under GE, a measured gain may move the value without asking, with the evidence GE names. Bit-for-bit equality is a verification aid where it holds. The predictor is one production reads (`expected_kept`), and this spec does not change it. |

## Notes for other specs

- **fn-289 (from task 6):** before task 6, the CPU reference (`prepare_whole_resident`) and the device already differed leaf for leaf. At seeds 1 and 7 the spruce's leaf counts differ by 2, and every leaf index differs (`BASELINE.md`, "After task 6"). Task 6's packed-frame move (1, 6 and 10 spruce leaves, at most 0.197 mm) is on the device side. No second path was built to match the two.

## Open

Questions for the owner. None blocks this spec.

1. **K7.** Should `catalogue_identity.rs`'s opening comment keep its per-spec re-pin history (the 237-line `//!` header, for four digests), or keep only the digests and the reason they exist? The test guards refusals production reads. The question is the changelog only.
2. **fn-28.** Its premise was fn-11's growth path, which fn-181 removed. Re-ground it on the engine (the mature tree is the product, AGENTS.md 2026-09-18), or close it?
3. **The memory entry** `a-new-habitparams-row-changes-the-2026-09-26` names `specimen/snapshot.rs`, which no longer exists. It sits outside this task's Touches (`.flow/memory/`). Retire it, or rewrite it around the catalogue wire test?
