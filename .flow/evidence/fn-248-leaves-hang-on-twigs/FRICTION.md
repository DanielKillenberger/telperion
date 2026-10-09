# fn-248 friction

## 2026-10-09, task 1 (R1 measurement)

- **No public leaf-to-wood record, and no wood age on `Tree`.** I needed to know which wood each leaf sits on and how old that wood is. Nothing public answers either question. Placement keeps no node index, and `plan::runs` / `bearing_of` are crate-private. The conversion drops each phytomer's `cycle`. Working around it took a code survey and an example that rebuilds both: it matches each plan descriptor to its node by bit-equal position, and it regrows the engine structure to look up each node's cycle by key. Cost: about 15 minutes and roughly 140k subagent tokens for the survey. What would remove it: a measurement accessor that returns each planned station's node, plus each node's birth cycle carried as record data, the way `grown` is.
- **`cargo clippy -p telperion-core` fails on the base.** The lib has 26 clippy errors that predate this branch, so a lint check of the new example cannot run cleanly. The example builds without warnings. Cost: 2 minutes. What would remove it: a clean lib clippy, or CI running it.
- **The dcg hook blocks `> $VAR/file` redirects.** A command that wrote a log through a shell variable path was refused, and I had to retype it with a literal path. Cost: 1 minute. This is a local setup issue, reported here and not specced.

## 2026-10-09, task 1 (part 1, the device)

- **The cold-compile test fails when the render suite runs in parallel.** `every_pipeline_within_2_5_s_but_grow_within_20_s_cold` measured the `grow` pipeline at 23.1 s against its 20 s bound during `cargo test -p telperion-render --lib` with 4 test threads. Run alone it passed, in 23.03 s for the whole first tree, of which 17.9 s was `grow`. This change touches no part of the `grow` pipeline, so the failure is contention between tests. Cost: one extra GPU run, about 3 minutes. What would remove it: run this test in its own process, or serially, so it times the compile and not its neighbours.
- **Two WGSL builtin names were shadowed.** A local `floor` in `dress.wgsl` and a local `length` in `stations.wgsl` shadowed the builtins of the same name. Naga reports this only when the shader is built on the GPU ("local declaration cannot be called"), so each one cost a full GPU run to find. Cost: two GPU runs, about 4 minutes. What would remove it: a CPU-side naga validation of every shader source in an ordinary test, which would catch these without a device.
- **The cold-compile test failed again inside the gate.** `grow` took 26.8 s in the full workspace run; run alone right after, it passed at 18.3 s. A gate that is red only because tests compete for the GPU cannot prove anything about this change. Cost: 3 minutes and an unknown status recorded. What would remove it: the same fix as above, a process of its own for that test.
- **The dcg hook blocked shell idioms.** It refused redirects to `$VAR` paths and `perl -pe` filters fed by a pipe, so the digest-stripping step had to be rewritten as a Python script. Cost: about 5 minutes. This is a local setup issue, reported here and not specced.

## 2026-10-09, task 1 (density)

- **One density trial ran for 15 minutes and was killed without a number.** The trial was three buds a node on Laurelin's fine ages. The CPU build grows the tree past the browser's phytomer ceiling, because a trial that runs outside the browser has no expected-count refusal. Cost: 15 minutes. What would remove it: give `twig_layer` the browser's `KEPT` ceiling so an over-ceiling trial is refused in seconds, the way the harness refuses it.
