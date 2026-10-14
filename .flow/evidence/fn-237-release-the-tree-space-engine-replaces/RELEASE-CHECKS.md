# fn-237 release checks: R9 and R14

Branch `fn-237-release-checks`, from `d2a8bd258`. Measured 2026-10-14 on the RTX 3080 (driver 610.57.04), at a load average of 1.4 to 4.4. Raw lines are in `raw/r9r14-*.txt`, which is ignored.

## R14: every shipped module against Tint

- **Tint runs here without a browser.** Dawn's GitHub releases ship a prebuilt Linux `tint` (`gh release download -R google/dawn -p '*ubuntu-latest-Release.tar.gz'`, release v20261009.190412, `bin/tint`, 15 MB). It needs no device. No crate wraps Tint, and building Dawn was not needed. `tint` emits one entry at a time (`--format spirv --ep <entry>`), as Dawn makes a pipeline on Vulkan.
- **The check.** `shader_tests::every_shader_passes_tint` is a release check (`TINT=<path>`, ignored by default; docs/gpu-tests.md). It runs every entry of all 37 shipped modules through Tint. First it proves that the binary refuses a red fixture for each class and accepts the fixture's sound twin. The classes are a NaN constant from `bitcast`, a barrier under a storage-bound loop, and `|` beside `<<`.
- **The 16-storage-buffer limit** is a device limit Dawn applies when it creates a pipeline, not a Tint rule: Tint accepted a 17-buffer fixture. The housekeeping check (`every_shader_holds_the_storage_buffer_limit`) holds every module to WebGPU's default of 8, which is stricter, and it still passes.
- **Finding: the cycle hierarchy build module.** Tint refused all 16 of its entries for one line: `select(...) | (rank / cfg.cap) << 2u` in `hier_blocks.wgsl:134`, "mixing '|' and '<<' requires parenthesis". Chrome would refuse the whole module (`begin_main`, which the owner's harness hit). Fixed in `6945bf72b` as `| ((rank / cfg.cap) << 2u)`. That is naga's own reading of the line: naga's IR of the module is identical before and after, spans stripped, so output does not change. The probe is red with the old line and green with the fix. No other module is refused.
- **The housekeeping findings in `select` and `positions` were the token checker's own faults.** Tint accepts both modules.
  - `select`: `group_count()` returns `(u.instances + ...)`, and the checker took the field name `instances` as a non-uniform identifier.
  - `positions` `admit`: a `continue` in a lane-bound loop marked the rest of the function, past the loop, as non-uniform.
  - Both are fixed in `tint_rules.rs`, with a fixture pair for each, every pair checked against Tint first. The check now runs on every shipped module in the default suite (`tint_rule_tests::every_shipped_shader_keeps_tints_rules`).
- **Codex review** (one reviewer, correctness): SHIP. It found two places where those fixes let a real violation through, both confirmed against Tint: two different uniform fields returned under a non-uniform branch, and a `return` reached after a divergent `continue`. Both are fixed, with a red fixture each.
- **Not covered by the default suite:** operator-mixing rules like the one above. Tint's grammar check has no token-level twin here; the release probe holds them.

## R9: compile time out of the default suite

- **The probe.** `every_pipeline_within_2_5_s_cold` (the `grow` 20 s exception was already gone with fn-256's small kernels) is now `#[ignore = "release check: ..."]`. `device_limit_probes_run_only_when_asked` now holds release-check marks too. Run once here: the first tree built cold in 2.05 s, with no pipeline over 2.5 s.
- **What tracks the compile cost.** `compile_size::tests::every_entry_made_cold_alone` makes each shipped compute entry alone, with the driver's disk cache off. Three cold runs gave a median per-entry spread of 3 ms (55 ms at most). It records the time beside two naga counts for each entry: its expressions, with every callee counted again at each call site (inlined size), and its expanded call sites. Across 196 entries:

  | Measure | Spearman to cold ms | Pearson, log-log |
  |---|---|---|
  | Inlined expressions | 0.92 | 0.96 |
  | Expanded call sites | 0.88 | 0.93 |
  | Module bytes | 0.60 | 0.51 |

  Nearly all of the time is the driver's compile: naga's SPIR-V emit takes about 1 ms an entry and is deterministic. Module creation is 64 ms in total.
- **The high end, from fn-256's tree** (`acf647a31`, built apart, same method):

  | Entry | Inlined expressions | Cold compile |
  |---|---|---|
  | Units `cycle_main` | 95,761 | 39.8 s |
  | Layout `draw_main` | 109,421 | 1.3 s |
  | Place `frame_main` | 110,875 | 1.7 s |
  | Layout `frame_main` | 117,267 | 2.4 s |

  So the count ranks cost within one shape of code but does not predict it across shapes. The largest shipped entry inlines 43,525 expressions and compiles in 0.76 s. Nothing between 43,525 and 95,761 was measured.
- **The guard.** `compile_size::tests::every_entry_inlines_under_the_ceiling` holds every shipped entry, drawing entries included, to 50,000 inlined expressions, 15 % over today's largest, with a red and a green fixture. An entry past the ceiling is timed by the release probe before the ceiling moves. The 50,000 is the worker's choice inside the gap the data leave open; the host may move it.

## End checks

- **Focused tests**, all green on `159a54064`: `tint_rule_tests` (6), `compile_size` (the guard and the call-site fixture), `shader_tests` (validation, the storage limit, file coverage), and `device_limit_probes_run_only_when_asked`. The release probes passed too: Tint on every entry, and the cold build at 2.05 s.
- **Gate** (`cargo test --profile ci --workspace --no-fail-fast`): not run to a result. At the dispatched 9 GB it was OOM-killed in telperion-core's tests after 9 minutes. At 10 GB it waited 42 minutes for room behind fn-272's film renders and was stopped (FRICTION.md). It is left to the host.
- npm test: not run, since no TypeScript was touched.
