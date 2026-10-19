# fn-259 R1: the scorecard, F4 beside 0.1.4

Measured 2026-10-19, 04:55-05:45, under `PROTOCOL.md`. It was committed in `dd09b47b9` before the first row, amended in `196062203` before the matched-age rows, and amended in `e27b07876` after review, before the field rows were rerun. The setup is the identities, hardware and route stated there. Raw rows: `raw/t4-oak-rows.jsonl`. Progress: `raw/t4-oak-progress.txt`. Byte sizes: `raw/t4-oak-bytes.txt`. Protocol check: `raw/t4-checks.jsonl`. All raw files are ignored. Task 5 adds the other presets below the oak. The European beech has no shipped 0.1.4 baseline (PROTOCOL.md, Presets), which is a host call before task 5.

Each cell is the median of the valid samples, [min, max] (n), F4 against 0.1.4. **Bold** marks F4 worse than 0.1.4.

## Oregon white oak at the product default (F4 age 40; 0.1.4 the mature direct build)

| Column | Seed | F4 | 0.1.4 | Verdict |
|---|--:|--:|--:|---|
| Build, fresh ms | 1 | **4,501** [4,354, 4,628] (5) | 2,599 [2,568, 2,641] (5) | regression, +1,902 |
| | 7 | **4,451** [4,375, 4,559] (5) | 2,617 [2,586, 2,910] (5) | regression, +1,834 |
| | 42 | **4,574** [4,409, 4,636] (5) | 2,624 [2,520, 2,739] (5) | regression, +1,950 |
| Build, warm ms | 1 | **243** [242, 247] (5) | 129 [128, 142] (5) | covered: under the 0.5 s budget |
| | 7 | **259** [259, 269] (5) | 158 [148, 191] (5) | covered |
| | 42 | **243** [241, 245] (5) | 164 [151, 170] (5) | covered |
| Frame ms, whole | 1 / 7 / 42 | 2.39 / 3.30 / 2.79 | 16.4 / 15.6 / 16.3 | no worse |
| Frame ms, mid | 1 / 7 / 42 | 5.68 / 8.03 / 6.66 | 29.0 / 33.7 / 33.0 | no worse |
| Frame ms, close | 1 / 7 / 42 | 8.01 / 10.4 / 10.1 | 25.3 / 28.2 / 32.0 | no worse |
| CPU peak MB (one build) | 1 | **783** [746, 794] (3) | 299 [298, 300] (3) | regression, +484 |
| | 7 | **775** [760, 798] (3) | 313 [313, 314] (3) | regression, +462 |
| | 42 | **773** [750, 802] (3) | 305 [305, 307] (3) | regression, +468 |
| Device MiB, allocated less wgpu-internal (one build) | 1 / 7 / 42 | 137 / 163 / 141 (3 each, no spread) | 319 / 356 / 347 | no worse |
| Field call, warm ms | 1 | **233** [229, 246] (5) | 151 [148, 164] (5) | regression, +82 |
| | 7 | **253** [251, 265] (5) | 189 [187, 214] (5) | regression, +64 |
| | 42 | **230** [229, 236] (5) | 173 [167, 180] (5) | regression, +57 |

Frame-time ranges are within 4 % of each median on both sides (`raw/`). Reported beside the counted columns, not counted:

- **Device reserved:** 512 MiB on both sides at every seed (one build), and 768 on both after the multi-build run.
- **wgpu-internal:** 6.5 MiB on both after one build.
- **Multi-build peak RSS:** F4 871-881 MB, 0.1.4 362-374.
- **Multi-build allocated less internal:** F4 146-182 MiB, 0.1.4 319-356.
- **The field's first call in a process:** F4 231-258 ms, 0.1.4 165-190 (medians).
- **F4's field start:** the device, the generator and `prepare_programs`, once a page, 2,285 [2,202, 2,307] ms (5).
- **Superseded field rows:** the first field rows (`f-` jobs) are not counted (PROTOCOL.md amendment 2). 0.1.4's probe had been built with the core's `geometry` feature, and F4's start had left out `prepare_programs`. Their warm medians were within 6 ms of the rerun's at every seed.
- **0.1.4's route:** every 0.1.4 build reported backend `Gpu` for foliage and wood, so nothing fell back to the CPU.

The two oaks are different trees. 0.1.4's is the mature specimen, and its 36-cell field marks 10,592-11,333 wood cells and 11,707-12,393 foliage cells. F4's oak at 40 marks 3,814-4,540 and 3,836-4,613.

## Download bytes (each shipped artifact; raw / gzip -9 / brotli -q 11)

| Artifact | F4 | 0.1.4 | Ceiling (raw) | Verdict |
|---|--:|--:|--:|---|
| `telperion.wasm` | **1,463,135 / 480,779 / 370,337** | 1,330,623 / 423,026 / 326,004 | 1,600,000 | raw covered; gzip +57,753 and brotli +44,333 regressions |
| `telperion-render.wasm` | **3,399,922 / 1,076,219 / 803,265** | 1,900,399 / 603,490 / 449,472 | 3,700,000 | raw covered; gzip +472,729 and brotli +353,793 regressions |
| `telperion-field.wasm` | not shipped | 344,798 / 131,050 / 107,663 | none (F4) | no worse |
| `telperion.js` | **193,862** / 17,821 / 11,240 | 118,387 / 22,453 / 18,335 | 200,000 | raw covered by the ceiling; gzip and brotli smaller |
| `wasm-source.js` | **64,437 / 15,556 / 13,195** | 235 / 200 / 155 | 70,000 | raw covered; gzip +15,356 and brotli +13,040 regressions |
| `field.js` | 1,917 / 963 / 881 | 2,720 / 1,247 / 1,096 | 16,000 | no worse |
| `voxelize.js` | 4,362 / 1,837 / 1,635 | 4,526 / 1,881 / 1,670 | 16,000 | no worse |
| all of the above | **5,127,635 / 1,593,175 / 1,200,553** | 3,701,688 / 1,183,347 / 904,395 | none | regression |
| packed tarball | **1,645,249** | 1,222,313 | none | regression, +422,936 |

## Matched-age diagnostic: F4's oak at 100

The diagnostic was **refused at every seed**. Every timing, memory and field job (seeds 1, 7 and 42, each run twice) exited 1 with `Oversize { buffer: "cycle state axes", bytes: 2558845952, limit: 2147483644 }`. That is 14 rows, all marked invalid, so no matched-age row exists. The field job's two rows ran the first F4 field probe, before amendment 2. The refusal comes from growth, which that amendment does not touch, so the job was not rerun (inferred). fn-258 drew F4's oak at 100 on 2026-10-10 (`RESULT.md`), so this refusal is newer than that branch (inferred; not traced). It is a diagnostic and carries no verdict, but it is a fact the owner's ledger should hold.

## Validity and load

- **Rows:** 83 in all. 58 are counted: 48 timing and memory rows and 10 rerun field rows. 10 are superseded field rows. 15 are invalid.
  - **Default-age jobs:** every row valid but one, `t-v014-42-3`. That row was invalid because a GNO embedding worker (`bun … gno … native-worker`, 1,143 MiB) appeared on the card during it. Its rerun was valid. So every default-age job has its full sample count.
  - **Matched-age jobs:** the 14 refused rows above.
- **Free device memory at the start of a row:** 7,287-8,478 MiB, median 8,462. The low end is rows with the GNO worker resident from the start, which rule (a) records and does not invalidate.
- **GPU utilisation at the start:** 0-17 %.
- **One-minute load over the counted rows:** 0.37-3.83, median 1.37.

## Protocol check (not samples)

F4's oak at seed 1, run four ways (`raw/t4-checks.jsonl`):

| Run | fresh ms |
|---|--:|
| The driver's default cache | 4,609 |
| `__GL_SHADER_DISK_CACHE=0` | 4,290 |
| An empty cache directory | 4,397 |
| That same directory, now holding the cache | 362 |

So the variable does take the cache out: cache-off is a cold compile. The protocol's stated test (cache-off above cache-on) could not show it, because the default cache missed: its 1.1 GB directory evidently returns nothing for these pipelines (inferred). Shader compilation is about 4 s of F4's 4.5 s fresh start. 0.1.4's fresh rows ran under the same variable.

## Open, for the owner (task 7's ledger)

- **Regressions at the default age, every seed:**
  - fresh build +1.83-1.95 s;
  - CPU peak +462-484 MB;
  - field call +57-82 ms.
- **Download regressions:**
  - the gzip and brotli growth of `telperion.wasm`, `telperion-render.wasm` and `wasm-source.js`;
  - the package total: +1,425,947 raw, +409,828 gzip, +296,158 brotli;
  - the tarball: +422,936.

  The raw ceilings cover raw bytes only (PROTOCOL.md amendment 4).

  The field-call regression is against 0.1.4 measured natively. fn-257 compared F4 with 0.1.4's Wasm figures from TEST5.md (273-332 ms at other seeds), which this protocol replaces.
- **Covered, by the owner's tolerances:**
  - the warm build (+79-114 ms, under the 0.5 s budget);
  - every artifact's raw size (under its ceiling).
- **Question for the owner:** do the raw ceilings also cover an artifact's compressed growth? As written, they do not.
- **Matched age:** F4 refuses the oak at 100 (above).
