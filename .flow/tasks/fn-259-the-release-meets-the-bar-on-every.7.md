---
satisfies: [R3]
---
# fn-259-the-release-meets-the-bar-on-every.7 Old-versus-new stills and the regression ledger for the owner

## Description
R3's packet: stills of every shipped species from 0.1.4 and F4 at the same pose and seed (default ages, and matched age where they differ, labelled), the regression ledger from task 5 and task 6 (each item, its measured size, the fidelity gain offered for it), and the questions the owner answers. Ends with NEEDS_HUMAN carrying the exact questions; the owner's words are recorded when they come.

**Size:** M
**Files:** `crates/telperion-render/examples/resident_stills.rs`, `.flow/evidence/fn-259-the-release-meets-the-bar-on-every/VERDICT.md`, compressed JPEG sheets beside it (docs/evidence-retention.md: sheets an owner verdict rests on are kept outside `raw/`)
**Touches:** [crates/telperion-render/examples/resident_stills.rs, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/VERDICT.md, .flow/evidence/fn-259-the-release-meets-the-bar-on-every/sheets/**]

### Approach
- F4 stills from the shipped resident device path only (`crates/telperion-render/examples/resident_stills.rs`, `Generator::prepare`), never `headless.rs` (it draws the CPU `mesh::build` reference). `resident_stills.rs` takes default age and seed only: add `AGE`/`SEED` overrides and use the hero pose from the tree's bounds, which both versions compute, so pairs match; the 0.1.4 side through task 4's adapter. Sheets with `scripts/contact-sheet.py`.
- The pixel helper (task 3) states how far each pair differs; the agent views at most four images per capture and makes no visual judgment of its own.
- Ledger: one row per regression (download included), its size from SCORECARD.md/AUDIT.md, and the fidelity gain offered named from recorded evidence (the owner's 2026-10-15 species verdicts, species specs), never invented.
- The owner's 2026-10-15 verdicts (fn-237) are cited; the question asked is old-versus-new per species and per ledger item.

### Investigation targets
**Required:**
- `.flow/specs/fn-237-release-the-tree-space-engine-replaces.md` - the owner's species verdicts table
- `docs/evidence-retention.md`
- `scripts/contact-sheet.py`

### Key context
The verdict is the owner's; the task is done when the packet is complete and the questions are asked. Its done summary carries NEEDS_HUMAN with the exact questions unless the owner has answered.
## Acceptance
- [ ] A sheet per shipped species, 0.1.4 beside F4 (F4 from the resident device path), same pose and seed, labelled with version, age and seed.
- [ ] The ledger lists every regression with its measured size and the evidence-cited fidelity gain.
- [ ] VERDICT.md ends with the owner's questions, one per species and per ledger item, and a place for the answers.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
