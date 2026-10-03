---
satisfies: [R1]
---
# fn-190-one-growth-law-species-are-points-in-a.4 Date palm (Corner): model specification and today's palm baseline

## Description
Stage A for the palm, reading only plus one render of today's palm as the bar.

**Size:** S
**Files:** `.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-PALM.md`
**Touches:** [.flow/evidence/fn-190-one-growth-law-species-are-points-in-a/MODEL-PALM.md, .flow/evidence/fn-190-one-growth-law-species-are-points-in-a/raw/palm-today/**]

### Approach
- Corner's model rules from LITERATURE.md (catalogue table) and sources, each quoted: one unbranched orthotropic stem, lateral inflorescences, a multi-stemmed palm is reiteration.
- Map to settings: branching readiness at zero; name what the stem needs from the engine (height, taper, lean) and which palm organs depend on the stem (`pipeline/foliage/plan/fronds.rs`, `pipeline/branching/leaf_bases.rs`, `pipeline/branching/lattice.rs`; preset `crates/telperion-core/presets/date-palm.values`).
- Render today's palm at seeds 1 and 7 with `growth_law today date-palm <seed> <out> <prefix>`; it is the bar.
- Traits from `catalogue/date-palm/packet/references.json` (P-WHOLE, P-TRUNK, P-BASE).

### Investigation targets
**Required:**
- `catalogue/date-palm/packet/references.json`
- `crates/telperion-core/presets/date-palm.values`
- `crates/telperion-render/examples/growth_law/main.rs` (the `today` command)

## Acceptance
- [ ] `MODEL-PALM.md` lists Corner's rules with quotes, settings and the stem features the palm's organs rely on
- [ ] Today's palm stills at seeds 1 and 7 are rendered, viewed and kept as the bar
- [ ] Three to five reference-visible traits are listed

## Done summary
MODEL-PALM.md written; today palm stills at seeds 1 and 7 kept as the bar (raw/palm-today, cropped crown: framing moved to .19). Host questions Q1-Q4: Corner vs Tomlinson model, stem girth without secondary thickening, inflorescence scope, crop.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 609c45fd83256bcae3d53fa89849c19597c0e13f
- Tests: cargo build --profile ci -p telperion-render --example growth_law
- PRs: