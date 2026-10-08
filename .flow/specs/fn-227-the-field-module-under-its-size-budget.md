# The field module under its size budget

## Goal & Context

After fn-211 and fn-212 merged into F4, `telperion-field.wasm`, the binary killenberger.com loads through `telperion/field`, is 1,171,132 bytes against its 632,580 ceiling (0.1.4 shipped 344,798). Measured 2026-10-08 with symbols kept: it holds only about 4.5 KB of the old scaffold grower; the size is the new engine, including the full-tree fine-root path that `growField`'s runtime `cut` option keeps reachable (realized, canonical, landing, ordered: about 76 KB), the stepped grower's per-cycle machinery (152 KB), std and formatting (151 KB unnamed). `opt-level = "z"` alone measured 953,711 bytes (-19 %). Owner (2026-10-08): "we should bring the field module's size down before release ideally"; on `cut: 0`: "there could be something useful built on cut:0 no?"; and "shouldn't we just be able to import modules as needed and be efficient when importing?" A wasm binary loads whole, so importing as needed means one binary per entry point, each linked with only what its exports reach; a capability chosen by a runtime flag cannot be dropped by the linker.

## Requirements

- **R1 (one export per capability):** the coarse field and the full-tree field are separate exports; `telperion/field`'s binary exports the coarse field (the default the site's unchanged `growField` call gets, fn-214 task 3) and reaches no full-tree fine-root code; the full-tree field stays available as its own export in an entry that already carries the full engine (the main package), or its own subpath, decided in the plan with sizes measured.
- **R2 (build):** the field entry built for size: opt-level, panic strategy and formatting footprint chosen by measurement (size and the site's call timing), and a wasm optimiser pass if it pays and can run in CI.
- **R3 (measured):** every built artifact's size against its ceiling; the field binary at or under 632,580 bytes, or the gap reported with its breakdown for the owner; the site's call pattern (FIELD-PATH.md) end to end against 0.1.4, Wasm one thread, low load.
- **R4 (held):** CI fails a build whose artifact exceeds its ceiling (the existing budget check covering the field entry); gate and npm test green.

## Boundaries

Packaging and build only; no engine behaviour changes. Runs after fn-226 (light, which may delete the per-cycle machinery) and fn-214 (the coarse field) land, since both move this code. Depends on fn-198 (F4).