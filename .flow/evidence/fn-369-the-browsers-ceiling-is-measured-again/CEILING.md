# fn-369: the browser's ceiling measured again

## R1, the reading (before any change)

**`foreseen` and `fits`** (`crates/telperion-core/src/pipeline/space.rs`). Both read `KEPT`, which is `Some(WASM_KEPT)` on `wasm32` and `None` natively, so neither refuses anything outside Wasm.

- `foreseen(species, age, KEPT)` runs in `grow` before the engine grows anything, and in `growth.rs` (the hidden growth path) before each grow and each step. It asks `telperion_space::expected_kept`, the closed form's seed-free count, and refuses when that exceeds the ceiling. A species whose buds read light (`Species::reads_light`: any shade, ground or balance hazard, shade size, or turning) has no expected count, so for it `foreseen` passes and only `fits` holds.
- `fits(structure, KEPT)` runs in `grow` right after `telperion_space::grow` returns and before `Establishing::apply` and `convert`. It sums `axes[].phytomers.len()` (the kept phytomers) and refuses over the ceiling. The growth path calls it on each grown structure too.
- Both refuse with `OVER`, `ResourceLimit("the tree keeps more than 4,000,000 phytomers, more than a browser prepares in its 4 GB Wasm memory; grow it younger")`. The number in the message is written by hand; `a_tree_over_the_wasm_ceiling_is_refused_by_name` pins the constant and the message together.

**What holds memory in a Wasm build** (`crates/telperion-wasm`, `pipeline::build`). The binding's allocator counts the live heap (`live.rs`); Wasm memory only grows, so `memory.buffer.byteLength` after a build is the largest the heap ever reached, plus static data and stack.

- *The grower's stores:* `telperion_space::grow` holds the growing and retired phytomers (fn-357: 24 bytes a retired root, fn-358: 72 a growing phytomer) and returns the `Structure`, every kept phytomer on its axis. This is the peak during stage 2.
- *The conversion:* `Establishing::apply` takes the structure and returns another; `convert` builds the `Tree` (nodes, sections, frames, keys, grown, bearing, handed, blooms, clusters, leafing, unfolding). The structure is dropped when `grow` returns. `fitted` shrinks every node buffer to its length before the wood reserves (fn-291 R2a).
- *The curve and the wood:* the surface stage lays out rings and chunks; `surface::budget::check` computes the wood's exact need from its layout and refuses by name (`Error::WoodMemory`, "the wood needs N bytes and M are left in 32-bit Wasm memory") when it exceeds what `memory::left()` reports. So the drawn wood is already guarded in bytes; the kept-count ceiling guards what comes before it.
- *The leaf instances:* foliage placement and the cull, then `foliage::Instances` (three words a leaf) and the flowers' instances.
- *The buffers handed out:* the binding's `Output` keeps the wood chunks (positions, oct normals, u16 indices), the leaf element, the instances and any structure or field buffers until the next build or `release`. JS copies each with `slice()` outside Wasm memory.

**fn-198's BROWSER.md** is in the tree: `.flow/evidence/fn-198-tree-space-f-the-engine-in-production/BROWSER.md`, "The browser's ceiling (host decision 5)". It measured in the dev harness in fresh Chromium pages at seed 42, the spruce by age: 73 kept 4,267,676 and built, 75 kept 4,479,766 and trapped (a `RuntimeError: unreachable`). The kept counts came from the native engine (`harness/grown_count.rs`, since removed), not from the page. The beech and the oak at 80 (2.77 M and 1.68 M kept) built; they were not walked to a trap. The 4,000,000 was set about 6 percent under the largest tree built.

## How this measurement differs

- In Wasm under node, through `dist/telperion.js` as `scripts/test-dist.mjs` loads it, surface and foliage asked, one process a case.
- The refusal is lifted by a **build switch**: the `measure-ceiling` cargo feature of `telperion-core`, forwarded by `telperion-wasm`, turns `KEPT` off and adds one export, `measured_kept`, which reads the kept count `fits` last saw. The module is built into `target/ceiling` and never copied into `src` or `dist`; the shipped module is built without the feature.
- The kept count is the Wasm build's own, read after a trap as well.

## R2, the table (14 Nov 2026, 03:11 to 04:58)

`node scripts/wasm-ceiling.mjs target/ceiling/wasm32-unknown-unknown/release/telperion_wasm.wasm` on `dist` built at `ddb7ba640` plus this task's switch, surface and foliage asked, one node 26.12.1 process a case, through `telperion-heavy 8`. Raw: `raw/table.md`, `raw/cases.jsonl`. "Wasm bytes" is `memory.buffer.byteLength` after the build; 32-bit Wasm stops at 4,294,967,296. Every failure was a trap; no case was refused by name (the wood's byte refusal never fired).

| Tree | Age | Seed | Kept phytomers | Wasm bytes | Outcome | Bytes per kept |
|---|--:|--:|--:|--:|---|--:|
| norway-spruce | 40 | 42 | 1,142,638 | 1,018,232,832 | built | 891 |
| norway-spruce | 50 | 42 | 1,864,323 | 1,330,774,016 | built | 714 |
| norway-spruce | 60 | 42 | 2,710,102 | 2,134,835,200 | built | 788 |
| norway-spruce | 70 | 42 | 3,635,531 | 2,773,942,272 | built | 763 |
| norway-spruce | 80 | 42 | 4,516,307 | 3,693,150,208 | built | 818 |
| norway-spruce | 85 | 42 | 4,913,926 | 3,684,630,528 | built | 750 |
| norway-spruce | 87 | 42 | 5,062,006 | 3,902,799,872 | built | 771 |
| norway-spruce | **88** | 42 | **5,138,207** | 3,586,326,528 | **built** | 698 |
| norway-spruce | **89** | 42 | **5,215,960** | 3,586,326,528 | **trapped** | |
| norway-spruce | 90 | 42 | 5,288,471 | 3,586,326,528 | trapped | |
| european-beech | 40 | 42 | 917,858 | 455,999,488 | built | 497 |
| european-beech | 50 | 42 | 2,051,935 | 999,292,928 | built | 487 |
| european-beech | 60 | 42 | 3,556,709 | 1,869,414,400 | built | 526 |
| european-beech | 70 | 42 | 5,180,321 | 3,203,792,896 | built | 618 |
| european-beech | 80 | 42 | 6,694,767 | 3,684,368,384 | built | 550 |
| european-beech | 90 | 42 | 7,895,358 | 3,874,357,248 | built | 491 |
| european-beech | 100 | 42 | 8,336,340 | 4,099,080,192 | built | 492 |
| european-beech | 110 | 42 | 8,632,432 | 4,128,112,640 | built | 478 |
| european-beech | 120 | 42 | 9,153,670 | 4,180,213,760 | built | 457 |
| european-beech | **121** | 42 | **9,187,129** | 4,280,483,840 | **built** | 466 |
| european-beech | **122** | 42 | **9,183,489** | 3,494,313,984 | **trapped** | |
| european-beech | 125 | 42 | 9,230,988 | 3,496,148,992 | trapped | |
| european-beech | 130 | 42 | 9,101,547 | 3,522,887,680 | trapped | |
| oregon-white-oak | 40 to 400 by 10 | 42 | peak 3,542,083 at 140, 1,207,062 at 400 | 235 M to 1,912,274,944 | all built | 497 to 1,569 |
| telperion | 82 | 42 | 9,066,716 | 4,194,828,288 | built | 463 |
| telperion | 82 | 1 | 5,591,003 | 3,353,280,512 | built | 600 |
| telperion | 82 | 7 | 8,344,437 | 3,928,621,056 | built | 471 |
| laurelin | 80 | 42 | 8,227,011 | 4,274,388,992 | built | 520 |
| laurelin | 80 | 1 | 5,087,956 | 3,124,494,336 | built | 614 |
| laurelin | 80 | 7 | 7,596,573 | 3,705,995,264 | built | 488 |

The oak's rows by age are in `raw/table.md`; its kept count rises to 3.54 M at 140 and falls to 1.21 M at 400 while its memory stays at 1.76 to 1.91 GB, so its bytes per kept rise from about 500 to 1,569.

## What the table says

- **The traps:** the spruce's first trap is at 89 with 5,215,960 kept; the beech's at 122 with 9,183,489 kept, fewer than the 9,187,129 it kept at 121 and built. The oak does not trap up to 400.
- **The rule's number, not pinned:** the smallest trapping kept count is the spruce's 5,215,960; times 0.89 is 4,642,204, rounded down to 4,600,000.
- **Stop 1 fires:** bytes per kept phytomer at the largest built tree are 698 for the spruce (88) and 466 for the beech (121), 50 percent apart; over each species' rows the spruce runs 698 to 891, the beech 457 to 618, the oak 497 to 1,569. The beech keeps 1.76 times as many phytomers as the spruce when it traps. A count is the wrong unit, so `WASM_KEPT` is left at 4,000,000.
- **Stop 3 would fire under the rule's number:** at 4,600,000 every Two Trees case except Laurelin at seed 1 would be refused, though all six build. Telperion at seed 42 keeps 9.07 M and peaks at 4,194,828,288 bytes, 100 MB under the 4 GiB; Laurelin at seed 42 peaks at 4,274,388,992, 20.6 MB under it.
- **Stop 2 does not fire:** the spruce's trap lies at 5.22 M kept, over fn-198's 4.48 M.
- **The memory is not linear in kept:** the high-water marks repeat across different trees (3,586,326,528 for the spruce at 88, 89 and 90; 1,763 M and 1,909 M for most oak ages), and a trap reads about 3.5 GB, so the trap is one large allocation that the address space cannot give rather than a slow fill. Which allocation it is was not measured: **unknown**.

## R6, the wood refusal (test-dist.mjs's refused case)

`raw/r6-probe.jsonl`, the same measurement module, `surface.radialSegments` set to 64 (the parameter's top) and nothing else changed:

| Tree | Age | Seed | Kept | Nodes | Wasm bytes at 64 sides | at 12 sides (above) | Outcome |
|---|--:|--:|--:|--:|--:|--:|---|
| norway-spruce | 70 | 42 | 3,635,531 | 3,516,456 | 2,773,942,272 | 2,773,942,272 | built |
| european-beech | 60 | 42 | 3,556,709 | 3,156,444 | 1,869,414,400 | 1,869,414,400 | built |
| oregon-white-oak | 140 | 42 | 3,542,083 | 1,725,279 | 1,912,274,944 | 1,912,274,944 | built |
| telperion | 82 | 7 | 8,344,437 | 7,056,523 | 3,928,621,056 | 3,928,621,056 | built |

At 64 sides the high-water mark is the same byte for byte as at 12: the wood never sets it; the peak lies before the wood, in growth or the conversion. Telperion at seed 7 with 64 sides, the test's own request, builds at 3.93 GB with the kept refusal lifted. No tree under 4,000,000 kept, at the parameter's top, comes near the wood refusal. Whether another stated parameter (a leaf or lobe count, `lobes`) can push the wood past what is left under the kept bound was not measured: **unknown**.

## Step 1 (host, 14 Nov 2026): which allocation traps, and what holds the peak

`node scripts/wasm-ceiling.mjs <module> --allocations` on the `measure-ceiling` module, whose global allocator (`crates/telperion-wasm/src/measured.rs`) wraps the counting one: live bytes and their high-water mark, every allocation or reallocation of 16 MiB or more with its size, old size and stage, the large blocks live when the mark last rose, and a failed request with its size, stage and the live bytes. The stage is a tag (`telperion_space::measure`) set at boundaries the code already has: each growth year, the settle, lineage order, establishing, conversion, clothing and fitting, rings (and the curve), faces, the leaf plan, foliage, flowers, structure, the binding's handout. Raw: `raw/allocations.jsonl`. MiB below are 2^20 bytes. The tracing does not move the peak: each case's memory matches its untraced run in R2 to the 0.1 MiB (the spruce at 88, 3,420.3 MiB = 3,586,326,528 bytes; the beech at 121, 4,082.2; Telperion 4,000.5; Laurelin 4,076.3).

### The failed request

| Tree | Kept | Failed request | Stage | Realloc of | Live at failure | Memory at failure | Room to grow |
|---|--:|--:|---|--:|--:|--:|--:|
| spruce 89 | 5,215,960 | 768.0 MiB | convert | 384.0 MiB | 1,832.7 MiB | 3,420.3 MiB | 675.7 MiB |
| beech 122 | 9,183,489 | 768.0 MiB | convert | 384.0 MiB | 2,239.9 MiB | 3,332.5 MiB | 763.5 MiB |

Both traps are the same request: `tree.nodes` in `pipeline/space.rs`'s `unfolded`, begun as `vec![Node::root()]` and pushed one node a phytomer, doubling its capacity from 2^22 to 2^23 nodes (96 bytes a node: 768 MiB is 2^23 × 96). Neither fails for want of bytes: 1,588 and 1,093 MiB lie free inside the memory, and 2,264 and 1,856 MiB of the 4 GiB are not live. The memory cannot grow the 768 MiB (676 and 764 MiB of room) and no free hole inside it takes 768 MiB whole.

### At the high-water mark

| Tree | Built | Live at the mark | Stage | Memory then | Not live (freed, unreturned) | Ten largest live blocks (MiB @ stage) |
|---|---|--:|---|--:|--:|---|
| spruce 88 | yes | 2,817.7 | settle | 3,420.3 | 602.6 (17.6%) | 435.5 @ growth 88, 331.6 @ growth 76, 249.4 @ settle, 235.9 @ growth 88, 96.0 @ growth 88, 96.0 @ settle, 64.0 @ growth 67, 27.2 @ growth 88, 18.2 @ growth 73, 18.2 @ growth 73 |
| spruce 89 | trapped | 2,871.6 | settle | 3,420.3 | 548.7 (16.0%) | 435.5 @ growth 88, 331.6 @ growth 76, 281.4 @ settle, 235.9 @ growth 88, 96.0 @ growth 88, 96.0 @ settle, 64.0 @ growth 67, 27.2 @ growth 88, 18.2 @ growth 73, 18.2 @ growth 73 |
| beech 121 | yes | 3,171.1 | foliage | 4,082.3 | 911.2 (22.3%) | 703.5 @ clothe, 175.9 @ clothe, 117.2 @ clothe, 58.6 @ clothe, 58.6 @ clothe, 58.6 @ foliage, 29.3 @ clothe |
| beech 122 | trapped | 2,648.5 | settle | 2,820.4 | 171.9 (6.1%) | 207.8 @ growth 101, 173.0 @ growth 85, 135.9 @ settle, 112.6 @ growth 101, 64.0 @ growth 80 |
| Telperion 82, seed 42 | yes | 3,819.2 | organ | 4,000.6 | 181.4 (4.5%) | 717.0 @ clothe, 179.2 @ clothe, 119.5 @ clothe, 68.0 @ organ, 59.7 @ clothe, 59.7 @ clothe, 29.9 @ clothe, 17.8 @ foliage |
| Laurelin 80, seed 42 | yes | 3,718.3 | organ | 4,076.4 | 358.1 (8.8%) | 644.4 @ clothe, 272.0 @ organ, 161.1 @ clothe, 107.4 @ clothe, 53.7 @ clothe, 53.7 @ clothe, 48.0 @ organ, 26.8 @ clothe, 16.2 @ foliage |

Fewer than ten rows mean fewer large blocks were live. The "@ clothe" blocks are the conversion's per-node vectors after `fitted` shrinks them: `nodes` (96 bytes a node), `frames` (`[f32; 6]`, 24 bytes), a 16-byte vector, `lengths` and `bearing` (`f64`), `keys` (`u32`). Their 2^23-capacity forms in the conversion sum to 1,248 MiB. The spruce peaks in the settle, with the grower's stores still live; the beech and the Two Trees peak in foliage and the flowers, when the shrunk node vectors are the largest live blocks.

### Doublings

| Tree | Large events | Doublings (new = 2 × old) | In growth | In settle | In convert | In foliage / organ |
|---|--:|--:|--:|--:|--:|--:|
| spruce 88 | 434 | 237 | 208 | 6 | 22 | 1 |
| spruce 89 | 418 | 238 | 211 | 6 | 21 | 0 |
| beech 121 | 233 | 122 | 99 | 1 | 22 | 0 |
| beech 122 | 216 | 122 | 100 | 1 | 21 | 0 |
| Telperion 82 | 167 | 97 | 71 | 1 | 22 | 3 |
| Laurelin 80 | 147 | 91 | 61 | 1 | 22 | 7 |

- **Known beforehand:** the conversion's 21 or 22 (each per-node vector, `nodes` among them, the one that fails). Each phytomer kept becomes at most one node, and `fits` has just counted the kept phytomers, so kept + 1 is an upper bound before the first push. The exact count needs a pass over the structure, or `fitted`'s shrink after the reserve.
- **Not known before the year runs, as the code stands:** the growth's 61 to 211, the grower's stores growing with the tree; and the flowers' (Laurelin's florets to 272 MiB, `organ`). Whether either has a bound the code could compute first was not read: **unknown**.
- **The settle's** six on the spruce (8 to 96 MiB) were not named: **unknown**.

### The next cliff

Telperion keeps about 7.8 M nodes at seed 42 (717.0 MiB / 96 bytes) and Laurelin 7.0 M. The next doubling of `nodes`, past 8,388,608 nodes, asks for 1,536 MiB beside a 768 MiB block.

## Step 1, what the code does today (read, not run, unless said)

- **The loader after a trap.** `TreeEngine.build` (`src/browser/core.ts`) does not catch it: the `RuntimeError` reaches the caller and the `TreeEngine` keeps its instance, not disposed. `initializeTreeCore` returns that same shared engine on the next call (it only replaces a disposed one); `telperion/field`'s `engine()` keeps its promise for the page. No path makes a fresh instance. Inside the instance, `build` holds `ENGINE.borrow_mut()` when it traps, and Wasm does not unwind (wasm32-unknown-unknown aborts), so the `RefCell` stays borrowed: the next `build`, `metadata_ptr`, `buffer_ptr` or `chunk_ptr` panics on the borrow and traps again. Exports that read no engine state still answer, and the memory stays readable: **run**, `measured_kept` and `measured_words` were read after each trap above.
- **`fits`** reads the grown `Structure`'s kept phytomers, the sum of `axes[].phytomers.len()`, after growth and before the structure is established or converted, against `KEPT` (`Some(4,000,000)` on wasm32). `foreseen` reads the closed form's seed-free expectation before growth, and only for a species that reads no light; the hidden growth path (`space/growth.rs`) calls both.
- **The wood's refusal** (fn-291 R2f, `surface/budget.rs`) reads `need(layout, coords)`, the exact bytes the wood's chunks, lists, run tables and scratch will take plus one chunk's positions of headroom, against `memory::left()`: 2^32 less `__heap_base` less the live heap the counting allocator holds. It counts freed bytes as left. At the spruce's failure it would have read about 2,264 MiB left while a 768 MiB request failed.

## Step 1, against the direction

1. **A failed allocation cannot become an `Err` where it happens.** `Vec::push` and the like call `handle_alloc_error`, which aborts. The wrapper can record the failure but cannot return a refusal through the pipeline; the refusal would be made by the loader after the trap, from the record that survives it (measured to survive), and the instance replaced. Today nothing replaces it.
2. **Live bytes are not the machine's answer.** Both traps had 1.8 to 2.2 GiB of the 4 GiB not live; the wood's `left()` would have passed them. A refusal from counted bytes passes requests the address space then refuses; only the failed request itself, or exact sizing that removes the hole-finding, reads the machine.
3. **The peak depends on the instance's history.** Every number here is a fresh instance. The harness and the field entry reuse one instance across builds, whose freed blocks change where the next large block fits, so "builds in Wasm under node, fresh" does not say a page that built other trees first can build it. Not measured: **unknown**.
4. **`scripts/test-dist.mjs` holds every preset at or under 3.5 GiB** (`LIMIT`). 3.5 GiB is 3,758,096,384 bytes; Telperion at seeds 42 and 7 (4.19 and 3.93 GB) and Laurelin at seed 42 (4.27 GB) peak over it, so that bar fails them whatever replaces `WASM_KEPT`.
5. **`WASM_KEPT` has three readers:** `grow`, `foreseen` and the hidden growth path (`growth.rs` calls `foreseen` before every step and `fits` after).

## Step 2 (host, 14 Nov): the count goes, the machine answers

Built on this branch and re-proved after merging `fn-351-candidate` at `edfc27090` (the candidate without fn-364's extension rule, station handing and R8). Raw: `raw/m-*.jsonl`, `raw/m-table.md`, `raw/m2.log`, `raw/m3.log`, `raw/m-test-dist.txt`; the pre-merge runs are `raw/e-*`.

**A. The conversion sizes its stores once.** `unfolded` reserves `nodes`, `frames`, `keys`, `lengths`, and where on `unfolded`, `leafing` (the 16-byte store) and `clustered`, with `try_reserve_exact` at 1 + kept phytomers + the relays joined inside a span (a span a relay joins inside draws a node for the relay as well as its own, so kept + 1 alone is not a bound). A refused reserve is `Error::Memory { store, needed }`: "the conversion's frame store needs 247628808 bytes and the machine did not give them". `fitted` still shrinks them. Of the other stores after growth: the **flowers'** matrices are reserved at every bloom's florets and its rachis, and their packed instances at the matrices' count; the **foliage** already reserved its instances from the plan's prediction, fallibly ("foliage allocation"); the **curve** is not traced in a Wasm build (`Request::curve` is false there); the **handout** copies only the leaf and flower element meshes (small) and moves every large buffer. The wood's buffers were already reserved exactly from the layout; their refusal is now the same `Error::Memory`.
- Equal to the bit: the clothed trees of `space_hash` (beech, spruce, oak, palm at 40 and 80, seeds 1 and 7) hash the same before and after A, 16 of 16 (`raw/space-hash-base.txt`, `raw/step2-tests.txt`); `catalogue_identity`'s five pins and `every_preset_grows_the_same_tree_again_and_on_any_cores` pass before and after the merge.
- Pre-merge, the conversion's 21 or 22 large doublings went to none, and the peaks fell: the beech at 121 from 4,082 to 3,854 MiB, Laurelin from 4,076 to 3,830, Telperion from 4,001 to 3,926. Both step-1 traps now built: the spruce to 123 and the beech to 400.

**B. A failure the code cannot return is still named.** The shipped allocator (`telperion-wasm/src/live.rs`) records a refused request's size, the live bytes and the stage tag (`telperion_space::stage`, one static store a boundary). `TreeEngine.build` catches a trap, reads the record through `refusal_ptr` and `stage_name_ptr`/`stage_name_len`, throws "Tree core: the settle needs 370877184 bytes and the machine did not give them (3706444010 bytes live)", and replaces its instance from the module it keeps; a trap with no record is rethrown as it is. `initializeTreeCore` and the field entry hand out the same `TreeEngine`, so its fresh instance is theirs. No public surface changed (the constructor is private). Proof: `a_request_the_machine_refuses_is_recorded_with_its_stage`; test-dist's refused case (below) builds the next request on the same engine.

**C.** Deleted: `WASM_KEPT`, `KEPT`, `fits`, `foreseen` (nothing else read it; `expected_kept` stays for `blend::size`, the examples and the walks' tests), their calls in `grow` and the growth path, `ceiling_tests.rs`, `a_tree_over_the_wasm_ceiling_is_refused_by_name`, the two `foreseen` tests, `surface/budget.rs` with `memory::left`, `set_left` and `live::left`. `telperion.values`'s comment and `docs/native-entry.md`'s promise follow.

**D.** `scripts/test-dist.mjs`: every shipped preset at seeds 1, 7 and 42 builds in Wasm in its own process, peak printed, all 24 at or under 3,584 MiB (the 3.5 GiB bar holds). The refused case: the spruce at seed 42 at 118, two past the first refused (116), is refused by name and the next request builds (1,112,555 nodes).

### E, on the merge

| Tree | Age | Seed | Kept | Peak (bytes) | Outcome |
|---|--:|--:|--:|--:|---|
| spruce | 88 | 42 | 5,248,108 | 3,353.8 MiB (traced) | built |
| spruce | 89 | 42 | 5,333,043 | 3,422.3 MiB (traced) | built |
| spruce | **115** | 42 | 7,372,337 | 4,246,274,048 | **built, the largest** |
| spruce | 116 | 42 | unknown | 3,968,139,264 | trap named: the settle needs 407,127,360 bytes (3,683,770,054 live) |
| spruce | 117 | 42 | unknown | 3,909,025,792 | trap named: the settle needs 452,774,592 bytes |
| spruce | 120 | 42 | unknown | 4,246,274,048 | trap named: growth in year 120 needs 926,999,040 bytes |
| beech | 90 | 42 | 9,132,408 | 3,782,606,848 | built |
| beech | **96** | 42 | 9,836,905 | 4,287,823,872 | **built, the largest** |
| beech | 97 | 42 | 9,956,854 | 4,118,740,992 | refused (reserve): the conversion's frame store needs 239,061,840 bytes |
| beech | 100 | 42 | 10,313,465 | 4,216,324,096 | refused (reserve): the conversion's frame store needs 247,628,808 bytes |
| beech | 121 | 42 | unknown | 4,294,967,296 | trap named: the settle needs 2,992 bytes (3,972,406,120 live) |
| beech | 122 | 42 | unknown | 4,076.2 MiB (traced) | trap named: the settle needs 192,293,376 bytes |
| Telperion | 82 | 42 / 1 / 7 | 6,120,872 / 3,701,260 / 5,264,660 | 3,091,529,728 / 1,822,687,232 / 2,674,458,624 | built |
| Laurelin | 80 | 42 / 1 / 7 | 5,422,804 / 3,368,079 / 4,810,022 | 2,805,071,872 / 1,694,040,064 / 2,489,843,712 | built |

The merged grower keeps more on the beech (10.3 M at 100 against 8.3 M before), so its edge moved from past 400 to 96. Every failure is now named: a reserve the conversion makes returns the refusal, the grower's and the settle's stores trap and the loader names them.

At the high-water mark (traced): Telperion at seed 42 peaks in the flowers with 2,896 MiB live of 2,948 MiB; its largest blocks are the shrunk node store (542.8 MiB), the frames (135.7), the flower matrices (102.2) and the length, leafing and key stores. Laurelin likewise, 2,642 of 2,675 MiB. Remaining doublings are the grower's (46 to 211 a tree), the settle's (1 to 6) and one in foliage; none in the conversion.

**Reuse** (`raw/m-reuse.jsonl`): the oak at 40 then Telperion at 82 in one instance builds (3,091,529,728 bytes after, the same as Telperion fresh), and Telperion twice in a row builds both times, the memory unchanged after the second.

### Open, for the host

- **Unknown:** whether `new WebAssembly.Instance` (synchronous, from the compiled module) is allowed on a browser's main thread for this module's size; agents do not drive a browser. Node allows it.
- The grower's and the settle's stores are where every remaining trap falls; their sizes are not known before the year runs, as the code stands.
