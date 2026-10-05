# fn-197 friction

## 2026-10-05, design research: no per-stage timing in the engine

- **Doing:** costing the light and carbon options against today's 80-year grow time.
- **Slowed by:** `grow()` is the only public entry, and nothing reports how its time splits between the cycle loop, `assign`, `shed`, `thicken`, `place` and sag. I wrote a throwaway example that times whole grows and a sag-off variant, then deleted it. That gives the sag pass by difference, and the rest stays unknown.
- **Cost:** about 10 minutes and two release builds. The split the design needs most (the cycle loop against `place`) is still unknown.
- **Would remove it:** a `timings` example, or a feature-gated stage timer in `grow.rs`, that prints each stage's time for the shipped species at 80 years.

**Resolved 2026-10-05 (host decision 6):** `grow_staged` reports each stage as it ends, and `cargo run --release -p telperion-space --example measures -- stages` prints growth, rough layout, light sweep, settle and final lay, with peak memory.

## 2026-10-05, step 2: a loaded machine skews timings

- **Doing:** timing stages at 80 years.
- **Slowed by:** the first run had a load average of 9 from other sessions. The spruce under light then read 40 s, of which 17 s was settle, against 16 s and 2.4 s in a quiet run.
- **Cost:** one repeated run, about 5 minutes.
- **Would remove it:** checking the load average before timing, as the runs here did afterwards; or a dedicated timing machine.

## 2026-10-05, step 2 gate: one failure inherited from the oak branch

- **Doing:** the workspace gate (`cargo test --profile ci --workspace --no-fail-fast`) at the end of step 2.
- **Slowed by:** 1,048 tests passed and one failed: `telperion-core --test material_detail`, `every_shipped_young_wood_row_crosses_the_wire_the_page_sends_unchanged`.
  - It lists `oregon-white-oak` among the presets with a young-wood row.
  - The oak's round 4 commit (`e1b6e1ea`, fn-195) added that row without updating the test.
  - fn-197 changes nothing in telperion-core.
- **Cost:** one gate run (about 20 minutes) to establish that.
- **Would remove it:** fn-195 updates the test's expected list. It belongs to the oak's branch, not here.

## 2026-10-05, step 3: no measured leaf area for the oak in reach

- **Doing:** sourcing the oak's leaf area per node.
- **Slowed by:**
  - Neither MODEL-OAK.md nor fn-195's SOURCES.md has a leaf size.
  - Web search gave only "around 10 cm in length" (Woodland Trust) and "one leaf per node" (Go Botany).
  - HAL's leaf-morphology paper sits behind a bot wall.
  - The paper index had two Q. robur leaf-trait papers with no full text.
- **Cost:** about 10 minutes and five searches. The value stays estimated: 30 cm² a node.
- **Would remove it:** a species' literature stage that collects leaf size with its source, as the add-species pipeline does for the old generator.

## 2026-10-05, step 3b: the walk suite outgrew a 30-minute run

- **Doing:** running every walk after decision 9 (refine depth 6) and decision 8's in-leaf walk of every setting.
- **Slowed by:** the whole `walks` test binary did not finish within the 30-minute background limit and was stopped. Before these changes the same binary took about 95 s.
  - Depth 6 refines each walk's three steepest steps through six levels of nine trees, against three.
  - The in-leaf walk repeats every setting's 600 trees with light, the rough layout and the full lay.
- **Cost:** 30 minutes of wall time with no result, then a rerun of each walk alone to time it.
- **Would remove it:** refining only the steps the 3-level check flags; or running the in-leaf walk of every setting on a sample of settings, or as a separate, slower suite. The host's call: the gate runs this binary.
- **Update, same day:** run alone, the light walks took 88 s, every setting's walks 91 s and the release law 2 s, all green at depth 6. The in-leaf walk of every setting hit its 3,000 s timeout without finishing.
  - Leafy walk trees with relative allocation can outgrow a shoot's whole size along a lineage, so some settings grow far larger trees (see STEP3B.md).
  - The test is committed as `#[ignore]`, and decision 8's "a walk of any setting stays within the bound" is unchecked: `NEEDS_HUMAN` on its scope.

## 2026-10-05, step 4: the balance's memory has no source (host decision 20)

- **Doing:** building the carbon balance's remembered average.
- **Slowed by:** no source in reach gives how many years a branch's carbon balance is "remembered" before it is shed. `MEMORY` = 0.5 (a half-life of a year) is an estimate.
- **Cost:** none yet. It is a value that no source backs.
- **Would remove it:** a literature search (Takenaka 1994's shedding criterion; Sprugel 1991 and Sprugel et al. 1991 on branch autonomy), as its own small task.

## 2026-10-05, step 4b: no measured dead-branch persistence for oaks

- **Doing:** sourcing how long oaks hold dead branches (host decision 18).
- **Slowed by:**
  - Web search found only an arborists' page ("oaks often retain dead branches for years").
  - The paper index's self-pruning preprint had no full text.
  - The *Q. serrata* crop-tree paper gives dead-branch zones, not years.
  - Kint et al. 2010 (self-pruning of young *Q. robur*) is abstract-only.
- **Cost:** about 10 minutes and five searches. The 5 years stays estimated.
- **Would remove it:** access to Kint et al. 2010's full text, or the oak branch's own literature stage.
