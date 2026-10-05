# fn-209 friction

## 2026-10-05, the light lattice is private to its crate (worker, R0)

- **Doing:** measuring, for the proposal, how much bud light changes when each small subtree's leaves are gathered at its centroid (a stand-in's light coupling).
- **Slowed:** `light::Field` is `pub(crate)`, and an example sees only the public API, so the probe rewrote the lattice (deposit, sweep, read; about 150 lines) and checked the rewrite against `bud_light` (most apart 0.0). The alternative, a temporary test module inside `src/`, meant editing the crate for evidence.
- **Cost:** about 10 minutes and one extra build.
- **Would remove it:** a `light::field(light, leaves, reads) -> impl Fn(Vec3) -> f64` in the public API beside `bud_light`, or an evidence feature that exposes it to examples.
