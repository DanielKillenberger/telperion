# fn-253: where the flowers are seated and what holds them

## R1: measured before any code (worker, 2026-10-09)

Measured on branch `fn-253-flowers-hang-from-the-tree` at base `750460d3` (F4 with fn-248 merged).

**The instruments.**
- `crates/telperion-core/examples/bloom_seats.rs`, run as `cargo run --profile ci -p telperion-core --example bloom_seats -- <preset> [age] [seed]`. It builds the tree through `pipeline::build`, regrows the engine structure to recover each node's age (PA), axis and wood age by phytomer key (the conversion keeps neither), and replays `bloom.rs`'s presence draw to say which seats flower. For each drawn floret it measures the distance from the nearest flowering seat's wood surface to the floret's base.
- The device: `preset_florets` in `generation/fine/florets_tests.rs`, an ignored probe that builds a shipped preset on the real path and matches its florets against the CPU whole tree's, under the GPU lock.
- Logs: `raw/bloom_seats-before.log`, `raw/preset_florets-before.log`.

### Where each inflorescence is seated

Both trees flower on one age only, 14 (the beech's short shoots; `/supernatural/flowering/age14`).

| | Laurelin 80, seed 42 | Telperion 82, seed 42 | Laurelin 80, seed 1 | Telperion 82, seed 1 |
|---|---|---|---|---|
| Nodes | 3,331,690 | 3,572,076 | 1,939,618 | 2,075,326 |
| Bloom seats (every drawn node of age 14) | 1,934,353 | 2,072,892 | 1,130,053 | 1,208,178 |
| Short-shoot tips (fn-223 clusters) | 196,853 | 209,747 | 114,488 | 122,312 |
| Seats that flower (share 0.13 / 0.09) | 279,853 | 208,167 | 163,472 | 121,494 |
| of those at their axis's tip | 28,624 (10 %) | 21,306 (10 %) | 16,669 (10 %) | 12,273 (10 %) |
| Short shoots bearing more than one | 79,278 of 141,129 | 55,503 of 125,682 | 46,371 of 82,410 | 32,565 of 73,184 |
| Grown wood between seat and tip, mm p10 / median / p90 | 0 / 19.8 / 66 | 0 / 19.8 / 66 | 0 / 18.0 / 66 | 0 / 18.1 / 66 |
| Seat wood diameter, mm p10 / median / p90 | 1.2 / 3.8 / 6.6 | 1.2 / 3.8 / 6.6 | 1.1 / 3.3 / 6.6 | 1.1 / 3.3 / 6.6 |
| Florets drawn | 1,677,467 | 1,039,409 | 979,974 | 606,710 |

Seats by wood age (years since the phytomer under them grew), Laurelin seed 42: 0: 85,287; 1: 83,602; 2: 58,476; 3: 35,865; 4: 16,623. Telperion's shares are the same to the percent. A short shoot lives four years and adds a growth unit each year, so its wood is 0 to 4 years old and up to about 66 mm long; nine in ten flowering seats stand along it, not at its tip, and more than half the bearing short shoots carry two or more inflorescences at different points along it.

### What connects a floret to the wood

Nothing that is drawn.
- **The rows.** Laurelin's rachis is 0.72 m with a 24 mm pedicel, six florets, hanging (droop 1). Telperion's is 0.39 m with a 45 mm pedicel, five florets, standing (droop -1).
- **The gap,** from the seat's wood surface to the floret's base, mm p10 / median / p90: Laurelin 50 / 167 / 391 (seed 42) and 50 / 170 / 394 (seed 1); Telperion 49 / 118 / 245 and 48 / 116 / 245. Every floret has its seat within one reach (rachis plus pedicel).
- **Checked against the code.** `bloom.rs` `inflorescence` lays the axis (`Axis::laid`, eight segments from the node along the bud's side, turned toward hanging or standing by the droop), places floret `i` at `t = (i + 0.5) / florets` along it, and pushes one transform per floret at `at + stalk * pedicel`. The organ element (`bloom.rs` `element`) is the petals and the stamens alone. The axis and stalks are used for placement and never emitted, as fn-217's review said (ASTRA-REVIEW-DESIGN.md §5).

### The seating rule, checked against the code, on both executors

- **CPU.** `pipeline/space.rs` `converted` pushes a `Bloom` for every node it draws on an axis whose age's flowering share is above zero (`share > 0.0 && d.relay.is_none()`), not only the tip. The presence draw (`bloom.rs` `presence`, keyed by the node's lineage under `BLOOM`) then keeps the share of them that flowers.
- **Device.** `dress.wgsl` `place` writes the age's flowering share into every own-drawn record (word 28, the sweep node's word 29), and `blooms.wgsl` `grown_bloom` reads it on every fine node. The tip of a short shoot is marked (`TIP`) only for its cluster; blooms do not read the mark.
- **They agree.** The probe matched every floret: Laurelin at 30 (seed 1) 16,630 on both sides, Telperion at 30 9,548, and Laurelin at 80 (seed 42) 1,677,419 device florets plus 48 in the CPU block against the whole tree's 1,677,467. Every floret is exact or within fn-229's screen, except 6, 5 and 127 per side that the probe's "turned" class misses because its screen is the catkin fixture's 3 mm stalk, not these trees' 24 to 45 mm (largest position difference among the matched: 1.4 mm).

### What it means

The owner's "placed randomly and attached to the air" is both causes on record, measured:
1. **Randomly:** nine in ten inflorescences stand somewhere along a short shoot's four years of wood, often two or three to a shoot, not at its tip.
2. **In the air:** every floret hangs 5 to 39 cm from its wood (median 17 cm on Laurelin, 12 cm on Telperion) with nothing drawn between them.

**One consequence for R2's values.** Tip seating caps the inflorescences at one per short shoot: 114,488 tips on Laurelin at seed 1 against today's 163,472 flowering seats, so even a share of 1 seats 70 % of today's inflorescences. Holding the florets' coverage (fn-248 held it within 1 %) then takes the share up near 1, and possibly more florets per inflorescence; that is measured on the organ mask after R2 lands.
