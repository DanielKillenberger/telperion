# fn-150-the-slim-field-package-grows-every.1 Slim field grows every preset through the core pipeline

## Description
The spec's R1 to R5 as one task: the slim crate asks the core pipeline for the field, and the tests, npm test and 0.1.4 follow.

## Blocker (NEEDS_HUMAN, 2026-09-25)
The fix works but grows the slim Wasm by 48%, which meets the brief's "stop" condition. `crates/telperion-field` now calls `pipeline::build` with a field-only `Request`, which needs core's `geometry` feature. All five slim tests pass, including R1 (the date palm at seeds 1, 7, 1407 and 4242, red on master with "family without a leaf plan"). `telperion-field.wasm` went from 355,100 to 526,965 bytes (+171,865, +48%): gzip 126,663 to 186,705, brotli 104,737 to 151,287. For comparison, the full binding is 1,330,943 bytes (gzip 417,882). By twiggy, grouped by module: the placed-leaf fallback (surface rings about 22 KB, foliage station/placement/rosette/clumping/short_shoots/packed/cull about 45 KB, Field::new 7 KB, pipeline::outputs 10 KB) is most of it. Another part is skeleton parity that the slim copy skipped (leaf_bases 8 KB, clear_apical_twigs 1.6 KB, tree::identity 23 KB shared). The rest is about 5.6 KB of `surface::build::faces` that is linked but never runs, and 14.8 KB of name section. The host decides: accept the size and let R2, R3, R5 and 0.1.4 go ahead, or pick another design (for example, a leaf plan that describes a rosette).

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
