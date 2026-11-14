# fn-212 R1: the contract between settled fine geometry and its readers (task 1, worker, 2026-10-07)

> Archived 2026-10-16: the files this contract cites beside it (`AUDIT.md`, `TASK4.md` to `TASK11.md`, `FRICTION.md`, `share-probe.patch`, `baseline-marks.patch`, `build-fix.patch`) are not kept; see .flow/evidence/ARCHIVE-MANIFEST.md, `git show 8dab10634:.flow/evidence/fn-212-tree-space-fine-geometry-dressed-on-the/<file>`. The host decisions are in the fn-212 spec.


What the device-side readers (rings, stations, curve points) take in and hand on, checked against the code at `1419a832` (fn-198's tip with F1 through task 9 merged). Every claim cites file:line at that commit. A claim not checked is marked **unknown**. Design questions are listed for the host at the end; none is decided here. AUDIT.md, beside this file, holds the deletion audit, the fresh baseline and the proposed R3 probe.

## Answer in brief

- **Identities are not botanical yet.** Stage 2's conversion still takes seven geometry decisions that set node counts and identities (`pipeline/space.rs`, table below). Measured: the beech at 80 loses 4.5 to 6.3 % of its kept phytomers to them, the spruce 2.4 to 3.0 %. FINE-CONTRACT host decision 2 makes them draw-time choices. **That change has to land before the probe's reference is frozen.** A probe matched against today's bytes would preserve the rejected behaviour.
- **The device holds 91 to 96 % of a tree's kept phytomers under the realized executor's final routing, measured.** Beech at 80: 96.1 to 96.2 %. Spruce at 80: 91.0 to 92.0 %. The rest is:
  - the structure above the cut: 1.0 to 2.8 %;
  - the ground-routed roots laid on the CPU: 1.1 to 1.3 % of the beech, 6.9 to 7.9 % of the spruce.
  - The ground-routed roots are also grown and laid on the device, where their copy is stale (see "Ownership").
- **The seam crosses runs, not only axes.**
  - The sweep's runs follow the widest non-codominant child at every node (`surface/paths.rs:44-52`). Axes do not bound them, so a run can pass from a bearer above the cut into a fine root.
  - A branch's socket reads its parent's radius (`surface/samples.rs:114-121`).
  - A seated leaf's contact reads its node's ring pair and the neighbouring pair on the same run (`generation/contact.wgsl:31-34`).
  - How many runs cross the seam is **unknown**: nothing records a node's provenance after conversion.
- **Five whole-tree reductions gate every reader:**
  - the envelope (height, crown base, widest reach, fullness, shoulder);
  - the leaf box (`Reference::grown`);
  - the root radius that classifies twigs;
  - the stem radius that classifies bearing wood;
  - the run ranking.
  - Each is a fold over every node. The shoulder's front, the only part that is not a plain fold, keeps 34 to 61 candidates on the beech at 80 and 3,567 to 3,988 on the spruce (measured). Every one of them fits a bounded summary.
- **Fine-root buffers are root-local, f32 and test-only.**
  - Each root's geometry is in metres from its bearer's node (`generation/fine/laid.rs:4-7`).
  - The laid buffer carries no parent link, start radius or node kind (`laid.rs:19-20`). Topology stays in the packed run buffers.
  - The whole module is `#[cfg(test)]` (`generation.rs:9-10`).
  - Every wait blocks (`io.rs:333-337`, `device.rs:126-131`).
  - The driver spawns threads, and `sow` reads `std::time::Instant` (`fine/expansion.rs:333-380`, `:420`).

## How checked

- **Code:** read at `1419a832` in this worktree. The line numbers are that commit's.
- **Counts:** `share-probe.patch` is a temporary ignored test, removed afterwards. It drives `settle_with` with the CPU reference executor and the landing on (`land = true`), as the public `settle` does (`fine/expansion.rs:107-123`), for the beech and spruce at 40 and 80, seeds 1 and 7. Output is in `raw/share.txt`. Its device counts equal REALIZED.md's GPU table to the phytomer (for example, spruce 80 seed 1: 4,351,256).
- **Node counts after conversion:** `baseline-marks.patch` prints them from `pipeline/space.rs`. The shoulder's front was counted by a temporary print in `input/shoulder.rs`, in the same patch.
- **The branch tip does not compile as merged** (FRICTION.md, first entry). Every measurement ran with `build-fix.patch` applied, a one-line clock fix. It is not committed.

## 1. Identities (FINE-CONTRACT host decision 2)

Decision 2: "identities are botanical (one per kept phytomer); merging or omitting thin wood becomes a draw-time level-of-detail choice". Today the conversion still decides identities from laid geometry.

| Decision | Site | Reads |
|---|---|---|
| Wood thinner than `FINEST` (0.2 mm) is not drawn, and neither is anything it bears | `space.rs:40`, `:254` (`take_while(radius >= FINEST)`) | f64 radius |
| An internode shorter than `SHORTEST` (1 mm) merges into its node | `space.rs:41`, `:261` | laid tips |
| A relay's first internodes behind its node, or inside the joint's wood, merge | `space.rs:245-262` | laid tips, parent radius |
| A relay takes its parent's run from the node nearest its base, and the stopped head turns aside | `space.rs:171-182`, `:126-158` | laid tips, radii |
| A head inside its joint's wood is scaled down to `FORK_FROM` of its carrier | `space.rs:139-149` | laid tips, radii |
| Only a whole axis is carried on | `space.rs:295-297` | the decisions above |
| A node's kind: `STRUCTURAL` (5 %) of the root's radius, or a twig where nothing grows from it | `space.rs:309-315` | radius against a whole-tree value |
| Codominance weight | `space.rs:163-166` | radius ratio |

Measured (`PROBE` lines in `raw/pass*/…err`; nodes include the root node):

| Tree | Kept phytomers | Pipeline nodes | Lost to the decisions |
|---|--:|--:|--:|
| beech 80 seed 1 | 1,596,774 | 1,525,191 | 4.5 % |
| beech 80 seed 7 | 2,704,252 | 2,532,841 | 6.3 % |
| spruce 80 seed 1 | 4,780,267 | 4,637,482 | 3.0 % |
| spruce 80 seed 7 | 4,722,702 | 4,610,322 | 2.4 % |

**Further identity-like decisions downstream** of conversion, which read continuous radii against whole-tree values:
- **Which wood bears leaves** (`foliage/plan.rs:270-281`). It is a node's kind, or its pipe radius (`tree.rs:135-141`) under the stem radius (`tree.rs:151`) times `shoot_radius`.
- **A station's count**, from a bearing run's length (`plan.rs:90-107`).

On two executors whose radii differ by up to 6.4e-6 relative (REALIZED.md), these can flip at a bound. fn-125's tolerance requires equal counts. Whether this flips any node on the four fixtures is **unknown**.

## 2. The partition boundary, from the final routing

- **Driver:** `settle` → `settle_with(…, land = true)` (`fine/expansion.rs:107-123`). `telperion_space::grow`, which stage 2 calls, runs `grow_fine` with `land = false` (`fine.rs:163-169`, `grow.rs:66-77`). The production tree today is therefore the CPU executor's, with no landing.
- **Routing rule:** a root is laid on the CPU when its closed-form reach from the lower of its node and the point below can reach the landing zone (`fine/landing.rs:43-58`). The rule is checked unbent in phase 3 (`:127-156`) and from either frame in phase 4 (`:176-208`).
- **What the device still does with those roots:** it grows and lays them anyway. `lay` and `bend` run over every live root (`generation/fine/device.rs:296-338`, the `Roots` impl from `:217`), and the CPU's copy replaces the device's load (`landing.rs`).

Measured (`raw/share.txt`, cut 1,024; shares of the kept phytomers):

| Tree | Roots (live) | Ground-routed roots | Above the cut | Fine roots | of them on the CPU | **On the device** |
|---|--:|--:|--:|--:|--:|--:|
| beech 40 s1 | 553 (475) | 4 | 2.63 % | 97.37 % | 3.18 % | **94.19 %** |
| beech 40 s7 | 975 (824) | 3 | 2.83 % | 97.17 % | 1.20 % | **95.97 %** |
| beech 80 s1 | 7,865 (4,997) | 16 | 2.72 % | 97.28 % | 1.09 % | **96.19 %** |
| beech 80 s7 | 12,829 (8,161) | 29 | 2.64 % | 97.36 % | 1.29 % | **96.07 %** |
| spruce 40 s1 | 21,307 (19,527) | 1,337 | 1.04 % | 98.96 % | 6.95 % | **92.01 %** |
| spruce 40 s7 | 20,444 (18,821) | 1,021 | 1.02 % | 98.98 % | 5.51 % | **93.47 %** |
| spruce 80 s1 | 92,703 (74,358) | 5,623 | 1.11 % | 98.89 % | 7.86 % | **91.03 %** |
| spruce 80 s7 | 91,262 (73,492) | 4,843 | 1.11 % | 98.89 % | 6.93 % | **91.96 %** |

These replace the spec's "95-99 %".

## 3. The seam

Reading across the seam, in the order the readers take it:

1. **Runs.**
   - `paths` continues every run into the widest non-codominant child by `start_radius` (`paths.rs:44-52`), or the widest codominant child where all children are codominant (`:59-63`).
   - Children at the same point (within 1e-9 m) collapse into their parent (`:36`, `:53-54`).
   - So a fine root's first node continues its bearer's run wherever it is the widest such child at its node, for example at the last node of an axis with no continuation. The run graph does not follow the cut.
   - Run order is a global sort by largest radius (`surface/rings.rs:292-301`; `compact.rs:153-157`), which mixes runs from both sides.
2. **Samples.**
   - A branch run's first sample sinks into its parent along `-away`, with the parent's radius `pr` (`samples.rs:114-121`).
   - The fork easing reads `paths.forks` at the bearer (`samples.rs:43-68`).
   - Distance along the wood is cumulative from the root (`rings.rs:277-288`). A fine root's points therefore need its bearer's distance at its node.
3. **Frames.** Parallel transport along the run (`surface/frames.rs:25-61`). A run that crosses the seam carries the bearer's frame in.
4. **Contacts** (seated leaves only).
   - A station's contact is `[lower ring, upper ring, run's first ring, run's last ring]` (`compact.rs:172-181`).
   - The search reads that pair and the neighbouring pair within the run (`contact.wgsl:31-34`).
   - On a run that crosses the seam, the neighbour can be on the other side.
5. **Stations.**
   - Bearing runs are their own split, joined while a bearing parent has one bearing child on the same branch (`plan.rs:291-295`).
   - They have their own transported frames (`foliage/station.rs:304-356`).

**Unknown:** the number of runs, contacts and bearing runs that cross the seam on each fixture. Method: record each converted node's axis, and so its side of the cut, in a probe of `space.rs`, then count the run edges whose ends differ.

## 4. Whole-tree reductions

Every reader of fine geometry needs these first. Each is a fold over all nodes, so the device must reduce its fine nodes and combine them with the CPU's.

| Value | Site | Fold | Readers |
|---|---|---|---|
| height | `input.rs:60` | max y | twist phase (`compact.rs:197`; the curve's tree row `curve.rs:96`), flare depth (`build.rs:65`, trunk runs only), station spacing (`station.rs:126`), leaf box |
| crown base | `input.rs:62-66` | min y over non-stem nodes | envelope, shoulder |
| widest reach and its height | `input.rs:67-68` | argmax of `hypot(x, z)` | spread, fullness, shoulder |
| shoulder | `input.rs:82`, `input/shoulder.rs:22-85` | 1,024 bins of the most reach, then the nodes no other passes (`:64-74`), then a 40-step bisection | the cull's outline (`place.wgsl:24-41`) |
| leaf box | `foliage/reference.rs:91-106` | min/max of each node ± reach(max radius) | every leaf's quantisation (`pack.wgsl` `station`, box test `:38-40`) |
| root radius | `space.rs:299` | the first node's start radius | node kind (`space.rs:309`) |
| stem radius | `tree.rs:151` | over stems leaving the root | bearing wood (`plan.rs:275`) |
| run ranking | `rings.rs:292-301` | sort of every run's largest radius | ring, curve and run-table order |

- **The shoulder's front is bounded:** 34 (beech 80 s1), 61 (beech 80 s7), 3,567 (spruce 80 s1) and 3,988 (spruce 80 s7) candidates after the bin filter (`shoulder.rs:74`; temporary print). The device can reduce the bins (1,024 floats), filter, and hand on at most a few thousand (p, q) pairs. **The bisection must stay on the CPU:** it runs on the pinned `quadrant` (`envelope.rs`), which f32 cannot reproduce.
- **Decision 11's state at this tip:**
  - the shoulder is measured (`input.rs:82`);
  - irregularity is 0 (`:83`);
  - the lobe scale is the struct default, read by nothing on this path (`:84`; host decision 11a removes both rows at R0).
  - So the remaining work is only the device reduction, which this spec owns.
- **The run ranking is a global sort.** Whether its order is read for anything but the run table's order (`SurfaceRun`, `compact.rs:217-221`; `CurveRun`) is **unknown**. If not, it could become a per-side order and stop being a reduction.

## 5. Ownership and buffers

**Device buffers after the GPU executor's last phase** (`generation/fine/`):

| Buffer | Owner | Layout | Lifetime |
|---|---|---|---|
| laid, one per wave | `Group.laid: Vec<Wave>` (`group.rs:26`), one taken from the pool per wave (`group.rs:82-87`) | per axis 8 f32 (base unbent, base bent, PA, first phytomer; `laid.rs:19`, `walk.wgsl:205`); per phytomer 22 f32 (bent tip, heading and side, unbent tip, heading and side, radius `[18]`, scale `[19]`, rank Q52 `[20..22]`; `laid.rs:20`, `settle.wgsl:129-131`, `:198`) | returned to the pool when `Device` drops (`device.rs:208-215`); the pool keeps one build's buffers (`pool.rs:44-47`) |
| packed axes and phytomers | `Run.axes` (`AXW` = 80 words an axis), `Run.phyts` (`PW` = 11 words) (`roots.rs:113-114`, `geometry.wgsl:36-37`; taken from the pool at `roots.rs:232-233`) | lineage words 0-1, kind 4, region-local parent 5, node 6 (`geometry.wgsl:40-46`) | pooled as above |
| geometry scratch | `Run.scratch` (66 words an axis plus 12 a phytomer; `laid.rs:21-23`) | | pooled |
| records | `Group.records` (`group.rs:29`, made at `:90`, `CR` floats a chain) | each root's base radius and load as phases 2 and 3 hand them up (REALIZED.md; the word layout was not re-read) | a plain buffer, not pooled (`group.rs:90`) |

**What a reader cannot find on the device today:**
- **Parent links and identities** are in the packed buffers in region-local form. Only the test readback assembles topology (`fine/readback.rs`, test-only).
- **Start radius** (`Node.start_radius`, which the leader rule, socket, flare hold and bearing tests read) is not stored. The conversion derives it as `max(inherited, radius)` or the lateral's own (`space.rs:278-282`).
- **Positions are root-local f32**, in metres from the bearer's node (`laid.rs:4-7`). The bearer's laid node is a CPU value in the tree's frame. The conversion also turns z-up into y-up (`space.rs:119-121`).
- **The ground-routed roots' device copy is stale.** The CPU's laying is authoritative (`landing.rs`). A reader must take the CPU's copy for those roots, uploaded or read where they lie. Option open (host question 3).

**Bytes per fine phytomer on the device** (layout arithmetic, not measured): 88 laid + 44 packed + 48 scratch = 180 B, plus per axis 32 + 320 + 264 B. The region buffers are about 1.4 GB (REALIZED.md, task 8).

**Resident artifacts a drawn tree holds after `submit_prepared`** (`generation/submit.rs:16-24`):
- the curve's GPU form (`curve.rs:72-110`): 32 B a point, plus clusters and sections;
- the leaves, 12 B each (`compute.rs:181-187`);
- the occupancy masses (`compute.rs:277-283`);
- the element.

Measured `tree_buffer_bytes` after the submit: 141 MB (beech 80 s1), 240 MB (beech 80 s7), 759 MB and 756 MB (spruce 80 s1, s7).

## 6. Synchronization and the browser

- **`Roots` and `settle` are synchronous** (`fine/expansion.rs:47-76`, `:107`).
- **The device waits block:**
  - every read is `pollster::block_on` (`generation/io.rs:333-337`), with `device.poll(wait)` (`io.rs:185`);
  - other waits are at `device.rs:126-131` (`wait`), `io.rs:279` and `:345`.
- **The driver uses threads:**
  - `std::thread::scope` (`expansion.rs:208`, `:267`, `:310`, `:342`) and `std::thread::spawn` (`:227`, `:420`);
  - the species tables arrive on a `JoinHandle` (`device.rs:27`, joined at `:96`).
- **`sow` reads `std::time::Instant::now()`** (`expansion.rs:333`, `:348`, `:362`, `:380`). `telperion_space::grow` reaches it for the beech and spruce on every target (`grow.rs:75` → `fine.rs:163` → `expansion.rs:144`), wasm32 included. Whether the browser build traps on this tip is **unknown** (not run). fn-198 host decision 8 forbids `Instant` on any path Wasm runs.
- **Per-node readback:** none on the production path. The resident path reads 64 bytes of the placement summary (`compute.rs:163`) and the positions' admission (`positions.rs`).

## For the host

1. **Freeze the reference after decision 2 is built**, or freeze it on today's conversion and state that it moves? Astra's review asks the former. Building it changes every engine tree's node count (2.4 to 6.3 % more nodes at 80, by the table in §1) and its bytes, so the baseline is retaken after it.
2. **Leaf-bearing classification and station counts across executors** (§1, last list): decide them from record or integer data (for example the committed radius), or relax "counts are equal" for these two. Unknown whether any flips today.
3. **The ground-routed roots** (6.9 to 7.9 % of the spruce's phytomers): stop growing them on the device, overwrite the device copy with the CPU's laying, or dress them on the CPU path beside the structure above the cut.
4. **The run graph across the seam** (§3): keep `paths`' leader rule whole-tree (the device needs every bearer's leader decision and distance), or make a fine root always start its own run (the trees change: a bearer's tip no longer flows into its widest lateral).
5. **The run ranking** (§4): a global sort, or each side's own order, if nothing reads it but the table.
6. **The browser's clock and threads in `sow`** before R4 (§6; FRICTION.md).

## Host decisions on CONTRACT.md and AUDIT.md (2026-10-07)

- **Denominator: option B.** The CPU deletions the audit found are taken first (task 2); R3's 3x is measured against the build after them, so it comes from the device port alone.
- **D1 (reference):** the conversion keeps every kept phytomer (fn-209 decision 2 built here, task 3); thin wood is drawn as F3's sub-pixel ribbons instead of dropped by geometry; the probe's reference is frozen after that.
- **D2:** leaf-bearing and station counts come from record and integer data, so counts are equal across executors.
- **D3:** ground-routed roots never reach the device; they are excluded before upload.
  - **Amended from (a) to (c)** (host, 2026-10-07), after task 4 showed that phase 1 has no floor for a bearer's height. Routing stays exact where it is decided today, and landed roots are masked on the device by the `LANDED` flag. Measured cost: about 7 % of the spruce's phytomers masked (TASK4.md).
- **D4:** a fine root always starts its own sweep run, joined at the seam by a socket; the seam is judged on stills.
- **D5:** each side keeps its own run order, the coarse structure's first; no global sort.
- **D6:** on wasm32 the driver's sowing runs on the calling thread and every timer reads the engine's clock (`crate::clock::seconds()`); the same tree as natively.
- **The branch's compile error** (`Stages::add` assigning `Instant::now()` to an f64, expansion.rs:405) is fixed in task 2 with the patch the audit saved.

## Host decisions after task 5 (2026-10-07)

1. **Frame seed.** A fine run's frame is seeded from its bearer's frame at the socket, carried along the tangent by parallel transport, on both executors. A seed that jumps on near-equal components is a defect, not a contract. Outputs change on both sides, which the generator-evolution rule allows. Show what moved: the twist distribution before and after, and stills of the spruce and beech at 80, at the hero view and close up, viewed. Drop the test's exclusion of flipped runs.
   - **Worker, built and measured** (TASK5.md, "After the host's decisions"): the rotation from the bearer's heading turns where a run leaves backwards. At the spruce at 80, 5,457 runs leave within 10° of backwards, and 1,702 to 1,704 runs turn by up to 3.1 rad.
   - **What is committed for the host's choice:** the bearer's laid side, laid across the first tangent. It turns where a run leaves along its bearer's side, and at most 4 runs on the eight trees leave within 10° of that. Frames are within 0.0091 rad with no exclusion.
2. **Codominance tie.** A head scaled to exactly `FORK_FROM` gets no codominance on both executors. It is decided by an exact comparison, not an f64 epsilon. The cross-executor case breaks D1''s determinism assumption, so it is in scope. Built (`16328f13`).
3. **The spruce at 80, seed 7, point 1.0 mm off:** to be explained before task 7's parity verdict. Done: it is a `SHORTEST` flip at the 1 mm threshold (TASK5.md).
4. **GPU test memory.** The whole-tree probes serialise themselves through a process-wide lock in the test support code, so the suite stays green at the default thread count. Built: the lock is held while a test's kernels live.

## Host decisions after the seed's measurement (2026-10-07)

- **Decision 1 amended** (host, 2026-10-07): a fine run's first normal is its bearer's side vector, projected off the first tangent. The rotation rule was measured with about 1,700 spruce runs mismatched at 80.
- **A named residual class: threshold decisions fed by f32 against f64.** A decision the conversion takes against a threshold, fed by the device's float32 geometry on one side and the CPU's float64 on the other, flips where the input lies within the executors' disagreement of the threshold. D1''s margins belong to it, and so do:
  - **Codominance near `FORK_FROM`:** a lateral at 0.4 of its parent plus 2.7e-9 (beech 80 s7: one run's radius off 6 %). Contract A's exact radii are expected to remove it.
  - **`SHORTEST` at 1 mm:** spruce 80 s7 run 62151, a point off 1.0 mm. If it persists after contract A, it is a candidate for deciding on the CPU's fixed-point length; not built now.
  - **Task 7's parity verdict counts this class per tree: how many runs, and how far.**

## Host decisions on task 6 (2026-10-07)

1. **Station numbering: option (b).** A leaf's random values are drawn from its station's identity: the run's first node's lineage, plus the station's place within the run. The engine keys every draw by lineage, which is free of order and survives partial trees and level of detail. A global index ties leaves to whatever order an executor happens to walk. The stream changes for every tree, which generator evolution allows. Show that the leaf scale and scatter distributions are unchanged in law, and view stills (hero and close up, beech and spruce at 80, before and after).
2. **Station frame seed:** the same side seed for bearing runs, on both executors. Count the runs near its jump.
3. **Bearing runs across the seam: option (a).** A fine root's first node never counts among its bearer's bearing children, so dependencies cross the seam one way only. Measure how many bearers change, inside core: a crate-internal test or a `pub(crate)` helper, never a wider API.

## Host decision on the whole-internode ties (2026-10-07)

- **Option (a): station counts and split points come from the grown length** (Σ internode × scale over the phytomers a node draws), record data as D2 asks, on both executors. CPU counts move where a run's arc and chord differ.
- **Worker, built** (TASK6.md, "Grown length"):
  - Each node's grown wood is `Tree::grown` (the conversion's record) on the CPU and a drawn record's word on the device.
  - Each node's share is rounded to whole micrometres and summed as integers on both executors. A float sum of the internodes alone does not tie the same way in f32 and f64: the spruce's fine PA is 3.25 twig internodes, and four nodes sum to exactly 13.
  - A dyadic grid (1/4096 internode) was tried and dropped: it breaks the beech's 25/6.
  - Where a node's grown wood lies within float32's error of a half micrometre, the executors round it apart. That is the named f32-against-f64 class, and the probe counts it per tree.

## Host decisions on task 6's results (2026-10-07)

1. **A seated leaf's direction bound is consistent with the position bound.** It is the position screen (50 µm) divided by the chord of the leaf's segment, with 0.005 rad as its floor. It is written into the probe. A leaf still over it is a failure to explain, and so is the 1.55 mm position outlier on the spruce at 80 s7, which goes into the `SHORTEST` class only if traced there.
2. **Parted runs are located in this task.** A divergence away from both bounds with no known cause is not a residual. The device's per-node station words are read back for one key, and the cause is named, then fixed or classed.
3. **The micrometre grid stays.** Integer sums of a recorded quantity are the contract's own idiom, and they keep every decimal ratio exact. The 14,666.499 µm rounding case stays in the named f32-against-f64 class.

## Host decisions on the leaf bound and frames (2026-10-07)

1. **A seated leaf's direction bound is the sum, 0.005 + 50 µm / chord.** The errors add (the encoding, plus the input's difference), so a max undercounts. "Contact grazing" is a named class: a seat on another triangle of the same rings, its segment agreeing within the screen (1 leaf, spruce 80 s7).
2. **The frame divergence on the spruce (16 to 25 mrad, beyond the screen) is traced within task 6.** Read back the frames along one spruce 40 s1 run, find where they part, then fix it or name its class.

## Host decisions on the handoff (2026-10-07)

1. **Partial conversion.** Production converts only the structure above the cut plus the landed roots, taken from the device settle's structure (D3(c)). Attachments reach the device as the `Tree::attachments` buffer. The probe's stand-in (pruning the converted tree) is deleted once the real path exists. One conversion path, never two.
2. **Curve.** The device packs the fine curve into the renderer's own curve format, offset after the CPU's points, runs and clusters (D5's order). The renderer reads one block and learns nothing new.
3. **Leaves.** The fine block is placed directly at an offset in the one leaf buffer, with no copy, and one mass grid is built over both.
- Then: the submit takes those device buffers; final-output parity on a submitted tree against the CPU path, by named class; the seam close-ups (D4) at a close camera, viewed; the timing harness rerun on the real path (five builds a side, alternating, one per process, both spruce-80 seeds, per-stage medians), and whether the stations numbering pass dominates its 300 ms; the 3× verdict on the real path.

## Host decision on the seam couplings (2026-10-07)

- **Option 3.** A bearer's kind counts every attachment as a child, and its fork easing takes a fine root's codominance from the base radius the root sends up in phase 2. Both come from data the CPU already holds, so no round trip is added. Real codominant fine children still ease their bearer's fork, which option 1 would have removed for thin bearers. Trees change only where a root draws nothing at its seam or merges its first phytomers. Measure how many bearers, view the beech-30 runs that moved, and trace the 3 and 11 extra leaves at 30. If the phase-2 base radius is not available before conversion decides kind and easing, stop and report.

## Host decisions on task 10 (2026-10-08)

1. **Option A: `Roots` becomes async.** One driver serves both executors. Native code blocks with `pollster`, the browser awaits, and the CPU reference returns ready futures. The thread-scope split is made per target, and `Instant::now` is replaced with a clock that is safe on every target. JSPI is out because only one browser supports it. Option C is out because the CPU would hold every node again.
2. **Option (i): the device path is the default in `prepare_async`** for every family the realized settle accepts. The fine module leaves `#[cfg(test)]` and becomes production code. The oak and the palm keep the CPU executor as a capability selection. The reason is named in code and in TASK10.md as a recorded gap, never a silent fallback. The native wiring comes first and is gated on its own (R2: a natively drawn spruce and beech at 80 within tolerance, no per-node readback). The async conversion for the browser follows.
3. **The browser's device path is refused by named device limits.** The buffer sizes it needs are checked against the adapter's `maxBufferSize` and `maxStorageBufferBindingSize`, along with an estimate of device memory, and each check fails with a stated error. `WASM_KEPT` stays only where the CPU holds the nodes. The spruce at 80 is then measured in Chrome.

## Host decisions on task 10's browser findings (2026-10-08)

1. **Starvation: option (b).** The renderer submits a frame only once the last frame's GPU work has completed. The page stays interactive and the build gets the GPU. Spruce 80 → 40 and 40 → 80 are measured. Option (a) is added only if (b) alone still starves a build, and that is reported first.
2. **GPU memory: option (a).** One buffer type destroys itself on drop, and all owners are swept onto it. Then three spruce-80 builds in a row in one page are measured, plus large-to-small rebuilds. If OOM persists, the pool sizes are reported and the host decides on a stated budget.
3. **Spruce leaf order.** Fine stations are placed in their roots' order, structurally, with no sort pass, and the frame cost is re-measured. The beech's 30 leaves are traced now: unexplained is not a class.
4. **Wasm size.** The owner accepted raised ceilings on the branch. The ceiling becomes the measured size plus a small margin, recorded with its reason (the fine module ships), and is flagged in TASK10.md for the PR.
5. **The silver-birch catalogue page is regenerated** (`npm run catalogue:pages`), so `npm test` goes green.

## Host decisions on task 10 part 3 (2026-10-08)

1. **Pools trim to need.** After each build, the kernels' pool and the growth regions shrink to what a same-size retune of the drawn tree needs, and the excess is freed. Growth regions that never shrink are, in effect, a leak. A retune at the same size stays fast; a large-to-small change frees memory.
2. **A failed build never breaks the page.** On "Out of Memory" or any device error, the old tree keeps drawing and the error is reported by name. The invalid bind group after a failure is a defect: it is fixed, and a native test forces a build failure and draws the next frame.
3. **A stated budget before a browser build.** The estimated peak (from the predictor's counts) plus what is held is checked against the browser's budget. The budget is set from measurement on this card and recorded, and a build over it is refused by name. Then measured: a second spruce 80 with an 80 on screen, three in a row, and large-to-small.
4. **The beech's slender bound is decided on the exact recorded form both sides compute**, as SHORTEST was in task 8, and the 30 leaves are shown gone.
5. **The remaining 6 % frame gap** goes to fn-176 (level of detail), noted.

## Host decision on task 11's finding 8 (2026-10-08)

**Option (a): lobed ring vertices on the device.**
- The positions pass (`positions.wgsl`), which lays both blocks' ring vertices, applies the section's lobe profile at each ring's twist phase, as the CPU's wood does (`angular.rs`, `profile`).
- `CompactSurface::qualified` no longer excludes lobed wood.
- Seated leaves on lobed wood take the same path as on round wood: the device path in production, and the compact positions in the whole path.
- Option (b), a named gap in `apart`, was rejected. Lobe depth crossing 0 would switch the tree between two ways of building, and the build time would jump by seconds in the harness. The principles rule out a parameter that switches paths.
- Checked on the spruce, two lobes a tenth deep, at 40 and 80: vertex parity against the CPU's lobed vertices, leaf parity, and a close-up still (TASK11.md).
