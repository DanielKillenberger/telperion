# fn-259 R1: the scorecard's protocol

Written 2026-10-19 by task 4, before any row it governs was measured. Task 4 applies it to the Oregon white oak (`SCORECARD.md`); task 5 applies it to every other shipped species. A change to this file after a row was measured is an amendment, dated, naming the rows it touches.

## Artifact identities

| Side | Source | Identity |
|---|---|---|
| F4 | branch `fn-259-the-release-meets-the-bar-on-every` at `d59dac4e7` | native probes built from it with `--profile ci`: `frames` (task 3) and `field_time` (`f4-field-probe.patch`, an example never committed) |
| 0.1.4 | tag `v0.1.4` (tag object `c48a4f28a`, commit `c79bb590e`) | a detached scratch worktree outside the repository's worktree set, its own `CARGO_TARGET_DIR`, with `v014-probe.patch` applied (two examples, nothing else changed); built with `--profile ci` |

Both trees pin Rust 1.98.1 and wgpu 30.0.1, and both define `profile.ci` identically (release, `lto = false`, `codegen-units = 16`).

Probe binaries, sha256 (copied out of their target directories so a rebuild cannot swap them):

| Binary | sha256 |
|---|---|
| F4 `frames` | `5cbc010710abd96854ec8ab4316a140b81444b80616635b23afff36e20d17e6b` |
| F4 `field_time` | `ac6ad81e1923a3cdc2fa9f265a88893d4afd69ddd167a240670e597a424ad17e` |
| 0.1.4 `frames` | `74597b0db16253637bca51f368705163c405f80466a92b5c0473c920953e0169` |
| 0.1.4 `field_time` | `e8d74c0d7ab4f7ed9cad29f921f6a66bcaba37efa543be51e455371c759f9aa2` |

Shipped artifacts, sha256. F4's are `npm run build` at `d59dac4e7`. 0.1.4's are the published `telperion-0.1.4.tgz` (npm integrity `sha512-i1aj7U65lg6Qrk4WLi+8/ZpjJ3VRoKLHCTyIlLd+xpptKQyZXB31Lv3+OcIoLwYWH63/0zCLcrbhFxgpU+DDdA==`, file sha256 `d817d3a4a3fa5a07cc89f8e07ddda9973e7fe6246ddb707d6474f4be7d9619af`); its `telperion-field.wasm` is the module TEST5.md measured (`8b894e75…0baba3` there).

| Artifact | F4 | 0.1.4 |
|---|---|---|
| `telperion.wasm` | `092486af5731d8f80cea2ae3ee8bdb2f901e15fe6512773cbfef0519e28b25d5` | `200455a137b716e00069d86baf81ce81f7d00ae91ae3591e3d0cb485e6520cd9` |
| `telperion-render.wasm` | `fd72f1ba1810bf366499b54721124a8ca037c782d0434d91a7f91e26fb08e1d7` | `f1ed12809880d1a61d9d4ac6b1f4e12e498ef0a1a778e02c861c433ce6d40aac` |
| `telperion-field.wasm` | not shipped | `8b894e75592594c70607d0935747f35e16b2150481d4e20b72b06dc61d0baba3` |
| `telperion.js` | `871f52f1b9fbcb0a18777d0212102999c7ea1e34a71e2e6d34e50e3ad4a8650a` | `17c8f7e891271903e95854b3d8ef126e2b73131420f746ad33fcdb913f91e8a8` |
| `wasm-source.js` | `b5d64312dbb544701231dca67e32ba250b7bc6f032448d127476a4c61e760a57` | `18b949759fb9224cf0e3c3746e057e1efb9e893c3b1147a0cddeb22381a96341` |
| `field.js` | `4bbaa48ede510d129b43663a343e756ebf50035143e3f01266560b70230c5625` | `d70376d67d61de15fee6fa9b788d8eee674214a93d84657484942ac58aa4c00a` |
| `voxelize.js` | `acac5a12887df4a87f4fc47ef77431c4e5f29da9b75d860bdb50a343e3204e14` | `2f5fb1703902e0737a306caae697ed35fd23461e119b007c0a1ffa06430dc7a3` |

## Hardware and machine

The owner's desk: NVIDIA GeForce RTX 3080 (10,240 MiB, driver 610.57.04, VBIOS 94.02.42.40.33), AMD Ryzen 9 5950X (16 cores, 32 threads), 31 GB RAM, Linux 7.2.5-3-omarchy, Vulkan through wgpu. Native only: no browser and no WebGPU (spec, Boundaries).

## What is measured, per preset

- **Presets:** every preset both versions ship is a scorecard row. 0.1.4 ships `ordinary`, `oregon-white-oak`, `norway-spruce`, `european-beech`, `silver-birch`, `date-palm`, `telperion` and `laurelin` (`Preset::from_id` at `v0.1.4`); F4 ships the same eight. Task 4 measures the oak.
- **Seeds:** 1, 7 and 42, each reported on its own.
- **Ages.** The product default is each version's own: F4 reads the preset's `/age` (oak 40). 0.1.4's direct build reads no age: `Family::age` is the growth path's alone (`crates/telperion-core/src/family.rs:14` at `v0.1.4`), so its tree is the mature specimen whatever the row says. The matched-age diagnostic draws F4 at the age 0.1.4's row carries (`Family::default`, 100, for the oak: no preset overrides it). This is fn-258's precedent, which drew F4 at master's row ages. Diagnostic rows are F4-only (0.1.4 draws one tree at any age) and are reported apart, never as the scorecard's verdict.
- **Route:** the harness's GPU build, `setTreeGpu` on both versions: `Generator::prepare` (0.1.4 with `Delivery::Resident`), `submit_prepared`, the material, a draw. 0.1.4's synchronous `setTree` (CPU mesh) is not measured.
- **Viewport and poses:** 1920×1080, the renderer's own multisampling (`MULTISAMPLE`, four samples on this card), `STILL_FORMAT`. Whole is `hero_pose` over the tree's bounds; mid and close put the camera at a half and a quarter of the hero distance, the same target.

### Columns

| Column | Probe key | How |
|---|---|---|
| Build, fresh | `fresh_ms` | From `Renderer::new` (the adapter and device already requested) through the generator (F4 also `prepare_programs`), the first build and its first hero frame, to the device's fence. Shader compilation included: every row runs with `__GL_SHADER_DISK_CACHE=0`, so the driver compiles each pipeline rather than reading its disk cache. |
| Build, warm | `warm_ms` | A second build of the same tree in the same process, `prepare` and its submission, to a fenced hero frame. |
| Frame, whole/mid/close | `views[].frame_ms` | 20 frames to settle, then 120 pipelined frames to a fence, wall clock per frame. The timestamp protocol's per-pass medians are kept in the raw row and are not a column. |
| CPU peak | `peak_rss_mb` of the one-build row | The process's `VmHWM` after one build and four hero frames (`ONE_BUILD=1`), as a page that shows the tree holds it. The multi-build row's peak is reported beside it, not counted. |
| Device memory | `device_memory.allocated_mib − wgpu_internal_mib` of the one-build row | The bytes each version's own code allocates for the tree and its targets. The allocator's reserved blocks and wgpu's internal buffers are reported beside it, not counted. Reason: they are the native wgpu-hal allocator's policy and wgpu's indirect-draw validation, both identical on the two sides (one wgpu, 30.0.1). A page in a browser pays Dawn's allocator, not these, while the buffers the version allocates are the same on every backend. Task 3 measured the wgpu-internal part growing with the probe's own timing loop (60 MiB a buffer after 120 pipelined frames, against 3 MiB after four), not with the tree (`TOOLING.md`). |
| Field call | `samples[].warm_ms` of `field_time` | Per seed, the second of two calls in one process, timed from the request to the return of every answer on the host. F4: `Generator::grow_field(family, 36)` on a generator with no renderer, growth, preparation and the 36-cell read-out included. 0.1.4: the slim package's request (`pipeline::build` asked for the field alone), `voxelize.ts`'s `grid(bounds, 36)` and one `Field::query` per cell, natively on the CPU. The first call is reported apart. F4's device and program start (`start_ms`, fn-257's R3-0, once a page) is reported apart and is in neither side's call. |
| Download bytes | n/a | Each shipped `.wasm` and `.js` of the package, raw, `gzip -9` and `brotli -q 11`, and the packed tarball. F4: `npm run build` then `npm pack --dry-run`; 0.1.4: the published tarball. Declarations (`.d.ts`), README and LICENSE are listed in the raw output, not compared. |

The field call is the engine's per-tree call, measured natively on both versions with no renderer, as fn-257 R3' measured F4. 0.1.4's shipped call ran its Wasm in a worker; the native read-out is the same request in the same pipeline, compiled for the host. A browser's numbers are not measured: no agent runs WebGPU.

### Samples and aggregate

- Timing rows (`fresh`, `warm`, frame times): five processes per seed and version.
- Memory rows (`ONE_BUILD=1`): three processes per seed and version.
- Field rows: five processes per version, each calling every seed.
- Matched-age diagnostics: one timing, one memory and one field process per seed.
- F4 and 0.1.4 jobs alternate, the order flipped every pair, so drift falls on both.
- Aggregate: the median of the valid samples and their range [min, max], per seed. The count of valid samples is stated beside each.

### Isolation, validity and load (R4)

Every job runs as one row of `scripts/gpu-row.mjs` inside `telperion-heavy … --threads 4` with `RAYON_NUM_THREADS=4` on both versions:

- **Exclusion:** the row holds `${XDG_RUNTIME_DIR}/telperion-heavy.lock` from before its probe starts to after it ends, and starts only when no other `telperion-heavy-*` unit runs and no process naming telperion is on the card.
- **Detection:** `nvidia-smi --query-compute-apps` every 250 ms; a process that appears during the row and is not in the probe's tree makes it `valid: false`. So do a non-zero exit and a probe that prints no JSON.
- **Residual:** under the host's rule (a), 2026-10-19, the processes on the card at the start (the desktop's clients) are recorded with their memory and do not invalidate the row, so a non-Telperion compute job already running at the start is recorded, not caught. A GPU user that lives under 250 ms is not caught either.
- **Load:** each row records the card's free memory, used memory and utilisation at its start and end (`gpu_start`, `gpu_end`) and the one-minute load average when the probe read its memory (`probe.load`). Free device memory at the start is a column of the row.
- **Invalid rows:** an invalid row is run once more. A second invalid run is recorded invalid. A failed, refused or invalid run is a row marked so: it is never a sample, never a pass, and its job is named in `SCORECARD.md`.
- **Stops:** a probe is killed at 10 minutes (`timeout 600`), and the run stops with a FRICTION.md entry and NEEDS_HUMAN. So does a card that stays shared for 30 minutes.

### Raw results and progress

Rows are appended to `raw/t4-<preset>-rows.jsonl` (ignored), one gpu-row line each, with the probe's command line naming the binary, so each row names its version, seed and mode. `raw/t4-<preset>-progress.txt` records each finished job, its row line and its validity; a stopped run resumes from it. The runner, as run for the oak:

```bash
#!/usr/bin/env bash
# fn-259 task 4: the oak row, F4 beside 0.1.4 (PROTOCOL.md). Resumable: a
# job named in the progress file is skipped. Each job is one probe process
# through scripts/gpu-row.mjs inside telperion-heavy; an invalid row is run
# once more, then recorded invalid; a row killed at 10 minutes stops the run.
set -uo pipefail
cd /home/daniel/Projects/telperion/.worktrees/f4-259
RAW=.flow/evidence/fn-259-the-release-meets-the-bar-on-every/raw
ROWS=$RAW/t4-oak-rows.jsonl
PROG=$RAW/t4-oak-progress.txt
B=/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/t4/bin
OAK=oregon-white-oak
touch "$ROWS" "$PROG"

job() {  # <id> <mem GB> <vram GB> <env and command...>
  local id=$1 mem=$2 vram=$3; shift 3
  grep -q "^$id " "$PROG" && return 0
  local try before line valid rc
  for try in 1 2; do
    before=$(wc -l < "$ROWS")
    telperion-heavy "$mem" --gpu "$vram" --threads 4 -- \
      node scripts/gpu-row.mjs "$ROWS" -- \
      timeout 600 env __GL_SHADER_DISK_CACHE=0 RAYON_NUM_THREADS=4 "$@"
    rc=$?
    line=$((before + 1))
    if [ "$(wc -l < "$ROWS")" -ne "$line" ]; then
      echo "$id norow rc $rc" >> "$PROG"; echo "run-oak: $id wrote no row" >&2; exit 3
    fi
    if [ "$rc" -eq 124 ]; then
      echo "$id timeout row $line" >> "$PROG"; echo "run-oak: $id over 10 minutes" >&2; exit 3
    fi
    valid=$(sed -n "${line}p" "$ROWS" | node -e 'process.stdout.write(String(JSON.parse(require("fs").readFileSync(0)).valid))')
    if [ "$valid" = true ]; then
      echo "$id valid row $line try $try" >> "$PROG"; return 0
    fi
  done
  echo "$id invalid row $line" >> "$PROG"
}

# Two versions' jobs, in an order that flips every call.
flip=0
pair() {  # <f4 job...> -- <0.1.4 job...>
  local a=() b=() side=a arg
  for arg in "$@"; do
    if [ "$arg" = "--" ]; then side=b; continue; fi
    if [ $side = a ]; then a+=("$arg"); else b+=("$arg"); fi
  done
  if [ $((flip % 2)) -eq 0 ]; then job "${a[@]}"; job "${b[@]}"; else job "${b[@]}"; job "${a[@]}"; fi
  flip=$((flip + 1))
}

VIEWS="full whole mid close"
for s in 1 2 3 4 5; do
  for seed in 1 7 42; do
    pair "t-f4-$seed-$s" 4 3 SEED=$seed $B/f4-frames $OAK $VIEWS \
      -- "t-v014-$seed-$s" 4 3 SEED=$seed $B/v014-frames $OAK $VIEWS
  done
done
for s in 1 2 3; do
  for seed in 1 7 42; do
    pair "m-f4-$seed-$s" 4 3 ONE_BUILD=1 SEED=$seed $B/f4-frames $OAK \
      -- "m-v014-$seed-$s" 4 3 ONE_BUILD=1 SEED=$seed $B/v014-frames $OAK
  done
done
for s in 1 2 3 4 5; do
  pair "f-f4-$s" 4 3 $B/f4-field $OAK 1,7,42 -- "f-v014-$s" 4 0 $B/v014-field $OAK 1,7,42
done
# Matched-age diagnostics: F4 at the age 0.1.4's oak row carries (100).
for seed in 1 7 42; do
  job "dt-f4-100-$seed" 12 5 AGE=100 SEED=$seed $B/f4-frames $OAK $VIEWS
  job "dm-f4-100-$seed" 12 5 ONE_BUILD=1 AGE=100 SEED=$seed $B/f4-frames $OAK
done
job "df-f4-100" 12 5 AGE=100 $B/f4-field $OAK 1,7,42
echo "run-oak: done" >&2
```

Before the first row, one protocol check (`raw/t4-checks.jsonl`, not a sample): F4's oak at seed 1 once with the driver's disk cache on and once with `__GL_SHADER_DISK_CACHE=0`, to show the variable reaches the driver (a cache-off `fresh_ms` above the cache-on one).

## Verdict per column

- A column is **no worse** when F4's median is at or below 0.1.4's at every seed. It is **worse** at a seed when F4's median is above 0.1.4's by any amount. Sampling noise is shown by the ranges and is not a tolerance.
- **Owner tolerances** (spec, "Owner decisions"), the only ones counted:
  - fn-237 R0a's build budget: every shipped tree ready in under 0.5 s warm in the harness on this card. Telperion and Laurelin about 1.25 s, and Telperion's 1.33 s at seed 42 accepted, 2026-10-16. A warm build worse than 0.1.4 but under its budget at every seed is covered. The budget names warm builds only, so a worse fresh build is not covered.
  - The spruce's per-tree field at about 0.35 s against 0.1.4's 0.24 s, "acceptable", 2026-10-16. It covers the spruce's field call only.
  - The artifact ceilings raised to the sizes measured after fn-257, 2026-10-16 (`scripts/artifact-budgets.json`). An artifact larger than 0.1.4's but under its ceiling is covered. One without a ceiling is not.
- Every other worse row, by any amount, is a **regression** and goes to R3's ledger (task 7) for the owner with its measured size.
- Matched-age rows carry no verdict.

## Amendments

- **2026-10-19 05:16 (task 4), before any matched-age row:** the matched-age jobs' memory cap is 12 GB, not 14. The desk never had 14 GB of room (FRICTION.md); fn-258's oak at 100 peaked at 9.1 GB. The cap only kills a job over it; no measured row changes.
