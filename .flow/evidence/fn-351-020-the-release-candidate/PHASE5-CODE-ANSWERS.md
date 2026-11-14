# Phase 5 code answers, 14 Nov 2026

By a Fable agent, read-only. Nothing was built, run or committed; no design is proposed. Answers the seven questions the host put after `PHASE5-SURVEY-2026-11-13.md`.

**Read at.** Code: `.worktrees/fn-364`, head `8f6b017e2` (fn-363's charge by height and fn-364's stretch are in). Records: this folder, the fn-357, fn-358 and fn-327 evidence, and the goal log at the end of the fn-351 spec.

**Tags.** [code] read in source, cited as file, function and line at that head. [record] a filed number or reading. [derived] arithmetic or inference done here from [code] and [record]. [unknown] not found or not run. Paths starting `grow/`, `lineage/`, `closed_form/`, `presence/`, `shed.rs`, `girth.rs`, `species.rs` are under `crates/telperion-space/src/`; `pipeline/` is under `crates/telperion-core/src/`; presets are `crates/telperion-core/presets/*.values`.

**A reading rule for preset rows.** A row a `.values` file does not state holds the passed age's value [code: `presets/space.rs`, `rows`, lines 225-250, writes only what differs from `chain::passed()`]. The defaults that matter here [code: `chain.rs`, `passed`, lines 34-48 and on]: `continuation` 1, `viability` 1, `shedding` never, `relay_ended` 0, `relay_failed` 0, `abortion` 0, `shade_size` 0, zone nodes 0 to 0, `form.internode_spread` 0.

## The answers in short

1. `lasting` has one reader, `Shoot::shed_certain`, and both are switched off in the product's cycle. On the shipped presets a light-read turn cannot break what `lasting` assumes, but nothing in the code holds that; the function does not read `shade_short`.
2. `shed_certain` is not live in the one cycle: `Paused::with` sets every `lasting` entry to none, so it returns false before it reads `rules.age`.
3. By table, `lasting` closes more than the survey listed: the spruce's ages 10 and 11 as well as 12. Today's beech shoot is over the 0.5 m line on both measures (0.531 m expected, 0.86 m at most), because the short shoot's life went from 4 to 12 years after fn-339's figure was taken.
4. A leafy lateral axis holds about 740 bytes of fixed records, 40 a growth unit and 84 a phytomer. The balance, the allotment's stretch, the layout, the deposit and the light read each read its own geometry; the open pass, the wood pass and retirement do not.
5. Its phytomers cannot be rebuilt from key, species, cycle and bearer's node alone in a lit tree: each year's survival, shedding, size and stretch read the light at its own tip and the tree's balance and allotment, and its wood reads its leaf's history.
6. The newest full per-pass split on record is fn-327's (3 Nov, before fn-349 and fn-354). A split for any preset needs no new code: the `early` example prints it.
7. Nothing in `src/`, `harness/`, `tests/browser/` or `scripts/` sets a non-zero `fidelity`; the harness panel offers the row as a hand control. The rows' readers are six places in two crates, listed below.

## 1. Does a light-read turn change which kinds `lasting` closes? (survey question 10)

### What `lasting` computes and assumes

[code: `shed.rs`, `lasting`, lines 27-61]

```rust
for pa in (0..n).rev() {
    let state = &species.states[pa];
    let wakes = state.zones.iter()
        .any(|z| z.rate > 0.0 && z.dormant.iter().any(|&p| p > 0.0));
    let relays = state.relay_ended > 0.0 || (state.relay_failed > 0.0 && state.viability < 1.0);
    if state.lifespan <= 0.0 || wakes || relays { continue; }
    let mut after = Some(0.0f64);
    let mut bears = |q: Option<usize>| {
        after = match (after, q) {
            (Some(a), Some(q)) if q > pa => last[q].map(|b| a.max(b)),
            (_, Some(_)) => None,
            (a, None) => a,
        };
    };
    if let Some((next, _)) = species.successor(pa) { bears(Some(next)); }
    for zone in &laterals[pa] {
        for (j, _) in zone.iter().enumerate().filter(|&(_, &p)| p > 0.0) {
            bears(species.lived(j).map(|(q, _)| q));
        }
    }
    last[pa] = after.map(|a| state.life().ceil() + a);
}
```

It assumes three things [code, the same lines and the doc comment at 18-26]:

- **The kinds an age bears are exactly its tables' entries above 0** (`laterals[pa]`, which is `PaState::laterals`, the zones' `lateral` tables spread by the rhythm), plus its successor.
- **A lateral is never younger than its bearer** (`q > pa`, else none), so the oldest ages are bounded first.
- **Viability, abortion and the balance only shorten life.**

It reads no light row: not `shade_short`, not `shade_hazard`, not `balance_hazard` [code: no such field appears in the function].

### What a turn does

[code: `grow/unit.rs`, `grow_unit`, lines 78-103]

```rust
let shade = &species.states[drawn_pa].shade_short;
let turns = species.states[drawn_pa].turns();
let light = self.light(apex.axis);
let kind = match turns {
    true => lineage::kind_under_shade(shade, drawn_pa, light, key),
    false => drawn_pa,
};
let Some((lateral_pa, reach)) = species.lived(kind) else { continue; };
let keyed = species.lived(drawn_pa).map_or(lateral_pa, |(p, _)| p);
if self.shed_certain(lateral_pa, cycle, apex.axis) { ... }
```

- A bud drawn as PA `drawn_pa` becomes PA j with probability `(1 - L^e) × shade_short[j] / e`, L the light at its bearer's tip [code: `lineage/turn.rs`, `kind_under_shade`, lines 26-35].
- So a bearer can carry a lateral of a kind j that its own `lateral` table does not list. `lasting` never sees j as borne by that bearer.
- Validation refuses only a turn into the bud's own age [code: `species/validate.rs`, `shade_validate`, lines 254-270: "a bud cannot turn into its own age"]. It does not refuse a turn into a younger age, or into an age the bearer's bound did not count.
- The lateral itself is tested at its turned kind: `shed_certain(lateral_pa, ...)` takes the kind after the turn.

### Can a turn violate the bound?

- **In general, yes** [derived from the code above]. If age p is closed by table, bears kind q, and `shade_short` of q turns it into j where `last[j]` is none or larger than every bound p's table reached, then p's bound is too short. Nothing in `lasting` or in validation prevents such a species.
- **On the shipped presets, no** [derived from the preset rows]. `shadeShort` is stated in three presets only, the beech and the Two Trees, and only as age 9 into 14 and age 12 into 14, each 0.38 [code: `european-beech.values:600, 642`; `laurelin.values:595, 637`; `telperion.values:605, 647`].
  - The turning kinds are 9 (branch) and 12 (shoot). Every age that bears a 9 is already unbounded, because the beech's age 9 bears its own age (`zones/2/lateral/9 = 0.08`, line 612), so `last[9]` is none and every bearer of a 9 inherits none. The only bearer of a 12 is age 9 (lines 607, 616), unbounded for the same reason.
  - The two kinds the beech closes are 12 and 14. Age 12 bears only 14 (line 649), and age 14 states no `shadeShort`, so no turn is drawn inside either closed subtree.
  - The turned kind, 14, is itself closed (bound 12 cycles), and `shed_certain` reads that bound for the turned lateral.
- **So the closed set is unchanged by `shade_short` on today's presets**: the beech closes 12 and 14 with it or without it.

### Where `lasting` is read at all

- One reader: `Shoot::shed_certain` [code: grep over the crate; `grow/shoot.rs:318`].
- It is computed only for a grower with no rough layout: `if options.certain != Certain::Grow && sketch.is_none()` [code: `grow.rs`, `start`, lines 185-189], and a rough layout is grown "wherever light shades", wherever a ground hazard is set, and wherever wood is laid by leaf (lines 180-182).
- The one cycle overwrites it with none (question 2).
- With no rough layout, `Shoot::light` returns 1 [code: `grow/shoot.rs`, `light`, line 244: `self.reads.pencils.get(i).map_or(1.0, |p| p.light)`], so `L^e` is 1 and no bud turns. Where `lasting` is live, a turn cannot happen [derived].

A consequence for the survey's use of the word: "the kinds `lasting` closes" is a classification the survey borrowed. In the product today `lasting` classifies nothing; it is a table a test-only engine reads.

### A second thing a turn changes, which `lasting` does not carry

A turned lateral keeps the key of the kind its table drew: `self.sprout_keyed(key, (lateral_pa, keyed), ...)` and `Origin::Seed | Origin::Lateral { .. } => (key.0, key.onto(keyed))` [code: `grow/unit.rs:124`; `grow/shoot.rs`, `sprout_keyed`, lines 341-358]. An axis's lineage therefore does not say its kind: a beech short shoot born of a turned shoot bud carries a shoot's lineage. This bears on question 5.

## 2. Is `shed_certain` live in the one cycle? (survey question 11)

**No.** [code]

```rust
// grow/shoot.rs, shed_certain, lines 317-325
pub fn shed_certain(&self, pa: usize, cycle: u32, parent: usize) -> bool {
    let (Some(last), delay) = (self.rules.lasting[pa], self.species().states[pa].shedding)
    else { return false; };
    let horizon = self.horizon(parent).max(delay + 1.0);
    let age = f64::from(self.rules.age);
    delay.is_finite() && horizon <= age - (f64::from(cycle) + last)
}
```

```rust
// grow/cycle/paused.rs, Paused::with, lines 99-104
let mut g = start(species, request, Options::default())?;
(g.one, g.rules.one) = (true, true);
g.working = Some(Box::new(working));
g.sketch.get_or_insert_with(|| Sketch::new(request.light));
// Every lateral is grown: while it lives its leaves shade.
g.rules.lasting = vec![None; pas].into();
```

- Every product tree goes through `Paused::with`: `grow` calls `cycle::grow_cycle`, which calls `run`, which calls `Paused::with` and `grow_to` [code: `grow.rs`, `grow`, lines 63-65; `grow/cycle.rs`, lines 45-63]. The light capture sets the same [code: `grow/cycle/capture_light.rs:30`].
- With every entry none, `shed_certain` returns false at its first line and never reaches `rules.age`. `Grower::marked` stays empty.
- It is live only in the test-only whole-tree engine `grow::run` (`#[cfg(test)]`, `grow.rs:144-166`), and there only where no rough layout is grown. Its two tests, `a_lateral_certain_to_be_shed_is_shed` and `a_tree_grows_the_same_without_what_it_is_certain_to_shed`, call that engine with a `Certain` option [code: `grow/tests.rs:63-87`].

**How it sits with year-independence.** In the one cycle it does not sit against it, because it is dead there [derived]. The tests in `grow/cycle/paused/tests.rs` (`the_tree_at_a_year_is_the_horizons_prefix`, `a_year_is_a_state_not_a_horizon`, `a_growth_goes_on_past_its_first_request`, `every_year_is_the_horizons_prefix`, `a_tree_paused_and_grown_on_is_the_tree_grown_at_once`) all grow through `Paused`. Were `lasting` ever restored in the one cycle, `shed_certain` would make the grown state depend on the age asked for, since `rules.age` is the first request's age and a lateral left ungrown for a tree of 40 would be missing when that grower went on to 80 [derived from the two snippets; not run].

**Where the age asked for is still read in the one cycle** [code]:

- `Grower::mark` reads `self.rules.age` for a woken bud's kept share [`grow/settle.rs:19, 31`]. `Paused::settle` settles a copy whose `rules.age` is set to the age grown [`grow/cycle/paused.rs`, `settled_copy`, lines 319-320]. `Paused::finish` settles the grower itself, whose `rules.age` is the first request's; `grow_cycle` only calls it after `grow_to(request.age)`, so the two agree there.
- `Paused::digest` hashes `g.rules.age` and `g.marked` [`grow/cycle/paused.rs:296-306`].
- The windows are built for `request.age` and rebuilt further when growth passes them; "an entry is the same however far its table runs" [`grow.rs`, `year`, lines 381-386; `presence/windows.rs` module doc].
- Retirement reads the age grown so far, `cycle - 1`, never the age asked for [`grow/cycle/retire.rs:151-152`].

## 3. Expected reach against the extent bound (survey question 9)

### Which fine ages `lasting` would close, by table

[derived from `lasting` above and the preset rows cited]

| Species | Age | Why closed or not | Bound, cycles |
|---|---|---|---|
| Oak | 14 short | lifespan 3, continuation 0, no laterals | 3 |
| Oak | 13 twig | lifespan 5, bears 14 | 5 + 3 = 8 |
| Oak | 12 shoot | lifespan 4, bears 14 | 4 + 3 = 7 |
| Oak | 9 branch | `relayEnded = 1` (line 463): not closed | none |
| Oak | 8 sprig | bears 9: not closed | none |
| Beech | 14 short | lifespan 12, continuation 0, no laterals | 12 |
| Beech | 12 shoot | lifespan 5, bears 14 | 5 + 12 = 17 |
| Beech | 9 branch | bears its own age (line 612): not closed | none |
| Spruce | 12 shoot | lifespan 2, continuation 0, no laterals | 2 |
| Spruce | 11 spur | lifespan 40, continuation 0, bears 12 | 40 + 2 = 42 |
| Spruce | 10 branchlet | lifespan 10, **continuation 1 (unstated)** into 11, bears 12 | 10 + 42 = 52 |
| Spruce | 9, 8 | `dormant/10 = 0.45` at `rate = 0.3`: sleeping buds wake, not closed | none |
| Birch | 14 short | lifespan 6, continuation 0, no laterals | 6 |
| Birch | 9 | bears its own age (`zones/1/lateral/9 = 0.005`): not closed | none |
| Birch | 8 | bears 9: not closed | none |

The survey listed the spruce's shoot only. By the table the spruce's spur (11) and branchlet (10) close too. The branchlet is the one closed age whose continuation is not 0. The birch has no age 10 to 13.

### The rows used

| Species, age | internode m | nodes a unit (min to max, mean) | lifespan | viability | laterals | spread |
|---|---|---|---|---|---|---|
| Oak 14 | 0.006 | 3 to 5, 4 | 3 | 0.9 | none | 0 |
| Oak 13 | 0.012 | 1 + (2 to 3) + 1 = 4 to 5, 4.5 | 5 | 0.98 | 14 | 0 |
| Oak 12 | 0.015 | 1 + (2 to 3) + 1 = 4 to 5, 4.5 | 4 | 0.93 | 14 | 0 |
| Beech 14 | 0.006 | 3 to 5, 4 | 12 | 0.9 | none | 0 |
| Beech 12 | 0.025 | 1 + (2 to 3) = 3 to 4, 3.5 | 5 | 0.95 | 14 | 0 |
| Spruce 12 | 0.02 | 2 | 2 | 0.9 | none | 0 |
| Spruce 11 | 0.006 | 0 + 0 + 1 = 1 | 40 | 0.99 | 12 | 0 |
| Spruce 10 | 0.016 | 0 + 2 + 1 = 3 | 10 | 0.99 | 12; continues into 11 | 0 |
| Birch 14 | 0.003 | 2 to 3, 2.5 | 6 | 0.92 | none | 0 |

[code: `oregon-white-oak.values:502-609`; `european-beech.values:617-676`; `norway-spruce.values` ages 10 to 12 (lines about 397-484); `silver-birch.values:757-784`.] None of these ages states an abortion, a relay or an internode spread.

### The two measures

**Maximal reach, as `closed_form/kept/bound.rs` computes it** [code: `Kept::bound`, lines 39-104]:

```rust
let most: f64 = state.zones.iter().map(|z| quantile(&Count::pmf(z.nodes))).sum();
let longest = state.internode * (1.0 + state.form.internode_spread);
let path = most * longest * step.size;
let mut best = path + rest.map_or(0.0, |r| r.reach);
...
for lateral in &laterals[k] { for (j, _) in ... {
    let child = self.reach_bound(j, m - step.i, Carried::default(), None);
    best = best.max(path + step.size * child);
}}
...
if let Some((at, pa, onward, _)) = end.continuation {
    best = best.max(path + self.reach_bound(pa, m - at, onward, None));
}
```

- A uniform node law's quantile is its max. With whole units and given years enough, the recursion gives `lifespan × max nodes × internode + the largest child's reach`, the child standing on the last unit's tip [derived].
- It is per unit of base scale. A real lateral's bound is this times `bearer node scale × vigour` [code: `closed_form/kept/tests/bound.rs:31-33`].
- The module is test-only: `#[cfg(test)] pub(crate) mod kept;` [code: `closed_form.rs:20-21`].

**Expected reach, as fn-339 took it**: the expected length of the bud's own axis plus the expected length of one child axis of its lateral kind, in full light, at base scale 1, viability the only stop. Every unit draws its survival, the first included [code: `grow/advance.rs`, `advance`, lines 100-104, 143-157], so the expected units of a lifespan n are `v + v² + ... + vⁿ`. This form reproduces fn-339's three figures to the digit (oak twig 0.313, oak short 0.059, spruce shoot 0.068), so it is the measure the survey quoted [derived].

### The arithmetic

**Oak short (14).** Units: 0.9 + 0.81 + 0.729 = 2.439. Expected: 2.439 × 4 × 0.006 = **0.0585 m**. Most: 3 × 5 × 0.006 = **0.090 m**.

**Oak twig (13).** Units: 0.98 + 0.9604 + 0.9412 + 0.9224 + 0.9039 = 4.7079. Own: 4.7079 × 4.5 × 0.012 = 0.2542 m. With a short shoot: 0.2542 + 0.0585 = **0.313 m**. Most: 5 × 5 × 0.012 = 0.300, plus 0.090 = **0.390 m**.

**Oak shoot (12).** Units: 0.93 + 0.8649 + 0.8044 + 0.7481 = 3.3473. Own: 3.3473 × 4.5 × 0.015 = 0.2259 m. With a short shoot: **0.284 m**. Most: 4 × 5 × 0.015 = 0.300, plus 0.090 = **0.390 m**.

**Beech short (14).** Units: 0.9 × (1 − 0.9¹²) / 0.1 = 9 × (1 − 0.28243) = 6.4581. Expected: 6.4581 × 4 × 0.006 = **0.155 m**. Most: 12 × 5 × 0.006 = **0.360 m**.

**Beech shoot (12).** Units: 0.95 + 0.9025 + 0.8574 + 0.8145 + 0.7738 = 4.2982. Own: 4.2982 × 3.5 × 0.025 = 0.3761 m. With a short shoot: 0.3761 + 0.1550 = **0.531 m**. Most: 5 × 4 × 0.025 = 0.500, plus 0.360 = **0.860 m**.

- fn-339's figure for this kind was 0.450 m expected and 0.62 m potential. Both follow from a short shoot of lifespan 4: 0.3761 + (0.9 + 0.81 + 0.729 + 0.6561) × 4 × 0.006 = 0.3761 + 0.0743 = 0.450, and 0.500 + 4 × 5 × 0.006 = 0.62 [derived]. The lifespan has since become 12 (log, 12 Nov 04:02, per the survey).

**Spruce shoot (12).** Units: 0.9 + 0.81 = 1.71. Expected: 1.71 × 2 × 0.02 = **0.0684 m**. Most: 2 × 2 × 0.02 = **0.080 m**.

**Spruce spur (11).** Units: 0.99 × (1 − 0.99⁴⁰) / 0.01 = 99 × (1 − 0.66897) = 32.772. Own: 32.772 × 1 × 0.006 = 0.1966 m. With a shoot: **0.265 m**. Most: 40 × 0.006 = 0.240, plus 0.080 = **0.320 m**.

**Spruce branchlet (10).** Units: 99 × (1 − 0.99¹⁰) = 99 × 0.095618 = 9.4662. Own: 9.4662 × 3 × 0.016 = **0.454 m** before anything it bears. It reaches its end with chance 0.9044 and then continues into a spur: 0.454 + 0.9044 × 0.1966 = 0.632 m, and with the spur's shoot about **0.70 m**. Most: 10 × 3 × 0.016 = 0.480, plus the spur's 0.320 = **0.800 m**.

**Birch short (14).** Units: 0.92 × (1 − 0.92⁶) / 0.08 = 11.5 × (1 − 0.60636) = 4.5269. Expected: 4.5269 × 2.5 × 0.003 = **0.0340 m**. Most: 6 × 3 × 0.003 = **0.054 m**.

### Against the 0.5 m cell

| Species, age | Expected m | Most m | Expected under 0.5 | Most under 0.5 |
|---|---|---|---|---|
| Oak 14 short | 0.059 | 0.090 | yes | yes |
| Oak 13 twig | 0.313 | 0.390 | yes | yes |
| Oak 12 shoot | 0.284 | 0.390 | yes | yes |
| Beech 14 short | 0.155 | 0.360 | yes | yes |
| Beech 12 shoot | 0.531 | 0.860 | **no** | **no** |
| Spruce 12 shoot | 0.068 | 0.080 | yes | yes |
| Spruce 11 spur | 0.265 | 0.320 | yes | yes |
| Spruce 10 branchlet | 0.70 (own axis alone 0.454) | 0.800 | **no** | **no** |
| Birch 14 short | 0.034 | 0.054 | yes | yes |

Four limits on this table [code and derived]:

- **Under 0.5 m is not inside one cell.** The lattice is anchored at the world's origin and light is read trilinearly from eight corners (survey, section 3). A subtree of any reach whose bearing node stands near a cell face crosses it. The table says only that the reach is shorter than a cell's side, which is the test fn-339 used.
- **All figures are at base scale 1 and unit size 1.** A real lateral is shorter by its base scale. But a unit's laid length is `scale × size × stretch` [code: `grow/draw.rs`, `draw_bent`, lines 368-379], and the size is `(v/p)^ψ × Σp / Σp(v/p)^ψ` against the mean of its PA's growing buds [code: `allocation.rs`, `size_of`, lines 176-183], which can exceed 1 where `shade_size` is above 0. The oak's ages 12 to 14 state `shadeSize = 1.0` (lines 518, 558, 598); the beech's, spruce's and birch's fine ages state none. The bound's own test grows under `Light::NEUTRAL` [code: `closed_form/kept/tests.rs`, `tree`, lines 74-81], where every size is 1. Whether an oak twig in light is ever laid past 0.390 m times its base is [unknown: not measured].
- **Expected is in full light.** Shade lowers survival (`ln survival = ln viability × light^-φ`, `grow/advance.rs:67-82`) and the balance sheds, so the lit expectation is shorter; the census measured realised faint subtrees at 0.04 to 0.36 of the unlit expectation.
- **The bound is a straight path.** "A straight path is the farthest wood can stand from its base, whatever bends it" [code: `closed_form/kept/bound.rs:11-12`].

### How fn-364's s changes the reach

[code: `grow/cycle/passes.rs`, `surplus`, lines 167-176; `allot_one`, lines 143-148; `grow/draw.rs`, `draw_bent`, lines 368-383]

```rust
pub(super) fn surplus(axes: &[Axis<Growing>], i: usize, inverse_height: f32, light: f32) -> f32 {
    let charge = charged(1.0, axes, i, inverse_height);
    if charge == 0.0 { 1.0 } else if light <= 0.0 { 0.0 } else { (1.0 - charge / light).max(0.0) }
}
```

- Each year a living tip's unit is laid at `scale × size × stretch` of its internode, the stretch s read that year from the height of the axis's own last laid node above the collar and the light at its own tip. The unit's nodes, leaf, deposit and presence are unchanged.
- s lies in 0 to 1, so the **maximal reach does not move**: s is 1 at the collar, or for a species with no potential height. `bound.rs` does not read s [code: it reads `state.internode` and `step.size` only].
- The **expected reach is multiplied unit by unit by that year's s** [derived]. At full light s is `1 − (h/H)²`: 0.75 at half the potential height, 0.36 at 0.8 of it, 0 at it. At light L it is `1 − (h/H)²/L`, so 0 from `h = H√L` on. The potential heights are 34.8 m for the oak (`inverseHeight = 0.02874`), 40 m for the beech (0.025) and 61 m for the spruce (0.01639).
- A leafy shoot's height is within its reach of its bearer's node, so its s is close to its bearer's at the same light [derived]; the two read different tips and may read different light.
- The settled tree carries the stretch per phytomer, and the drawing keeps the allotted length beside the laid one for leaf stations ("a stretch moves stations, never makes or removes one", fn-364 spec; `pipeline/space/join.rs`, `Wood::of`, lines 56-60). So s shortens the wood's reach, not the count of leaves it bears.
- Whether `no_wood_reaches_past_the_extent_bound` still holds at this head was not run [unknown]. By the arithmetic it can only gain margin from s.

## 4. What one leafy lateral axis holds, and what each pass reads of it

"Closed kind" here means a kind the table closes (question 3). The axis has three states: growing (a living apex), stopped but held (its apex ended, its subtree closed in the working set, not yet retired), and retired.

### Bytes

Sizes are 64-bit. [code] where a test asserts the size; [derived] where summed from the fields.

**Per axis, fixed**

| Store | Bytes | Source |
|---|---|---|
| `axes[i]`, `Axis<Growing>` | 232 | [code: `grow/tests.rs`, `an_axis_and_its_draws_stay_compact`, asserts `Axis` at 232; a `Vec<P>` is the same width for either phytomer] |
| `draws[i]`, `Draws` | 144 | [code: the same test] |
| `pencils[i]`, `Pencil` | about 216 | [derived: `Layer` is five `[f32; 3]`, four `f32` and a `Vec` (104); nine 8-byte scalars; a `Vec`; `stretch`, `root`, `trunk`. No test asserts it. The footprint read 272 a held axis at fn-357, capacity included] |
| `horizon[i]`, `units[i]`, `roots[i]` | 8 + 8 + 8 | [code: `grow.rs`, `Grower`, lines 249-264] |
| `successor[i]`, an empty `Vec` | 24 | [code: the same] |
| `working`: `links` 24, `pa` 8, `by_node` 24, `open` 1, `at` 4, `light`, `cost`, `sub`, `incoming`, `bud` 20, `living` 1 | 82 | [derived from `grow/cycle/working.rs`, `Working`, lines 25-45] |
| `retire.fate` 12, `retire.woken` 4 | 16 | [derived from `grow/cycle/retire.rs`, `Fate`, `Retirement`] |
| its entry in its bearer's `by_node` list | 4 | [code: `Working::extend`, lines 133-138] |
| **Sum** | **about 742** | [derived] |

While its apex lives it also holds an `Apex` in `live` (16 bytes) and a place in `working.list` (8).

**Per growth unit:** `draws.units` 32 (`Unit`: four `f64`) and `draws.shade` 8 (`Shade`: two `f32`), **40** [code: `presence.rs`, lines 85-112].

**Per phytomer:** `Growing` **72** [code: `grow/growing_tests.rs:20`], `draws.nodes` 8, `pencil.path` 4: **84** [derived].

**Whole axes, by this arithmetic** [derived]:

- An oak short shoot at full life, 3 units of 4 nodes: 742 + 3 × 40 + 12 × 84 = **about 1,870 bytes**, in the three shared arrays plus five heap blocks of its own (phytomers, units, nodes, shade, path).
- A beech short shoot at 12 years of 4 nodes: 742 + 12 × 40 + 48 × 84 = **about 5,250 bytes**.
- The census measured more a held axis (1,050 bytes for the per-axis stores alone on the oak at 120), because vectors hold capacity past their length and it was read before fn-357 [record: `PHASE5-CENSUS.md`, M2].

**Once its apex stops.** Nothing is freed. It stays whole until its subtree is evaluated and its fade at the age grown reaches none [code: `grow/cycle/retire.rs`, `evaluate`, lines 280-286].

**Once retired** [code: `retire.rs`, `let_go`, lines 232-249]: `phytomers`, `draws.nodes`, `draws.units` and `draws.shade` are freed at once. The fixed records, and `pencil.path`, stay until compaction, which runs once the loose axes reach the larger of a quarter of the axes and 4,096 [code: `grow/cycle/compact.rs`, lines 75-80].

**After compaction** [code: `compact.rs`, `stands`, lines 145-179; `shed.rs`, `Stand`, lines 88-95]: a retired root keeps a `Stand` of **24 bytes** (bearer, node, birth, vigour, living), until its bearer retires in turn. An axis inside a retired subtree keeps nothing.

### What each yearly pass reads of it

The order in a year is compact, windows, open, retire, balance, allot, grow, wake, layout with its deposit, light read, wood [code: `grow.rs`, `year` and `step`, lines 336-412]. "Own geometry" means the axis's own tips, frames or layer; "bearer's place" means only which axis and node it stands on.

| Pass | Fields read of a growing axis | Needs its own geometry? | Where |
|---|---|---|---|
| Compact | its fate; every per-axis store is moved down | no | `compact.rs`, `rename_axes`, 97-139 |
| Open | its index in `live`; `links[i].parent` up to the root | no: bearer's place only | `passes.rs`, `open`, 14-23; `working.rs`, `open_from`, 155-196 |
| Retire | nothing while it is open. Once closed: a copy of its `Axis` (origin, pa, birth, vigour, apex_end and the rest), its phytomers' `cycle`, its `Draws`; `base`, `heading`, `side` are copied and not read | no | `retire.rs`, `copied`, 298-368; [record: fn-358 `READING.md`, "A closed axis's phytomers are read only for their cycle"] |
| Balance | `pencil.base_scale`, `running`, `light`; **the z of its own last laid node** less the seed's base z; its PA's `inverse_height`; `working.light`, `cost`, `living`; its `lineage` for the `SHED` draw; `pencil.memory`; its window. Writes `memory`, `kept`, or ends its apex | **yes: its own tip's height** (fn-363), one coordinate | `passes.rs`, `balance_one`, 52-117; `charged`, 157-160 |
| Allot | `pencil.base_scale`, `running`, `kept`, `light`; its PA's `shade_size` and `apical_control`; its place among its bearer's children (`by_node`, by node). Writes `pencil.size`. Then its stretch: **its own tip's z** and `light` again | the size: no, bearer's place and the tree's sums. **The stretch: yes** | `passes.rs`, `allot_one`, 124-150; `working.rs`, `split`, `sizes` |
| Grow | `lineage`, `pa`, `phytomers.len()`; `draws` (units, rest, ended, sleep, below, carried, woke); `apex.spent`; `pencil.light` (its survival, and its laterals' turn), `kept`, `size`, `stretch`; `horizon`, `roots`. Pushes phytomers, `draws.nodes`, a `Unit`, a `Shade` | no direct read. The light it reads was read at its own tip. `rests` reads its layer, only under a ground hazard | `advance.rs`, 17-228; `unit.rs`, 16-137; `shoot.rs`, `light`, `shade`, `kept`, `rests`, 241-270 |
| Wake | nothing, unless it bears a sleeping bud or carries one's bearer on. The closed kinds state no `dormant` row | no | `wake.rs`, `wake`, 18-58 |
| Layout | for the units it grew: `draws.units[k]` (size, survive, persist), `draws.nodes[j]`, `draws.shade(k)`; each new phytomer's `cycle` and `key`; `pencil.layer`, `base_scale`, `running`, `rank`, `path`; `birth`, `pa`, `draws.sleep`. Writes each phytomer's scale, grown, size, rank, tip, heading, side. In its first year: its frame from **its bearer's node's tip, heading, side, rank and unsized scale**, the bearer's phase, and the bearer's pencil (`layer.ends()`, `path_at(node)`) | **yes: its own layer; and its bearer's node's frame** | `draw.rs`, `pencil`, 235-311; `draw_bent`, 332-392; `geometry.rs`, `lateral`, 300-345 |
| Light deposit | each phytomer laid this year: **its tip**, and `leaf = leaf_area × scale × size × years kept` (its `cycle`). Where any age keeps leaf past a year (the spruce), every open axis's older phytomers still in leaf as well | **yes: each leaf goes in at its own phytomer's tip** | `deposit.rs`, `deposit`, 12-44; `girth.rs`, `leaf`, 50-53 |
| Light read | **its own tip**: its last phytomer's, or its base | **yes** | `sketch.rs`, `shade`, 103-122; `tip`, 240-242 |
| Wood | `origin` (bearer and node), `pa` (sapwood, rings, primary, leaf rows); each phytomer's `cycle`, `scale`, `size`, `section`, `sapwood`; its children's sums at their nodes. Hands its leaf sum to its bearer at its node | **no: bearer's place only** | `girth.rs`, `lay`, 81-126 |
| Settle | `live` and `apex.spent`; `draws` (birth, units, nodes, shade, sleep); each phytomer's `cycle`, `key`, `section`, `sapwood`. It rewrites scale, grown, size, stretch and rank from the draws, then every tip and frame in the final lay | not the grower's: the final lay rewrites it | `settle.rs`; `presence.rs`, `assign_one`, 235-263; [record: fn-358 `READING.md`, "a laid tree reads only cycle, key, section and sapwood from the grower"] |

**Once its apex has stopped and its subtree is closed** [code and record]:

- The balance, the allotment and the light read skip it: they visit living apexes and the open list [code: `passes.rs:48, 130`; `working.rs`, `totals`, 209-213: "A closed child adds nothing"].
- The wood pass returns 0 for it at once: `if closed(i) { return 0.0; }` [code: `girth.rs:94-96`].
- The deposit skips it for older leaf: `if closed(i) { continue; }` [code: `deposit.rs:31-33`].
- The layout reads it only if something new is framed on it, and nothing is born on a closed subtree [record: fn-357 `READING.md`].
- Its pencil is read by nothing [record: fn-358 `READING.md`, "Pencils of a closed axis are read by nothing"; checked here against `passes.rs`, `sketch.rs`, `draw.rs` at this head, which read pencils of living apexes, of axes that grew, and of a new axis's bearer]. fn-364 added one pencil field, `stretch`, written and read for living apexes only [code: `passes.rs:143-148`; `shoot.rs`, `shade`, 259-264].
- What is still read of it: retirement's evaluation (its `cycle`s and its draws, once) and the settle (cycle, key, section, sapwood, draws).

**What reads its geometry, in one line** [derived from the table]: the balance's charge (tip height), the stretch (tip height), the layout (its layer, and its bearer's node frame at birth), the deposit (each phytomer's tip) and the light read (its tip). The open pass, the allotment's size, the wood pass, retirement and the settle's presences read no position of it.

## 5. What the pipeline after growth reads of it, and what a rebuild from its key would need

### Readers after growth

**The conversion** reads the settled axis directly [code: `pipeline/space.rs`, `unfolded`, lines 271-598; `pipeline/space/join.rs`, lines 56-60, 85-96]:

| Read | Of | For |
|---|---|---|
| `origin` | axis | the node it starts from: `drawn[parent].get(node)` (305-325) |
| `pa` | axis | its row: internode, spread, flowering, cluster, retention; whether it is trunk (329-358) |
| `lineage` | axis | its cluster's whole leaf count, `telperion_space::cluster(cluster.0, axis.lineage)` (361); each node's key, `node_key(axis.lineage, place)` (`join.rs:91`) |
| `radius` | phytomer | drawn or not: `take_while(\|(_, p)\| p.radius >= least)`, the 0.2 mm floor (371-375, 399) |
| `cycle` | phytomer | leaf retention and the year's unfolding (381, 494) |
| `key` | phytomer | the internode's spread factor (385); the node's identity (`join.rs:90`) |
| `length()` and `scale` | phytomer | wood as laid, `internode × p.length() × factor`, and as allotted, `internode × p.scale × factor` (`join.rs:56-60`) |
| `stretch` | phytomer | whether the tree keeps allotted wood beside laid wood (295) |
| `tip` | phytomer | the node's position (462, `join.rs:88`) |
| `heading`, `side` | phytomer | the node's frame (484-485) |

An axis of an age that states a cluster is marked a short shoot: each node it draws is marked in `clustered`, and its last drawn node keeps one `Cluster { node, leaves, spread }` (515-517, 548-554).

**Everything after the conversion reads the pipeline's tree, not the axis.** The tree carries `nodes`, `frames`, `keys`, `grown`, `bearing`, `handed`, `followed`, `blooms`, `clusters`, `clustered`, `leafing` (580-596).

- **The curve:** every run of the sweep as the points its rings stand on, read from the nodes' positions, radii and frames [code: `pipeline/surface/curve.rs`, module doc, lines 1-8].
- **The foliage plan:** "which wood bears leaves, how many stations each bearing segment carries ... all read off the solved tree before any leaf is placed" [code: `pipeline/foliage/plan.rs`, module doc, lines 1-6]. A short shoot's leaves are seated at its tip from `cluster_key(tip)`, the tip node's key under `CLUSTER` [code: `tree/cluster.rs`, `cluster_key`; `lineage/keys.rs:85`].
- **The field:** "a read-out of the drawn tree to a grid": each solved node's sweep and each leaf-bearing record of the plan [code: `pipeline/field.rs`, module doc, lines 1-9].
- **The CPU mesh:** built from the curve's rings (survey, section 3; not re-read here).

The bodies of the curve, plan, field and mesh were not read past their module docs for this answer; the statement that none of them reaches back to the engine's axes rests on the conversion handing them only the `Tree` [code: `pipeline/space.rs`, `grow`, lines 54-70] and is otherwise [unknown].

### What its phytomers are drawn from

The axis's lineage is its bud place's key carried onto its kind: `place.onto(keyed)`, the place being `unit.child(ZONE + z).child(drawn).child(slot)` under its bearer's unit key `Key(bearer lineage).child(cycle)` [code: `grow/unit.rs:30, 43-46, 78`; `grow/advance.rs:24`; `lineage.rs`, `Key::onto`, lines 46-48; `lineage/keys.rs`: `ZONE` 28, `PLACE` 32, `CONTINUATION` 112].

Each of its growth units is keyed `U = Key(lineage).child(cycle)` [code: `grow/advance.rs:24`]. The draws under it, by the registry [code: `lineage/keys.rs`]:

| Draw | Key | Decides | Reads beyond key, species and cycle |
|---|---|---|---|
| Survival | `U.child(VIABILITY)` (keys.rs:23) | whether the unit grows | **the light at its own tip** where `shade_hazard` is above 0 (`advance.rs:70-82`); the ground under a ground hazard. All four species' closed ages state a shade hazard |
| Failure's relay | `U.child(RELAY)` (39) | a relay of a failed unit | nothing; `relay_failed` is 0 on the closed ages |
| Abortion | `U.child(ABORTION)` (24) | whether the apex aborts after the unit | nothing; `abortion` is 0 on the closed ages |
| Node count | `U.child(ZONE + z)` (28) | each zone's nodes | nothing for the count; each node's presence reads the windows, which read the species and the year (`unit.rs:30-37`) |
| A cut unit's nodes | `zone.child(j).child(LAID)` (68) | nodes of a unit a fractional lifespan cuts | nothing; the closed ages' lifespans are whole |
| Bud places | `zone.child(i).child(slot)`, `PLACE` (32); `PLACED` (65) | which lateral each place bears | nothing |
| A lateral's turn | `place.child(TURN)` (110) | its kind under shade | **the light at this axis's tip**; none is drawn inside a closed kind today (question 1) |
| Yearly shedding | `Key(lineage).child(SHED).child(cycle)` (35) | whether its balance sheds it | **its subtree's light and charge, its remembered balance, and the walk's verdicts** (`passes.rs:79-100`) |
| Age's end | `Key(lineage).child(END)`, `MOVE` (50, 26) | moving on or relaying | nothing; continuation and relay are 0 on all but the spruce's branchlet |
| Roll, insertion, tropism, dominance | under its lineage (41, 57, 59, 43) | its frame and its share | nothing |
| Internode spread, wander | under each node's key (52, 62) | each internode's length and bend | nothing |
| Cluster, organs, retention | under the tip node's key (85, 107, 90) | its leaves, in the drawing | nothing |

So, of a closed-kind axis's phytomers:

- **From key, species and cycle alone** [code]: its node counts, its bud places, its angles and wander, its cluster's leaves.
- **Not from them** [code]:
  - Which units exist: each year's survival reads the light at its own tip, and its shedding reads its balance, its memory and the tree's walk.
  - Each unit's size: the allotment splits the whole tree's vigour and normalises over every growing bud of its PA [`working.rs`, `split`, `sizes`].
  - Each unit's stretch: its own tip's height and light [`passes.rs`, `surplus`].
  - Each unit's presence: its survival's lead times `kept`, the balance walk's share [`advance.rs:159-174`].
  - Its base scale: its bearer's node's unsized scale times its birth presences times its dominance share [`draw.rs`, `pencil`, 267-290]. The birth presence of a bud of a turning kind reads its bearer's light when it was formed [`unit.rs:113-118`].
  - Its kind, for a beech short shoot born of a turned bud: the key is the drawn kind's (question 1).
  - Its frame: its bearer's node's tip, heading, side and rank, and the bearer's phase [`geometry.rs`, `lateral`].
  - Its position in the settled tree: the final lay, with sag by load and radius and erection by the axis's age.
  - Its section and sapwood: laid year by year from the leaf above, through a clamp, so they read its leaf's history [`girth.rs`, `ring`, `lay`].
- **Under neutral light** every light read returns 1 and no layout is grown, so the survival, size and stretch reduce to key, species and cycle [derived from `shoot.rs`, `light`, `shade`, `kept`, each defaulting to whole where there is no pencil]. The product never grows that way: the one cycle always grows a rough layout [`paused.rs:102`].

**Answer to the question as asked.** The draws that rebuild the axis are `VIABILITY`, `ZONE + z`, the place keys with `PLACED`, `SHED`, and for the drawing `ROLL`, `INSERTION`, `TROPISM`, `DOMINANCE`, `INTERNODE`, `WANDER` and `CLUSTER`. Two of them, `VIABILITY` and `SHED`, are tested against thresholds that depend on more than the key, the species, the cycle and the bearer's node: the light at the axis's own tip each year, and the tree's balance. The unit's size and stretch, which set its leaf and its length, are not draws at all and depend on the same.

## 6. Where the grower's time goes by pass, from records

### What is on record

**fn-327, 3 Nov** [record: `.flow/evidence/fn-327-the-cycle-costs-less-per-bud-byte-for/RESULT.md`, R1]. One thread, native, seed 1, the preset's age, second run, milliseconds:

| Preset, fidelity | Total | Balance | Units | Sketch | Shade | Allot |
|---|---|---|---|---|---|---|
| Oak, 0 | 1,101 | 288 | 266 | 154 | 69 | 28 |
| Oak, 1 | 165 | 41 | 33 | 25 | 15 | 8 |
| Spruce, 0 | 5,204 | 1,324 | 971 | 909 | 217 | 143 |
| Spruce, 1 | 789 | 226 | 179 | 116 | 36 | 24 |
| Birch, 0 | 802 | 175 | 204 | 113 | 65 | 15 |
| Birch, 1 | 165 | 35 | 33 | 21 | 24 | 5 |

- "No pass or operation dominates the cycle."
- "The pass called Balance is mostly retirement": on the oak at fidelity 0, of 288 ms, judging closed subtrees took 116, freezing girth 71, applying verdicts 30, opening the working set 20, the totals 23.
- Inside Units and Sketch on the same oak: laying phytomers and sprouting 134, drawing wood 100, deciding 50, node draws and sort 44, zone setup 39, framing new axes 30, gathering leaves 26.
- This predates fn-349 (the corrected balance), fn-354 (the wood pass, no girth freeze), fn-355, fn-357, fn-358, fn-363 and fn-364. It is the newest full split found.

**The wood pass** [record: goal log, 11 Nov 21:05 and 22:00]: 13, 18 and 19% of the grow at 40, 120 and 300 as first built; 8, 13 and 13% once it skipped closed wood (one thread, load 21 to 37).

**Whole-grow seconds** [record: `PHASE5-CENSUS.md`, M2; goal log, 12 Nov 14:24]: grow against mesh, oak 80 26.8 and 2.0 s; oak 120 97.2 and 2.8; beech 40 0.7 and 0.2; spruce 40 3.9 and 4.7. Presets on one thread, seed 1001: oak 40 1.58 s, oak 120 42.8 s, spruce 11.6 s.

**A split by pass on today's grower, for any tree:** [unknown: none found in the goal log or under `.flow/evidence` for a build after fn-354].

### How to obtain one without new code

The grower reports every pass as it ends, and the one cycle sums the seconds between reports by the stage that ended them [code: `grow/stage.rs`, `Stage`, `Pass`; `grow/cycle.rs`, `tick`, lines 66-73; `grow/cycle/paused.rs`, `cycles`, 202-214, and `finish`, 271-278]. `grow_census` returns them in `Census::phases` [code: `grow/cycle.rs:49-53`]. Two entries print them:

- **The example** [code: `crates/telperion-core/examples/early.rs`, lines 1-8, 100-108]:

  ```
  cargo run --profile ci -p telperion-core --example early -- three <preset> <age|default> <seed> cycle
  ```

  It prints `phases (s): ...`, every pass's seconds, largest first, for one preset, age and seed in its own process.
- **The ignored test** `pass_costs` [code: `pipeline/cycle_cost_tests.rs`, lines 78-110]: the oak, spruce and birch at their preset age, seed 1, at fidelity 0 and 1, a warm run first, the top 14 passes in milliseconds.

How to read the names [code: `grow.rs`, `step` and `year`; `sketch.rs`, `shade`]: a pass's seconds run from the report before it, so `Pass(Balance)` holds compaction, any window rebuild, the open pass, retirement and the balance; `Pass(Sketch)` holds the layout and the deposit; `Pass(ShadeField)` holds the lattice's build and the reads at the tips; `Pass(Wood)` is `girth::lay`; `Homes`, `Units`, `Merge` are the growth; `Mark` to `SagPlace` are the settle. The census itself adds work each cycle (`Grower::count`, `census.lattice`), not measured apart [unknown]. Per AGENTS.md, a run goes through `telperion-heavy`; none was run here.

## 7. Who passes a non-zero `fidelity`, and who reads the `/space/coarse` rows

### Callers with a non-zero `fidelity`

- **`src/`:** none. The word appears in two generated files only: the parameter table, `src/browser/parameters.generated.ts:12, 840`, and every preset's value, `"fidelity": 0`, in `src/browser/presets.generated.ts` [code: grep over `src/`].
- **`harness/`:** no file names it [code: grep]. The panel offers it as a control all the same: it shows every row whose reach is mature, and the row is `{ path: "/fidelity", ..., low: 0, high: 1000000, reach: "mature", group: "advanced" }`, with no dial window [code: `harness/rows.ts`, `shownRows`, lines 44-46; `harness/panel.ts`, `controls`, 31-38; `parameters.generated.ts:840`]. So a person can type a non-zero value in the harness's advanced group; no code sets one.
- **`tests/browser/`, `scripts/`, `catalogue/`, `index.html`:** none [code: grep].
- **Rust, tests only:** `pipeline/cycle_cost_tests.rs` (`family(id, age, seed, fidelity)` at 0 and 1; the digest pin `every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial`) and `pipeline/fidelity_tests.rs` [code]. `suite/geometry_benchmark.rs:382` asserts the published default is 0; `suite/sweep.rs:85` holds `/fidelity` out of the sweep.
- **The default** is 0 [code: `family.rs:96`].
- **The website** as something apart from `src/` and `harness/`: no such directory is in this repository [code: `ls` of the checkout]. The plan's "coarsest oak 164 ms in Wasm" is a measurement; which caller it was taken through was not found [unknown].

### Where the rows are

`/space/coarse/{first, factor, leaf, bud, hazard, shedding}` in three presets, unchanged at this head [code: `oregon-white-oak.values:617-622`, `norway-spruce.values:493-498`, `silver-birch.values:792-797`], and mirrored as `"coarse"` objects in `src/browser/presets.generated.ts:2359, 3211, 4996`.

### Every reader

| Reader | What it does | Where |
|---|---|---|
| `GrowInput::of` | coarsens the species where `family.fidelity > 0.0 && space.coarse != Coarsening::NONE` | `pipeline/input.rs:65-68` |
| `Species::coarsened` | applies the rows at `t = (fidelity / COARSEST).clamp(0, 1)` | `crates/telperion-space/src/species/coarse.rs:72-73` |
| `Family` validation | validates the rows, and the species coarsened to the coarsest | `family.rs:139-143` |
| Blend | blends two species' rows | `blend.rs:172-194` |
| Preset parser and writer | reads `["coarse", name]` rows; writes those that differ from `Coarsening::NONE` | `presets/space.rs:128, 146, 158, 271-284` |
| Admission | refuses a `fidelity` off its range, read by the conversion and the growth entry | `pipeline/input.rs:81-87`; `pipeline/space.rs:108`; `pipeline/space/growth.rs:36` |
| Tests | `pipeline/fidelity_tests.rs`; `pipeline/cycle_cost_tests.rs`; the unit tests in `species/coarse.rs` | as named |
| Generated mirrors and docs | `presets.generated.ts`; `parameters.generated.ts`; `docs/parameters.md:46-53`; the catalogue's group for `/fidelity` (`catalogue/group.rs:52`) | as named |

`telperion_space` exports `Coarsening` and `COARSEST` [code: `crates/telperion-space/src/lib.rs:72`]. The render crate's uses of "coarsen" are the shadow map's and the wood's view budget, unrelated to these rows [code: grep].

## What was not read or run

- Nothing was run: no test, no example, no measurement. Every "holds" above is from reading.
- The bodies of the curve, the foliage plan, the field and the CPU mesh, past their module docs.
- `contract/shed.rs`'s `walk`, beyond its call in `balance_one`; `presence::assign_living` and `shed::subtree_living`, taken from the fn-357 and fn-358 readings.
- The palm, the ordinary tree and the Two Trees for question 3; the Two Trees' `shadeShort` rows were checked for question 1 only.
- `Pencil`'s size is summed from its fields, not asserted by any test.
- The second half of `PHASE5-CENSUS.md` (the beech's and spruce's full tables, lines 595 on).
