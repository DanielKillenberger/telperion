# fn-209 friction

## 2026-10-05, the light lattice is private to its crate (worker, R0)

- **Doing:** measuring, for the proposal, how much bud light changes when each small subtree's leaves are gathered at its centroid (a stand-in's light coupling).
- **Slowed:** `light::Field` is `pub(crate)`, and an example sees only the public API, so the probe rewrote the lattice (deposit, sweep, read; about 150 lines) and checked the rewrite against `bud_light` (most apart 0.0). The alternative, a temporary test module inside `src/`, meant editing the crate for evidence.
- **Cost:** about 10 minutes and one extra build.
- **Would remove it:** a `light::field(light, leaves, reads) -> impl Fn(Vec3) -> f64` in the public API beside `bud_light`, or an evidence feature that exposes it to examples.

## 2026-10-05, one test binary of the crate's suite runs 404 s (worker, step 1)

- **Doing:** running `cargo test --profile ci -p telperion-space` once after the closed-form refactor.
- **Slowed:** every other binary of the crate finishes in under 9 s; one (11 tests, 1 ignored) took 404 s, most of the suite's wall time. The quick red/green loop for the envelope ran on `--lib envelope` instead.
- **Cost:** about 7 minutes of waiting, once.
- **Would remove it:** naming the slow binary's tests and moving the long walks behind the gate's own run, or splitting them, so a crate-level check stays under a minute.
