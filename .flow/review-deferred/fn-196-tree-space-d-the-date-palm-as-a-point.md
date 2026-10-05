# Deferred review findings — fn-196-tree-space-d-the-date-palm-as-a-point

## 2026-10-05 15:10 — review session fn-206-tree-space-one-reference-axis-every.1 (01a10c91-7092-7fa1-b72f-f0868473e1e3)

- [P1, confidence 75, introduced] crates/telperion-space/src/grow/stop.rs:113 — A vanishing end-of-age relay applies a full insertion angle to subsequent growth. Reproduce with one state, one certain 
  - Suggested: Make a relay’s geometric departure vanish with its growth share, so a near-zero-length relay preserves the frame inherited by subsequent growth. Add this whole-lifespan boundary case to the continuity tests.
  - Deferred reason: host decision 17 (2026-10-05): a continuity defect only at an exact boundary (lifespan 1 -/+ 1e-9); changes no species or strip; deferred to continuity hardening beside fn-199

- [P1, confidence 75, introduced] crates/telperion-space/src/grow.rs:223 — The same vanishing relay increments `ended_relays`, changing the next full unit’s survival key. Use the preceding specie
  - Suggested: Preserve subsequent decision keys when a relay’s growth share vanishes, while retaining independent decisions for distinct stops. Add the seed-244 case on both sides of the boundary.
  - Deferred reason: host decision 17 (2026-10-05): a continuity defect only at an exact boundary (lifespan 1 -/+ 1e-9); changes no species or strip; deferred to continuity hardening beside fn-199
