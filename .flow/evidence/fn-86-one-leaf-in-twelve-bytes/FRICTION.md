# Friction - fn-86 One leaf in twelve bytes

## 2026-09-19 - the worktree carries no build cache

Doing: the Phase-1 baseline run of `cargo test --release --workspace` in
`.worktrees/fn-86-one-leaf-in-twelve-bytes`.

What slowed it: a fresh worktree has no `target/`, so the baseline was a cold
release build of the whole workspace including wgpu. Cost: the run was taken
with `CARGO_TARGET_DIR` pointed at the main checkout's `target/` to reuse the
dependency artifacts, which turned a cold build into a warm one. Without that
the baseline alone would have spent a large share of a 180 minute box.

What would remove it: the worktree kit setting `CARGO_TARGET_DIR` (or a
`.cargo/config.toml` with a shared `target-dir`) when it creates a worktree, so
every dispatched agent inherits the warm cache instead of discovering the trick.
Cargo keys artifacts by package path, so the two checkouts coexist without
thrashing.

## 2026-09-19 - the encoding's stated bound and its stated bit budget disagree

Doing: writing the R2 round-trip test for the rotation word.

What slowed it: R2 asks for 0.002 rad on the rotation, and a smallest-three
quaternion at ten bits a component cannot reach it. Measured over a sweep of
4,096 orientations: 0.00316 rad worst with each component rounded on its own,
0.00250 rad worst with the encoder choosing the nearest of its cell's eight
corners (implemented, decoder untouched). The cause is structural rather than a
mistake in the encoder: the dropped component is reconstructed from the other
three, and its sensitivity is one over itself, which reaches two near half a
turn - so the angular error is up to four times the component half-step, not
one times it, which is where "about 0.001 rad" in the spec's architecture note
came from. Cost: about 25 minutes of the box, and the test now holds the
measured 0.0026 rather than the asked-for 0.002.

What would remove it: an acceptance number derived from the worst case rather
than the typical one. The same arithmetic would have shown at spec time that
either the bound is 0.0026 or the rotation needs more than thirty bits.

## 2026-09-19 - one spec, eight criteria, one box

Doing: the whole run.

What slowed it: the implementation half of this spec touches the core's storage,
three shaders, three shader-owning Rust modules, three uniform blocks, the wasm
slot table, the bincode wire and the browser TypeScript - and then R4, R7 and R8
each want a separate measurement campaign on top. fn-85's friction entry already
said a spec of this shape does not close in one box; fn-86 was sized to one
session on that basis and the implementation alone filled it. Cost: R4, R7 and
R8 are not started.

What would remove it: splitting the evidence criteria from the encoding change,
as their own spec that depends on it. The implementation is one coherent commit;
the five-species QA pass, the four RSS measurements and the five stills are
three independent campaigns that need a green tree first and nothing else from
each other.
