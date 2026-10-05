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
