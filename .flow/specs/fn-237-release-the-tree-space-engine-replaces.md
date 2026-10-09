# Release: the tree-space engine replaces 0.1.4, everywhere

## Goal & Context

The owner's goal (2026-10-08/09): "replace the old generator with the new one that's higher fidelity", "make it all the way. Ship it all together": the engine, every species (oak, spruce, beech, birch, palm, ordinary, Laurelin, Telperion), the harness and killenberger.com's move off telperion 0.1.4, in one release PR from the F4 integration branch (fn-198) to master, with nothing merged to master without the owner. This spec holds no code: it is the release map. Its dependencies are every spec the release needs; `flowctl` and `/flow-next:deps` on it show what blocks what. fn-198 task 9 (release checks, the PR) runs when every dependency here is done.

## Requirements

- **R1:** every dependency of this spec done and merged into F4.
- **R2:** fn-198 task 9's release checks pass (harness timing against master, the site's field against 0.1.4, sizes, docs, npm version) and the release PR is opened per docs/pr-format.md.
- **R3:** the owner's verdict on each species and on the site's grove recorded.

- **R4 (found on the way):** fn-234's measurement found the resident device path's tree reaching 0.227 m below ground against the CPU's 0.040 m (the cherry beech at 40); checked in the release checks, and a spec minted if it is a defect.
- **R5 (found on the way):** `src/field/field.test.ts` "grows every shipped preset, byte for byte the main entry's field" runs about 60 s against its 60 s timeout; it timed out under load twice (fn-217's merge at load 58, F4 after fn-234 at load ~30) and passes alone. Before the release PR, made to finish well inside its timeout (or its timeout set from a measured CI run), so CI does not flake on it.
- **R6 (found on the way):** fn-235's cold-build test holds the first tree to 5 s besides `grow` with about 0.3 s of margin after the fn-231 merge (MERGE-235.md), so it may fail under load; before the release PR, the margin restored (MERGE-235 names one route: the lateral place threshold's logarithm from the CPU table instead of in `asks`) or the bound restated from a measured CI run.
- **R7 (friction, obvious fix):** `cargo fmt --all --check` reports 79 unformatted spots that predate the work, so every agent's `cargo fmt -p` rewrites files it never touched (fn-235, fn-241, fn-213 FRICTION.md). Once the last open branch merges, F4 is formatted once in a commit of its own, with no other change, so no open branch inherits the conflicts.
- **R8 (found on the way):** the render module is 3,295,462 bytes against its 2,800,000-byte ceiling (`scripts/artifact-budgets.json`, fn-243's report); measured on F4 before fn-243 to see what moved it, then brought under or the ceiling restated with the owner.
- **R9 (found on the way):** fn-235's cold-build test times `grow`'s compile against its 20 s exception, and failed at 20.6-26.0 s under load from other sessions (fn-246 FRICTION.md) while passing alone; a compile time measures the machine's load as much as the shader. Before the release PR, the test runs as an on-demand probe (like the device-limit probes, `docs/gpu-tests.md`) in the release checks on a quiet machine, and the default suite keeps a load-independent guard (the shader's size or call-site count) instead.

## Boundaries

Map only. Post-release: fn-232, fn-233.
