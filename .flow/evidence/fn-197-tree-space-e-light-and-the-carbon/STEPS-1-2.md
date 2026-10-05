# fn-197 steps 1 and 2: the light lattice and the rough layout, 2026-10-05

Host decisions 1 to 6 are in DESIGN-OPTIONS.md, section 8. All measures were taken on this machine with release builds: `cargo run --release -p telperion-space --example measures -- <hash|stages|gap> 80 1,7 [species]`. The load average was 2.4 to 4 during the runs. Each species ran in its own process, twice.

## Step 1: the lattice against closed forms (`src/light.rs`, `src/light/tests.rs`)

**The model:**

- Leaf area is deposited by trilinear weights on a lattice with 0.5 m spacing, anchored at the world's origin.
- The field is swept down nine sky directions, layer by layer, bilinear along each ray, with the trapezoid of the density.
- The directions are overhead, four at 60 degrees and four at 30.
- `Light { extinction, sky }` sits on `Request`. It is a site setting: at no extinction, light is exactly 1.

**The tests:**

| Test | Oracle | Result |
|---|---|---|
| Leaf slab, each of the 9 directions | exp(−k·LAI / sin e) (Monsi & Saeki) | Exact to 1e-9 |
| Sphere of even density, 9 directions, 4 points | exp(−k·ρ·chord) | Worst optical-depth error 0.032, against a tolerance of 0.075 |
| Even crown from overhead | GreenLab Q = Sp·(1 − e^(−k·S/Sp)) (Letort eq. 1) | Error 3.3% at a 5 m radius and 1.5% at 10 m: it converges as the rim's half-cell spread shrinks against the crown |
| Sky shares at 0.5 and 1 | Closed-form band integrals of (1 + 2 sin e) and of 1, checked against a quadrature | Within 1e-6; every sky's shares sum to 1; 0 is overhead only |
| Neutral | — | No extinction reads exactly 1 everywhere |
| By degree | — | A leaf or a read moved 1 mm across a cell face moves the light by less than 50 × the step |
| Refusals | — | NaN, negative or above-10 extinction is refused by name, and so is a sky outside 0 to 1 |

**The sky blend:** linear from overhead only (0) to the standard overcast sky (0.5), then linear on to the uniform sky (1). The standard sky's law, radiance ∝ 1 + 2 sin e, is cited from memory (Moon & Spencer 1942, CIE) and not yet checked against its source.

## Step 2: the rough layout grown with the tree (`src/grow/sketch.rs`)

**How it lays the tree:**

- At the end of each cycle, every growth unit grown in it is laid on from where its axis's walk stood.
- That walk is `geometry::Layer`. The final `lay` now uses the same stepper (`src/geometry/lay.rs`), moved out of geometry.rs unchanged.
- Each new axis is framed from its parent by the same `frame`, at the scale its draws give it so far, as `assign` and `scale` will size it.
- Left out:
  - sag;
  - the shedding fade;
  - a woken bud's `kept`.
- Taken at the axis's age when the unit grew:
  - erection and straightening;
  - the straightening's fade, over the reach so far.
- The cycle's leaves then go into the lattice, one leaf per node of 0.005 m² sized by scale (`light::NODE_LEAF`, a stand-in for step 3), and every living apex reads its light.

**Checks:**

- **The stripped oak:** without sag, erection, straightening or shedding, the rough layout puts every phytomer above 0.5 m within 1e-9 m of its final place (`tests/light.rs`). That checks every frame, scale and rank it carries.
- **Light no bud reads leaves every passed species to the bit:**
  - `tests/light.rs`, at age 30, seeds 1 and 7, under three shading lights and a sky-only change.
  - `measures hash` at 80 years matches the hashes taken before any change (`eda1cb31`'s parent):

| Tree, 80 years | Hash before | Hash after |
|---|---|---|
| beech s1 | 9b15095b76c87db9 | 9b15095b76c87db9 |
| beech s7 | c98a7af3be92f198 | c98a7af3be92f198 |
| spruce s1 | b51d107c42cef928 | b51d107c42cef928 |
| spruce s7 | 204b4664253575d2 | 204b4664253575d2 |
| oak s1 | eb3a775b729f8339 | eb3a775b729f8339 |
| oak s7 | 4823045dfcec130b | 4823045dfcec130b |

### Stage times at 80 years (seconds, the second of two runs; extinction 0.5, sky 0.5)

| Tree | Light | Total | Growth | Rough layout | Light sweep | Settle (sizes, shed, girth) | Final lay | Peak memory |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| beech s1 | off | 3.90 | 2.45 | – | – | 0.94 | 0.28 | 2.09 GB |
| beech s1 | on | 5.14 | 1.89 | 1.43 | 0.49 | 0.88 | 0.26 | 2.31 GB |
| beech s7 | off | 2.20 | 1.22 | – | – | 0.61 | 0.22 | |
| beech s7 | on | 3.55 | 1.22 | 1.01 | 0.34 | 0.63 | 0.22 | |
| oak s1 | off | 4.12 | 2.55 | – | – | 0.78 | 0.64 | 1.81 GB |
| oak s1 | on | 4.97 | 1.83 | 1.18 | 0.40 | 0.74 | 0.66 | 2.01 GB |
| oak s7 | off | 4.77 | 2.47 | – | – | 1.07 | 0.95 | 2.55 GB |
| oak s7 | on | 6.90 | 2.45 | 1.83 | 0.52 | 0.99 | 0.85 | 2.76 GB |
| spruce s1 | off | 10.18 | 4.84 | – | – | 2.42 | 2.35 | 4.87 GB |
| spruce s1 | on | 15.96 | 4.07 | 5.55 | 1.13 | 2.30 | 2.32 | 5.64 GB |
| spruce s7 | off | 9.30 | 3.87 | – | – | 2.40 | 2.40 | |
| spruce s7 | on | 16.05 | 3.94 | 5.45 | 1.09 | 2.44 | 2.45 | 5.80 GB |

**Reading the times:**

- **The rough layout is the new cost:** about 0.28 µs per grown phytomer, the same order as the final lay's per kept phytomer.
- **The sweep is a quarter to a third of it.**
- **Light on, read by nothing, adds:**
  - 0.9 to 2.1 s on the oak (+20 to 45%);
  - 1.2 to 1.3 s on the beech (+30 to 60%);
  - about 6 s on the spruce (+55 to 70%).
- **Memory grows by 0.2 to 0.9 GB:** a pencil per grown axis.
- **Growth time is noisy.** The first tree in a process pays for warming the allocator: beech s1 off, 2.45 s against 1.89 s on.
- **Phase F's lever, shading before growth, is not yet in play:** no bud reads light until step 3.

### The gap: rough layout against final lay at 80 years (extinction 0.5)

| Tree | Sky | Extent, final (m) | Extent, rough (m) | Distance from final place: median / 90th / 99th (m) | Bud light, final / rough | Mean abs. difference | Correlation | Buds off by > 0.1 |
|---|--:|---|---|---|---|--:|--:|--:|
| oak s1 | 0.5 | 23.5 × 18.6 × 21.9 | 16.9 × 16.5 × 22.8 | 1.53 / 3.81 / 6.32 | 0.179 / 0.151 | 0.061 | 0.848 | 19% |
| oak s1 | 0 | | | | 0.220 / 0.172 | 0.111 | 0.710 | 35% |
| oak s7 | 0.5 | 19.9 × 21.5 × 20.1 | 17.6 × 16.1 × 22.1 | 1.80 / 3.74 / 5.04 | 0.154 / 0.127 | 0.062 | 0.796 | 20% |
| oak s7 | 0 | | | | 0.184 / 0.139 | 0.098 | 0.721 | 32% |
| oak s1, no sag | 0.5 | 15.9 × 15.5 × 23.2 | 16.9 × 16.5 × 22.8 | 0.53 / 0.88 / 1.21 | 0.146 / 0.151 | 0.029 | 0.960 | 4.5% |
| oak s7, no sag | 0.5 | 16.7 × 15.4 × 22.3 | 17.6 × 16.1 × 22.1 | 0.33 / 0.89 / 4.22 | 0.122 / 0.127 | 0.036 | 0.882 | 8.7% |
| beech s1 | 0.5 | 22.5 × 21.2 × 18.6 | 23.5 × 23.2 × 17.4 | 2.64 / 5.92 / 9.99 | 0.154 / 0.178 | 0.089 | 0.760 | 29% |
| beech s1 | 0 | | | | 0.183 / 0.211 | 0.117 | 0.722 | 37% |
| beech s7 | 0.5 | 18.3 × 19.1 × 17.7 | 21.3 × 21.6 × 16.7 | 2.13 / 3.69 / 6.01 | 0.178 / 0.218 | 0.095 | 0.777 | 32% |
| beech s7 | 0 | | | | 0.214 / 0.266 | 0.132 | 0.723 | 43% |

**Reading the gap:**

- **The oak's gap is mostly sag.**
  - The rough crown is 16.5 to 17.6 m across against the drawn 19.9 to 23.5 m.
  - Removing sag from both cuts the median distance from 1.5–1.8 m to 0.3–0.5 m.
  - It lifts the light correlation from 0.80–0.85 to 0.88–0.96.
  - The rest is the shedding fade and the reach-so-far fade.
- **The beech's gap is erection and straightening** (it has no sag; erection 0.1, straightening up to 1).
  - The rough crown is 1 to 3 m wider and 1 m lower.
  - The limbs keep erecting for decades after their wood was laid, which the rough layout takes only at the age the wood grew.
- **Sky 0 doubles the disagreement.** A column's leaves shade straight down, so a bough displaced sideways falls out of or into its shade wholesale.
- **At this leaf area, bud light is low everywhere (0.12 to 0.27).** That is a step 3 calibration: `NODE_LEAF` and k.

## For the host before step 3

1. **Decision 1's re-lay:**
   - The oak's rough crown is 3 to 7 m narrower than the drawn one, and buds' light agrees at a correlation of 0.80–0.85 under the standard sky. Whether that misplaces the dome is for step 3's stills.
   - The beech has a gap of its own, from erection and straightening, which a re-lay with sag would also close.
2. **Cost:** the rough layout costs about 0.28 µs a grown phytomer, plus 20 to 70% on today's trees, before any growth is saved by shade.
3. **`NODE_LEAF` (0.005 m² per node, one leaf of an oak's size) is a stand-in with no owner.** Step 3 needs a named source per species, or one site value.
