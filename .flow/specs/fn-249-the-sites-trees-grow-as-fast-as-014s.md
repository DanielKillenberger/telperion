# The site's trees grow as fast as 0.1.4's

## Goal & Context

The owner (2026-10-09): "i really don't want to ship a release with a performance regression this big". killenberger.com's grove grows each tree as a coarse field in a Wasm worker; the release branch takes oak 1.1-1.5 s, birch 0.87-1.0 s, spruce 0.37-0.39 s per tree against 0.1.4's 0.28-0.32, 0.35 and 0.24 s. The stage profile (`.flow/evidence/fn-245-a-breakthrough-in-generation-time/PROFILE.md`) puts the whole regression in `grow`; Astra ranked the fixes (`ASTRA-SITE.md`). This spec is fn-245's release slice: the four byte-identical changes Astra judged release-sized, measured, then the remaining gap reported for the owner's release decision. Host (2026-10-09): release blocker.

## Requirements

- **R1 (closed-form rows cached):** the immutable closed-form `Rows` kept across grows in one Wasm session, keyed by the complete species parameters, age and row mode, with bounded retention; byte-identical output; cold and warm timings reported.
- **R2 (exact word-wise division):** `I256::quotient` (`fixed/feedback/wide.rs:102`) replaced by an exact limb division (normalisation, quotient correction) with signed truncation, zero and narrowing behaviour unchanged; differential tests against the old routine over boundary and random cases; contract A's CPU/device parity green.
- **R3 (placeholder framing):** first an isolated measurement of what the settle spends on fine-root placeholders (`geometry/place.rs:110`, `fine/lay.rs:60,76`); then the duplicate placeholder framing removed where consumers permit, keeping the unbent layout's root moments and every observable field; byte-identical.
- **R4 (lattice deposits):** the per-corner allocations in `fixed/feedback/lattice.rs:132` replaced by fixed arrays and reused buffers, only if a Wasm measurement confirms the gain; identical deposits.
- **R5 (measured):** the site's request end to end in Wasm (one thread, n = 24, 30, 36, 48, oak/birch/spruce, seeds 1234 and 5678, cold and warm) against 0.1.4, best of 3, load stated; every output byte-identical to before (field answers, `space_hash`); the remaining gap per species reported.
- **R6:** gate and npm test green.

## Boundaries

Byte-identical speed work only; no change to the oak's light model or growth. Release blocker for fn-237; the rest of fn-245 stays after the release.
