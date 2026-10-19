# fn-259 R2: replaced code in the shipped artifacts, and their sizes

Task 6 audited the package `npm run build` makes at `54fa05fcd` (2026-10-19) for what fn-245's stages replaced. This is qualification only. A replaced thing that still ships is reported here with its size, and the host mints the fix as its own spec. Nothing in the product was changed.

**In brief.** Contract A, the stepped light executor and the fine-root executor are absent from both Wasm modules, from the embedded shaders and from the JS. So are the growers the stages replaced and the field module. Two things the decisions keep ship and are reachable from the entries: the light lattice and the cut's predictor. Three small leftovers ship:

- a never-constructed `Error::Executor` whose message names "the fine roots' executor";
- a stale "chain grower" comment inside a shipped shader;
- the leaf-base lattice, which is not the lattice the decision names.

The software f64 and the held arithmetic also ship. They are not contract A as fn-254 defines it, and whether R2 counts them is left to the host (Open). Every artifact is under its ceiling.

## What was audited

- **The package.** `npm pack --dry-run` lists 19 files. Six of them are the shipped modules and scripts: `dist/telperion.wasm`, `dist/telperion-render.wasm`, `dist/telperion.js`, `dist/field.js`, `dist/voxelize.js` and `dist/wasm-source.js`. The rest are `.d.ts` files, the README, the LICENSE and `package.json`. There is no `telperion-field.wasm` (see F).
- **The build.** Built under `telperion-heavy 8 --threads 4 -- npm run build`, exit 0, with rustc 1.98.1, wasm-bindgen 0.2.128 and node 26.8.1. The sha256 prefixes:

  | File | sha256 prefix |
  |---|---|
  | `telperion.wasm` | `092486af5731d8f8` |
  | `telperion-render.wasm` | `fd72f1ba1810bf36` |
  | `telperion.js` | `871f52f1b9fbcb0a` |
  | `field.js` | `4bbaa48ede510d12` |
  | `voxelize.js` | `acac5a12887df4a8` |
  | `wasm-source.js` | `b5d64312dbb54470` |

- **Rust content.** Both modules carry a name section. `twiggy top --format csv` gave 1,922 function names for the main module and 3,622 for the renderer's, demangled with their crate hashes stripped. Each term below was matched against them. `twiggy paths` traced each name it found to an export. That path is static reachability: a call path from an export, not proof the path runs.
- **Shaders.** Each of the 100 `.wgsl` files under `crates/telperion-render/src` was matched as UTF-8 bytes against the renderer module's bytes. 97 are embedded whole. The remaining 3 are test-only and absent: `cycle/probe.wgsl`, `positions_admit.wgsl` and `wood_geometry.wgsl`, so fn-293 audit finding 10 is closed.
- **Deleted shaders.** Git history holds 44 deleted WGSL files under `crates/telperion-render/src` and `crates/telperion-space/src`. Each was read at its last version, and its `@compute` entry points and `fn` signatures were searched in the renderer module. Git records 14 more files as renamed out of `fine/` in `29218095e`. They are live today, so the scan above covers them (see C).
- **JS.** Each of the six scripts and each `.d.ts` was searched for every name below.

**What this method can and cannot show.** An inlined function leaves no name. So a missing name counts as absent only where its source is deleted or `#[cfg(test)]`, or the symbol has no production caller. Each row says which.

## The audit

Results are **absent**, **found and reachable** (a static path from a named export), **found and unreachable**, or **unknown**.

### A. Contract A (fn-254: exact Q32/Q62 fixed point with emulated 64-bit products)

| Symbols | Where searched | Result |
|---|---|---|
| `telperion_space::fixed` (`fixed.rs`, `fixed/{decide,feedback,feedback/{angle,edge,lattice,power}}.rs`) and the Q32, Q62 and W (i128) formats, CORDIC and `isqrt` (fn-211 `CONTRACT-A.md`) | The source, both name lists, and the live WGSL for `q32`, `q62`, `cordic`, `isqrt` and `mul_wide` | **Absent.** The source was deleted in `9d129f439` (fn-254), and no name matches. The 128-bit builtins in both modules (`__multi3`, `__udivti3`, `u128_div_rem`; 1,271 B main, 1,207 B renderer) come from libm's `sqrt` and `pow`, and `fmod` (`pipeline::space::pa_rows`, `clothe`), not from fixed point. |
| The device mirror `fine/contract.wgsl` (with `fine/wide.wgsl`) and the `fine/feedback_*.wgsl` arithmetic | The renderer's bytes (entry points and signatures) and the live WGSL | **Absent.** fn-254 rewrote `fine/contract.wgsl` in f32 (`9d129f439`, 476 lines out). `29218095e` then moved it to `cycle/decide.wgsl`, and `wide.wgsl` to `cycle/u64.wgsl`. No live WGSL carries a fixed-point marker. The feedback shaders were deleted in `9d129f439`, and none of their entry points is found. Today's `cycle::contract()` (`cycle.rs:168-175`) is fn-254's f32 contract: `u64`, `keys`, `decide` and `organs`. |

Exact arithmetic that ships and is not contract A under fn-254 R2 (see Open, item 1). All of it is found and reachable:

- **The software binary64.** `cycle/soft64.wgsl` (9,473 B of text) emulates IEEE binary64 on u32 pairs. The cycle's glue widens and narrows reals with it (`glue.rs:33`), and the settle uses its word product (`settle_device.rs:183-186`). Its rationale is fn-256 `DESIGN.md` §8: "every f64 operation of the cycle that feeds a decision is reproduced on the device bit for bit". The light sweep it first served now runs in f32 (`light_sweep.wgsl:2`, DESIGN §19).
- **The held arithmetic.** `cycle/held.wgsl` (2,275 B) holds float-to-integer sites to the CPU's order (fn-256 DESIGN §8, "added 2026-10-12, after the review").
- **The wide identity words.** `u64.wgsl` (3,146 B) is identity only, which fn-254 R3 keeps exact.
- **The pinned transcendentals.** `telperion_space::transcendental` (`sin_fixed`, `exp_fixed`, `powf_fixed` and the rest; 10 names, 917 B main, 899 B renderer) is fn-11 and fn-198 host decision 10. Astra (fn-245 `ASTRA-ARCH.md` §3) asked that it be questioned.

### B. The stepped light executor

| Symbols | Where searched | Result |
|---|---|---|
| `grow::stepped` (`stepped.rs`, `stepped/{feedback,handover,phases,standing,stands,buds,schedule,settle}.rs`), `fine::coarse::grow_coarse`, `Coarse`, `Stand`, `fixed::feedback` | The source; both name lists for `stepped`, `handover`, `grow_coarse`, `Coarse` and `feedback`; strings in both modules | **Absent.** Deleted in `187a89777` (2026-10-12), `9d129f439` and `8b19fbb96` (2026-10-18). No name matches. The renderer's three `feedback` names are `telperion_render::curve::feedback`, the curve's level readback. Its one `stepped` string is a WGSL comment ("never stepped onto"). |
| The device feedback shaders `fine/feedback_{stepped,slice,lay,probe,sites,turn,wide}.wgsl` | The renderer's bytes | **Absent.** None of their 8 entry points is found. |

### C. The fine-root cut

| Symbols | Where searched | Result |
|---|---|---|
| The CPU cut: `fine::{coarse,exchange,expansion,lay,realized,step,cases,geometry,landing,likely,spare,survey}`, `alone`, `Grower`'s `fine`, `oracle` and `couple` fields | The source and both name lists | **Absent.** Deleted in `187a89777` and `8b19fbb96`. `grow.rs` has no such field. |
| The device fine-root executor: `generation::fine` (`fine.rs` and 29 `fine/*.wgsl`, among them `chain.wgsl`, `assign`, `emit`, `settle` and `record`) | The renderer's names and bytes | **Absent as an executor.** Deleted in `29218095e` (fn-256). No `generation::fine` name. Four of its entry names occur in today's shaders, but in different code: `dress` is `cycle/dress.wgsl`, which the device cycle took over (fn-293 `PIPELINE-AUDIT.md`, the sanctioned table); `gather` is `cycle/wake.wgsl`'s, with another body; `count` and `emit` are `stems`', `stations`' and `sweep`'s. The same commit renamed 14 files out of `fine/`, and all of them are live. Into `cycle/`: `arith`, `contract` (as `decide`), `lineage` (as `keys`), `probe`, `wide` (as `u64`) and `zones`. Into `resident/`: `blooms`, `fold`, `handoff`, `rings`, `scan`, `sort`, `stations` and `sweep`. They are the device cycle's and the drawing stages' shared code (the sanctioned table). `resident/handoff.wgsl` was deleted later (`534a2fb64`). |
| What the cut left behind: `telperion_space::fine::{canonical::{lineage_order, identities, lineage_permutation}, tables::Units, surface::ChainTables}` | Both name lists and `twiggy paths` | **Found and reachable**, as `fine.rs:1-4` describes: the canonical lineage order and the cycle's unit tables. `lineage_order` is reached from export `build` (main) and `webrenderer_setTree` (renderer). 16 names, 19,117 B in main and 21 names, 27,440 B in the renderer, most of it sort monomorphisations. |
| `telperion_space::Error::Executor` (`error.rs:25-27`, message at `:58`) | Every crate's source, and the strings in both modules | **Found and unreachable.** The string "the fine roots' executor failed: " is in both modules. No crate constructs the variant, so only the `Display` arm links it in. About 33 B a module. |
| The `zones.wgsl:2-3` comment "shared by the unit kernel and the chain grower" | The renderer's bytes | **Found** as shipped shader text: the chain grower it names is gone. `units.rs:3` ("every fine-root shader binds") is a Rust comment and does not ship. |

### D. The lattice: present by decision

The decision is fn-237's "Astra's release review, incorporated" (host, 2026-10-16): "Not touched, as the review advises: the cycle, the lattice and the CPU reference". `ASTRA-RELEASE-REVIEW.md:91` gives the reason ("Growth is the product. COUNCIL.md rejected the hierarchy-light experiment"). That points at the light lattice (inferred).

| Symbols | Where searched | Result |
|---|---|---|
| **The light lattice.** `telperion_space::light` (`light.rs`, `light/field.rs`): `Field::sweep`, `Field::quantised_with`, `Field::at`, `Grid::corners`, `Light::directions`, `light::swept_all` | Both name lists and `twiggy paths` | **Present by the decision; found and reachable.** It is reached from export `build` (main) and `webrenderer_setTree` (renderer). It is Beer–Lambert light through a 0.5 m lattice of leaf area, deposited and swept per sky direction (`light.rs:1-8`, `light/field.rs:1-12`). Function bodies: 7,890 + 2,496 B in main, 7,536 + 2,387 B in the renderer. |
| **The device lattice.** `cycle/light.wgsl` and `cycle/light_sweep.wgsl` (with `build_light.rs`) | The renderer's bytes | **Present by the decision; found.** Embedded as 9,129 + 6,292 B of text, in f32 (DESIGN §19). |
| **The leaf-base lattice.** `pipeline/branching/lattice.rs` (`lattice::cell`): the spiral cell of a retained leaf base | Both name lists and the source | **Found and reachable through inlining.** It has no name of its own. Its one call, `leaf_bases.rs:179`, is in `leaf_bases` (line 93). The path is `clothe_leaf_bases` → `leaf_bases` → `lattice::cell`, inlined into `pipeline::clothe` (5,801 B main and 5,547 B renderer). `pipeline::clothe` is reached from export `build` (main) and `webrenderer_setTree` (renderer). The bytes that belong to the lattice alone are unknown. The task names this file as "the lattice that stays", but it is not the light lattice the decision names (see Open, item 2). |

### E. The growers and predictors the stages replaced

| Item (fn-293 `PIPELINE-AUDIT.md` finding) | Symbols | Result and evidence |
|---|---|---|
| The experimental passes (5) | `grow_oracle`, `grow_coupled`, `couple`, `grow_partitioned`, `subtree`, `realized`, `grow_bounded`, `bounded`, `run_beside` | **Absent.** Deleted in `8b19fbb96`. No name. The `subtree` and `bounded` hits are `BTreeMap::clone_subtree` and `AttachmentSurface::bounded`. |
| The kept recursion (11) | `closed_form::kept`, `closed_form::boundary` | **Absent.** Both are `#[cfg(test)]` (`closed_form.rs:16-21`), and no name matches. |
| The cut's predictor (11) | `closed_form::predictor::{tree, tilts, Walk::*}`, `expected_kept` | **Found and reachable, kept by decision:** "Not touched ... the predictors that still size and blend" (fn-237 line 74), with consumers named in `ASTRA-RELEASE-REVIEW.md:92`. Reached from `build` (main) and `webrenderer_admission` (renderer). 14 names, 29,384 B main and 28,406 B renderer, plus `expected_kept` at 502 B and 461 B. |
| The scaffold grower (4) | `pipeline::colonization`, `branching::specimen`'s grower, `branching::scaffold`, `twigs`' and `radius`' solves | **Absent.** The grower compiles only into tests (`pipeline.rs:257-261`), and no `colonization`, `specimen`, `scaffold` or `radius::` function name matches. What ships is the scaffold's wire rows: their getters and setters (fn-293 `CONTRACTS.md` K10, kept), `validate_skeleton` (`specimen::validate`, which fn-293 task 8 kept), `TwigParams::resolved` (reached from `Inputs::of`) and `leaf_bases`. fn-288 retires the rest after the release. |
| The species constructors (9) | `telperion_space::{oak,birch,beech,spruce,palm}`, `tolerance` | **Absent** from both name lists (dead-stripped). The source is compiled, and only tests and examples call it. |
| The unquantised light field (16) | `light::Field::new` | **Absent.** No name; only `quantised_with` ships. |
| The CPU delivery (6) | `Delivery::Cpu`, `prepare_whole`, `prepared.rs`, `for_cpu_output` | **Absent.** `generation::preparation` is `#[cfg(test)]` (`generation.rs:18-19`); no name. |
| `branch_diagnostics` (8) | `branch_diagnostics`, `generationCounts`, `levelCappedHandoffs` | **Absent.** No name, and none of the strings in either module or the JS. |
| The node-count blend (15) | `telperion_core::blend` | **Absent.** No name. The renderer's `blend` hits are wgpu's `set_blend`. |

### F. The field module and the CPU field's export

| Symbols | Where searched | Result |
|---|---|---|
| `telperion-field.wasm`, the slim field module (fn-293 `CONTRACTS.md` K8: "`git ls-files` names neither") | The `npm pack` list, `dist/`, `src/field/`, and the strings in both modules and the JS | **Absent** from all of them. The untracked `src/field/telperion-field.wasm` that fn-293's audit saw is gone too. |
| `field.snapshot()`, `FieldSnapshot`, `IndexSnapshot` (`CONTRACTS.md` K4) | The source, both modules' strings, the JS and the `.d.ts` | **Absent.** |
| The resident field primitives (fn-257 R8): `field_query.wgsl` and `field_build.wgsl` | The renderer's bytes | **The query is absent.** Its one entry point is not found. Eight of `field_build`'s 13 entry names (`clear`, `widest`, `carrier`, `opens`, `order`, `own`, `inherit`, `count`) occur in `resident/field.wgsl`, which is R8's grid read-out of the drawn records (`resident/field.rs:1-10`): the replacement, not the thing replaced. |

## Artifact sizes

These are measured on the same `dist/`, separately from the audit. Gzip is level 9 and brotli is quality 11. The ceilings are `scripts/artifact-budgets.json`'s. `node scripts/artifact-budgets.mjs` exits 0: every ceiling holds, and the growth share is checked only on a pull request.

| Artifact | Raw (B) | Gzip (B) | Brotli (B) | Ceiling (B) | Headroom | Baseline (B) |
|---|---:|---:|---:|---:|---:|---:|
| `dist/telperion.wasm` | 1,463,135 | 483,966 | 370,337 | 1,600,000 | 136,865 (8.6%) | 1,410,330 |
| `dist/telperion-render.wasm` | 3,399,922 | 1,085,340 | 803,265 | 3,700,000 | 300,078 (8.1%) | 1,985,291 |
| `dist/telperion.js` | 193,862 | 17,903 | 11,240 | 200,000 | 6,138 (3.1%) | 119,550 |
| `dist/wasm-source.js` | 64,437 | 15,650 | 13,195 | 70,000 | 5,563 (7.9%) | 64,564 |
| `dist/field.js` | 1,917 | 950 | 881 | 16,000 | 14,083 (88.0%) | 2,720 |
| `dist/voxelize.js` | 4,362 | 1,818 | 1,635 | 16,000 | 11,638 (72.7%) | 4,526 |
| **Total** | 5,127,635 | 1,605,627 | 1,200,553 | | | |

- **Baselines.** Each baseline is the file's recorded measurement: master `ed23f773` for every artifact except `wasm-source.js`, which was measured on fn-257 `bdf5c51b3`.
- **The package.** The `npm pack` tarball is 1,645,249 B, 5,252,247 B unpacked, with 19 entries.
- **The name sections** are part of the raw sizes above: 230,676 B of `telperion.wasm` (15.8%) and 518,548 B of `telperion-render.wasm` (15.3%). This is not replaced code. Whether they ship is a size question for fn-242 or the owner, and nothing was changed here.

## Open, for the host

1. **Does R2's "contract A" cover the exact arithmetic that ships today?** That is the software binary64 (`soft64.wgsl`), the held arithmetic (`held.wgsl`) and the pinned transcendentals. fn-254 R2 defines contract A as Q32/Q62 with wide products, which is absent. The three above are fn-256 DESIGN §8 and fn-198 decisions. This audit reports them as found and reachable, and makes no call.
2. **Which lattice does the decision mean?** This audit reads fn-237's "the lattice" as the light lattice (D, inferred from `ASTRA-RELEASE-REVIEW.md:91`). The task names the leaf-base lattice (`branching/lattice.rs`). Both ship.
3. **Two small leftovers of the fine-root cut ship:** the never-constructed `Error::Executor` with its message, and the stale `zones.wgsl` comment. If they should go, that is its own spec.

## Reproducing it

Run from the repository root after `npm run build`, with twiggy 0.8.0:

- `twiggy top -n 100000 --format csv dist/<module>.wasm` lists the names. Strip the `[16 hex]` crate hashes, then grep.
- `twiggy paths dist/<module>.wasm '<exact hashed name>' -d 16` traces a name to an export.
- To check the shaders, compare each `.wgsl` file's UTF-8 bytes, and its `fn` lines, against the module's bytes. For a deleted file, `git show <deleting commit>^:<path>` gives the text.
- `npm pack --dry-run --json` gives the file list.

No script was kept. The checks rest on twiggy and git history, and whether CI should hold any of them is a separate decision.
