# The birch grows the same wood on the device as on the CPU

## Goal & Context

fn-254's CHECKS.md (2026-10-11): the silver birch's device-grown wood stands up to 0.219 m off the CPU's, the same figure on contract A before fn-254 as on floats after it, while the beech, spruce, Telperion and Laurelin agree within their measured bounds. `preset_agree_tests::BIRCH` holds the birch to its own wider bound, and fn-254's leaf-area test lets the birch's 42,548 leaves apart in place and 5,673 in direction ride that gap. fn-256 puts every axis on the device, so an executor gap left here becomes the shipped birch. Unknown, to be found first: which stage first differs (growth, lay, sag, girth) and on which axes.

**Prerequisite:** fn-254 merged into F4 (this spec depends on it). The references below are on its branch `fn-254-growth-runs-on-ordinary-floats` until then: `.flow/evidence/fn-254-growth-runs-on-ordinary-floats/CHECKS.md` (the measured bounds the other GPU presets meet), the birch's own bound in the render crate's preset agreement test, and the birch exemption in `every_gpu_preset_seats_its_leaves_as_the_cpu_path_does` (`fine/resident_tests.rs`).

## Requirements

- **R1 (repro first):** a standalone test that grows the birch at 30, seed 42 on both executors and reports the first stage and the first lineage keys whose output differs, with the size of the difference; red on F4. The cause named and checked against the code before any fix.
- **R2 (fixed):** the cause fixed on the executor that is wrong, both executors then agreeing within the bounds the other GPU presets meet; the birch's own bound in `preset_agree_tests` and its exemption in the leaf-area test removed.
- **R3 (proved):** birch stills whole/mid/close beside the current ones, viewed; gate and npm test green.

## Boundaries

The birch's cross-executor agreement. Release item (fn-237): the shipped birch is the device's.
