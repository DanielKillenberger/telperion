# The field module under its size budget

## Goal & Context

After fn-211 and fn-212 merged into F4, `telperion-field.wasm`, the binary killenberger.com loads through `telperion/field`, is 1,171,132 bytes against its 632,580 ceiling (0.1.4 shipped 344,798). Measured 2026-10-08 with symbols kept: it holds only about 4.5 KB of the old scaffold grower; the size is the new engine, including the full-tree fine-root path that `growField`'s runtime `cut` option keeps reachable (realized, canonical, landing, ordered: about 76 KB), the stepped grower's per-cycle machinery (152 KB), std and formatting (151 KB unnamed). `opt-level = "z"` alone measured 953,711 bytes (-19 %). Owner (2026-10-08): "we should bring the field module's size down before release ideally"; on `cut: 0`: "there could be something useful built on cut:0 no?"; and "shouldn't we just be able to import modules as needed and be efficient when importing?" A wasm binary loads whole, so importing as needed means one binary per entry point, each linked with only what its exports reach; a capability chosen by a runtime flag cannot be dropped by the linker.

## Requirements

- **R1 (one export per capability):** the coarse field and the full-tree field are separate exports. `telperion/field`'s binary exports the coarse field only and reaches no full-tree fine-root code: `growField` there takes a cut in the supported coarse range (1 and above; its default is the one fn-214 task 3 sets), and `cut: 0` is refused by name, naming where the full tree lives, never quietly coarsened. The full-tree field moves to the main package entry (`telperion`), which already carries the full engine, as its own export; a test shows it answers byte for byte what `growField(..., { cut: 0 })` answered before the move, for every shipped preset (bounds included). The change and the migration (`cut: 0` callers move to the main entry) are stated in the package's changelog and docs (host, 2026-10-09).
- **R2 (build):** the field entry built for size: opt-level, panic strategy and formatting footprint chosen by measurement (size and the site's call timing), and a wasm optimiser pass if it pays and can run in CI.
- **R3 (measured):** every built artifact's size against its ceiling; the field binary at or under 632,580 bytes, the ceiling unchanged; if it cannot be reached, the work stops with the gap's breakdown for the host and the spec is not complete; the site's call pattern (FIELD-PATH.md) end to end against 0.1.4, Wasm one thread, low load.
- **R4 (held):** CI fails a build whose artifact exceeds its ceiling (the existing budget check covering the field entry); gate and npm test green.

## Boundaries

Packaging and build only; no engine behaviour changes. Runs after fn-226 (light, which may delete the per-cycle machinery) and fn-214 (the coarse field) land, since both move this code. Depends on fn-198 (F4).
## Closed (host, 2026-10-13)

Superseded by fn-257 (the site grows its trees on the device; owner, 2026-10-11): the site no longer grows a coarse field on the CPU, so the structure-above-the-cut field and its module size are replaced by fn-257 R1-R3 and fn-237 R8.
