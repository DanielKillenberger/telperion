# fn-197 step 3c: whole-tree Borchert–Honda, 2026-10-05

Host decisions 11 to 13 (DESIGN-OPTIONS.md, section 10) are built. The step-3b oak is recorded as not a pass (decision 13).

## What changed

- **The allocation (`src/allocation.rs`):** Pałubicki's extended Borchert–Honda over the whole tree.
  - A basipetal pass sums each subtree's light Q, where a bud's Q is its presence (base scale × draws' presence) × the light at its tip.
  - An acropetal pass splits the base's vigour at each branching point along an axis between the continuing axis (λ Q_m) and its laterals ((1 − λ) Q_l, each by its own Q).
  - At the tip, the axis's own bud and what carries it on share the rest by Q.
- **Size:** a bud's growth unit is sized by (v / v_expected)^ψ, with v_expected the same pass over the same tree with every bud's light 1.
- **The size stays the bud's own.** It is stored on the phytomer (`Phytomer::size`) and applied after every axis has inherited its bearer's scale (`geometry::scale`).
  - So it scales only that unit's internodes and its pipe (girth), and nothing compounds.
  - The rough layout lays and deposits leaves at the sized scale, and keeps the unsized scale for what the unit bears.
- **λ is per PA** (`PaState::apical_control`), neutral 0.5.
- **The sibling-relative form of step 3b is gone.**

## λ's neutral value, proved

- **At λ = 0.5 every split is in proportion to light.** The shares telescope from the base, so each bud's vigour is exactly its own Q. Test: `an_unbiased_split_gives_every_bud_its_own_light`, on a nine-axis tree with a whorl and continuations, to 1e-12.
- **So the size at λ = 0.5 is the bud's own light to ψ,** the absolute form of step 3: the split itself redistributes nothing.
- **"Today's tree," with no light effect, is ψ = 0 at every λ.** (v / v_expected)^0 = 1 exactly, and the neutral test (`tests/light.rs`) holds every species to the bit.
- **α cancels.** v and v_expected both scale with the base's vigour α·Q_total and α·W_total, so α divides out of every size. It is not built as a setting: a setting that changes nothing is deleted, not walked (docs/principles.md). Decision 11 asked for α to be walked; the host's call whether that stands.

## Oracle (allocation tests, all green)

| Test | Checks |
|---|---|
| `one_branching_point_splits_as_palubicki_states` | v_lat = v (1 − λ) Q_l / (λ Q_m + (1 − λ) Q_l), and v_main, at four (λ, Q) pairs, to 1e-12 |
| `the_trees_vigour_is_conserved_at_every_lambda` | Every bud's vigour sums to the tree's Q at λ from 0.1 to 0.95 |
| `an_unbiased_split_gives_every_bud_its_own_light` | λ = 0.5 telescopes |
| `a_shaded_limb_receives_less` | Two limbs alike but for light: the shaded one gets less vigour, and less against a uniformly lit tree, at λ 0.3, 0.5 and 0.7 |
| `vigour_moves_by_degree_as_light_does` | One bud's light moved to nothing in 1,000 steps moves no vigour by more than 0.01 a step |

## Walks (decision 12)

- **The gate:** `cargo test --profile ci -p telperion-space` is green: 135 s, the walks binary 106 s.
  - The light walks: φ, ψ, λ at the trunk and the limbs, leaf area, sky and extinction, on the leafy walk tree with the full lay at cycle 10.
  - A sample in leaf with the full lay: a limb's sag, a limb's survival (a lifespan is a whole number of units, so it is never walked) and a sleeping-bud probability.
  - Every setting, light off.
- **The every-setting in-leaf walk** is `#[ignore]`, a slow suite outside the gate. Its doc comment gives the command to run it.

## Renders (oak, 80 years, sky 0.5, φ 1, ψ 1, full lay on, under the GPU lock; I viewed every still)

Sheets in `raw/step3c/`:

- **`explore.png`:** seed 1 at λ 0.45, 0.5 and 0.55 with ψ 1, and λ 0.5 with ψ 2, beside S1, S2 and round 4.
- **`seeds-bare.png`, `seeds-whole.png`:** seeds 1, 7, 2, 3 and 4 at λ 0.45, under round 4's same seeds and the references.

| Seed | Round 4: height, width (m), leaves | BH: height, width (m), leaves, wood |
|--:|---|---|
| 1 | 21.9, 23.5 × 18.6, 2.16M | 20.1, 23.5 × 21.2, 0.88M, 6.9 km |
| 7 | 20.1, 19.9 × 21.5, 2.22M | 18.1, 23.8 × 26.1, 0.99M, 7.8 km |
| 2 | 20.7, 19.7 × 25.2, 3.51M | 18.5, 18.2 × 25.1, 1.34M, 10.4 km |
| 3 | 20.8, 22.0 × 22.1, 3.09M | 19.3, 25.5 × 27.3, 1.18M, 9.0 km |
| 4 | 19.0, 24.8 × 21.8, 0.87M | 18.4, 21.8 × 21.5, 0.41M, 5.4 km |

At seed 1, λ 0.5: 19.3 m, 0.83M leaves. At λ 0.55: 18.6 m, 0.77M. With ψ 2: 18.1 m, 0.57M.

### Reading

- **Size no longer balloons.** Every seed is 1 to 2 m lower than round 4, as wide or up to 5 m wider, and carries 40 to 60% of round 4's leaves: light bounds it.
- **The outline still does not close into a dome.**
  - The trees read as open, layered tiers of slender boughs with long, thin reaches.
  - In leaf they read as foliage clumps along the boughs with sky between them. Round 4's crowns were denser, and S2's is a closed dome.
  - Seeds 7 and 3 are broad and flat-topped, the nearest to an open-grown oak's spread. Seed 2 is a wide, low, layered fan; seed 1 a narrow, open column; seed 4 sparse.
- **Bare:** a lighter, more graceful branch structure than round 4's heavy web, but much sparser than S1's dense crown of fine twigs.
- **λ barely shows** between 0.45 and 0.55: a little wider at 0.45, a little narrower at 0.55.
- **Why it thins rather than fills:**
  - Against a uniformly lit tree, every bud's size reads its light relative to full light, so on average units shrink by the crown's mean light (about 0.25 to 0.3 at this leaf area, step 3b's gap table).
  - The lit periphery grows near whole while the interior is cut back. The scaffold reaches out, and the fill between thins.
  - Borchert–Honda conserves the tree's resource, but measured against a uniformly lit tree, that resource is the light the crown actually catches. That is less than what round 4 grew without light.
- **Not a pass:** the bounded size is the gain. The dome and the dense fine web are not there.

## For the host before step 4

1. **The reference vigour.** Measuring v against a uniformly lit tree makes the whole tree shrink by its mean light.
   - Measuring it against the tree's own mean vigour per presence would keep the average unit whole and only redistribute toward light.
   - That is a design choice: decision 11 named the uniform reference.
2. **α cancels under the ratio:** keep it deleted, or define what it should change.
3. **The leaf area** (30 cm² a node, estimated) and k 0.5 set the mean light, and so how much the crown thins. Neither has been calibrated against the oak.
