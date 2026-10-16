# The device draws the palm's leaf-base lattice

## Goal

fn-256 task 3 moves every resident build onto the device's one cycle (host decision B, 2026-10-12), so the device must draw the date palm. Its rosette fits the stations stage; its trunk's leaf-base boots (fn-110, fn-144), added on the CPU as wood in expansion's first step with a lattice cell each, did not. Without them the palm draws a bare trunk, a visible loss against F4. One generator gap per spec (AGENTS.md); fn-256 depends on it. Release item.

## Decisions

- Design first, in fn-256's DESIGN.md as 12.8: a device stage between the dress and the folds adds two drawn records and a cell per base; folds leave bases out of the envelope and take them into the leaf box; the sweep reshapes a packed base's run onto its cell with squared frames; the handoff passes cells as the curve's sections.
- `executor::bases` replaces `hangs_bases`; the device's refusal by that capability is removed.
- Found by runs (DESIGN.md 12.8.1): the device's storage-buffer limit is 8 (cells ride in the sweep's records binding); flags take bits 30 and 29 above the leaf count; the oldest base is merged as `paths.rs` merges it.

## Acceptance Criteria

- **R1:** design in DESIGN.md 12.8, by degree, within the stage limits — met.
- **R2:** the palm builds on the device and draws in the harness; boots parity test by lineage key and ordinal with missing-boot and perturbed-cell controls red; refusal removed — met (`boots_tests`).
- **R3:** palm stills whole/mid/close beside F4's, viewed, lattice visible; build time; gate and npm test green; codex review — met.

## Result

Parity at ages 12 and 40: 256 of 256 bases and shaped runs matched; points within 9.0e-5 m and directions within 2.2e-4 (bounds 2e-4 and 5e-4); both negative controls refused. Stills (viewed): the trunk carries F4's diamond lattice; pixels differing whole 2.5 %, mid 12.8 %, close 14.0 % (the leaflets' own draws). Warm build 98 ms with bases, 96-99 ms without. Codex SHIP, no findings.

## Open

None.

## Friction

- flowctl not on the worker's PATH -> dispatch names the path.
- Device storage-buffer limit is 8, not COUNCIL.md's 16 -> state the limit in COUNCIL.md/DESIGN.md or check bindings statically.
- Drawn flags word had no documented free bits (BOOT collided with the leaf count) -> flag layout written beside `CV` in `fold.rs`.
- jev replay needs the renderer's `headless` example when the gate is split around the GPU lock -> fn-237 R10.
