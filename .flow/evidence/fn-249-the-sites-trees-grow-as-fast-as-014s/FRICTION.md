# fn-249 friction

## 2026-10-09, R1: `cargo fmt -p telperion-space` rewrites 21 files

- Doing: formatting the shelf (R1) before its test.
- Hindered: the crate is not rustfmt-clean at 4b0f7b01, so `cargo fmt -p telperion-space` reformatted 21 files the task does not touch; they had to be reverted by hand.
- Cost: about 2 minutes.
- Would remove it: one formatting commit on F4 (or a CI `cargo fmt --check`), so a package-scoped `cargo fmt` changes only what was edited.

## 2026-10-09, R1: the site makes a fresh Wasm instance for every tree

- Doing: reading how the site's worker reaches `grow`, to know what "one Wasm session" spans.
- Hindered: `growField` (`src/field/index.ts:82`) calls `WebAssembly.instantiate` per tree, so any state kept inside the instance, R1's rows among it, is dropped after one tree. A warm hit needs a caller that grows several trees on one instance; neither `growField` nor the site's worker does today.
- Cost: no time lost; it decides what R1 can deliver at the site. Raised to the host as a design question (instance reuse in `growField` changes its contract: the slim engine holds one tree at a time).
