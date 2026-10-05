# fn-206 R0: one reference axis, every setting by degree (proposal)

A dispatched worker wrote this for the host's review before R1. It proposes; nothing here is decided or built. Every code claim is marked **checked** with file:line, against this worktree's HEAD (branch fn-196, which holds the engine with fn-205), or **unknown**. The oak was read from `fn-195-tree-space-d-the-oak-as-a-point:crates/telperion-space/src/oak.rs`, not built here, because that branch has no `form.secondary`.

## 0. A correction to the spec

The spec gives the beech 6 ages, the spruce 6, the oak 10 and the palm 1. Checked by dumping each `Species`: the beech has **9** (`beech.rs:12-20`, `:268`), the spruce **9** (`spruce.rs:13-21`), the oak **10** (`oak.rs:15-24`, `:332-335`) and the palm **1** (`palm.rs:63`).

## 1. Every setting: its continuous form, neutral value and closed-form effect

The closed form is `closed_form.rs`. It builds expected counts per PA from each unit's survival, its zones' mean node counts, buds and lateral probabilities (`closed_form.rs:100-125`, checked). The presence windows use the expected log-lengths it returns (`presence.rs:32-50`, `closed_form.rs:289-301`, checked).

### `Species`

| Setting | Today | Continuous form | Neutral | Closed form |
|---|---|---|---|---|
| `states` (`species.rs:37`) | a list of 1 to 64 (`species.rs:15`, `MAX_STATES`) | a fixed list of N on the shared chain (section 2) | — | per-PA tables of length N; unused ages contribute nothing |

### `PaState` (`species.rs:42-96`)

| Setting | Today | Continuous form | Neutral (today's value) | Closed form |
|---|---|---|---|---|
| `lifespan` (`:44`) | u32 ≥ 1; 0 refused (`species.rs:232-236`, checked) | real L ≥ 0: ⌊L⌋ whole units, and one more unit in this age made by a draw against frac(L), grown in by its lead (the presence of B, `presence.rs`). At L = 0 the age is passed through: the apex goes on to the next age without growing in this one | the integer | units per age become ⌊L⌋ + frac(L) in expectation; `units_left` and `spent_key` (`closed_form.rs:39-45`, checked) take the fractional unit's weight |
| `next` (`:46`) | `Option<usize>`, any later age (`species.rs:238-244`, checked) | always the next age on the chain, plus a new share `continuation` c ∈ [0, 1]: the chance an apex that has spent its lifespan moves on rather than stops, made by a draw grown in by its lead | c = 1 where `next` is `Some`, 0 where `None` (the chain's last age is always 0) | the move-on event is weighted by c and the stop by 1 − c (the same events `grow.rs:249-258` makes, checked) |
| `viability` (`:48`) | share | already continuous | — | unchanged |
| `zones` (`:50`) | 1 or more zones (`species.rs:250-253`, checked) | a fixed Z = 4 zones per age, base to top; a zone a species does not use has no nodes. Today's zones keep their indices, **unused ones appended after them**, because the zone key is `ZONE + z` (`lineage.rs:11-12`, `grow.rs:323`, checked) | today's zones, the rest at 0 nodes | a zone of 0 mean nodes adds no `Event::Nodes` or `Event::Bud` weight (`closed_form.rs:107-113`, checked) |
| `shedding` (`:53`) | `Option<u32>`, a dead lateral kept that many cycles (`shed.rs:28-30`, checked: `idle > d`) | real d ∈ [0, ∞]: an axis idle past d is faded out by the fraction past it rather than dropped | `None` = ∞, `Some(n)` = n | unknown whether the closed form reads it; `grep` finds no `shedding` in `closed_form.rs` (checked), so likely none |
| `internode` (`:55`), `insertion` (`:57`), `divergence` (`:59`) | reals | already continuous | — | lengths scale `internode`; angles are geometry only |
| `abortion` (`:62`), `abortion_rise` (`:67`), `relay` (`:71`), `relay_at` (`:76`), `epitony` (`:79`), `erection` (`:83`), `readiness` (`:86`), `rhythm` (`:90`), `straightening` (`:93`) | reals and shares | already continuous, walked by B (`tests/walk/mod.rs:186-300`, checked) | as today | abortion and relay enter the stop events; readiness and rhythm scale the lateral tables (`species.rs:355`, `PaState::laterals`, checked) |
| `form` (`:96`) | `Form` | see below | | |

### `Form` (`species.rs:103-149`)

All are reals already, walked by B or by fn-205 (`tests/walk/mod.rs`, checked).

| Setting | Neutral |
|---|---|
| `tropism` (`:105`) | 0 |
| `elevation` (`:108`) | — (a target) |
| `wander` (`:111`) | 0 |
| `plane` (`:114`) | 0 |
| `pipe` (`:117`) | — (a size) |
| `exponent` (`:122`) | 2 |
| `ripening` (`:126`) | 0 |
| `dominance` (`:131`) | 0 |
| `roll` (`:135`) | 0 |
| `sag` (`:141`) | 0 |
| `secondary` (`:148`) | 1 |

None enters the closed form's counts; girth and geometry are placed after the counts (`grow.rs:89-98`, checked).

### `Zone` (`species.rs:174-189`)

| Setting | Today | Continuous form | Neutral | Closed form |
|---|---|---|---|---|
| `nodes` (`:175`) | `Uniform { min, max }` (u32) or `Poisson { mean }` (`species.rs:193-202`); Uniform's count is `min + ⌊u (max − min + 1)⌋`, every node at full presence (`lineage.rs:89-98`, checked) | real bounds `[a, b]`: a node count drawn on the real interval, the last node grown in by its fractional lead, as Poisson's nodes already are (`lineage.rs:99-110`, checked) | integer bounds give today's count | mean (a + b) / 2, as `NodeLaw::mean` (`species.rs:205-210`, checked) |
| law kind (Uniform or Poisson) | an enum | **not walked**: all four species use Uniform (dump, checked); only B's walk tree uses Poisson. A species walk holds Uniform | — | — |
| `buds` (`:177`) | u8, 1 to 6 (`species.rs:11`, `MAX_BUDS`) | real b ∈ [1, 6]: ⌊b⌋ buds, and one more made by a draw against frac(b). The whorl's slots turn by TAU · slot / b with b real (today TAU · slot / whorl, `geometry.rs:406`, checked), so the angles move by degree | the integer | expected buds = nodes · b · p (`closed_form.rs:113`, checked, with b real) |
| `lateral` (`:179`), `dormant` (`:182`) | one share per PA | already continuous; length N | — | unchanged |
| `delay` (`:185`), `rate` (`:188`) | reals | already continuous (fn-202) | — | unchanged |

Draws are hashed from the lineage, never taken from a stream (`lineage.rs:1-5`, `:41-44`, checked). An extra draw for a fractional bud, node or unit therefore moves no other draw, and at an integer value it is not taken.

## 2. The chain

### N = 15, youngest first

The order is a common linear order of all four species: each species' moves to a next age go forward, and each lateral is no younger than its bearer (`species.rs:429`, `table`, checked: `j < pa && p > 0` refused).

| # | Role | Beech (9) | Spruce (9) | Oak (10) | Palm (1) |
|---|---|---|---|---|---|
| 0 | seedling | — | 0 seedling | — | — |
| 1 | sapling | — | 1 sapling | — | — |
| 2 | trunk | 0 trunk | 2 trunk | 0 trunk | 0 stem |
| 3 | low fork | — | — | 1 fork | — |
| 4 | leader | 1 leader | 3 crown | 2 leader | — |
| 5 | top fork | 2 fork | — | — | — |
| 6 | limb | 3 limb | — | 3 limb | — |
| 7 | bough | 4 bough | — | 4 bough | — |
| 8 | sprig | 5 spur | 4 sprig | 5 sprig | — |
| 9 | branch | 6 branch | 5 branch | 6 branch | — |
| 10 | branchlet | — | 6 branchlet | — | — |
| 11 | spur | — | 7 spur | — | — |
| 12 | shoot | 7 shoot | 8 shoot | 8 shoot | — |
| 13 | twig | — | — | 7 twig | — |
| 14 | short shoot | 8 short | — | 9 short | — |

### Each species' moves on the chain

The `next` values, `lifespan` and laterals below are checked by a dump of each species and by reading `oak.rs`.

- **Beech:** 2 → 4 → 5, then stops (fork lifespan 2, `next` None); 9 → 12.
- **Spruce:** 0 → 1 → 2 → 4; 8 → 9; 10 → 11.
- **Oak:** 2 → 3 → 4 → 6; 8 → 9 → 12.
- **Palm:** 2, never moves on.

Every move skips only ages the species does not use, which it passes through at lifespan 0. Every lateral target is at or after its bearer.

### Can `next` always be the next age on the chain?

Yes for all four, on this chain, with two conditions:

1. **Unused ages are passed through at lifespan 0.** The beech's trunk passes the oak's low fork (3); the oak's branch passes 10, 11 (and nothing else) to its shoot at 12.
2. **The oak's own order breaks it, and the chain repairs it.** In `oak.rs` the branch (6) moves on to the shoot (8), skipping the twig (7) (`oak.rs:272-274`, checked). The twig is a lateral the oak uses, so it cannot be passed through on that path. On the chain the shoot (12) stands before the twig (13): the branch moves 9 → 12 passing only unused ages. The oak's laterals keep their order: twig before short, branch before twig (`oak.rs:248-249`, `:277-278`, checked), so `lineage::bud`'s cumulative choice (`lineage.rs:76-87`, checked) is unchanged.

A **stop** is not a move on the chain. With `next` always the next age, the beech's fork, every lateral age with `next` None (the beech's limb, bough and spur, the spruce's crown, the oak's limb, bough, twig, shoot and short), and the palm's stem need the `continuation` share at 0. Without it they would move into the following age. This is the one new setting the chain needs beyond the continuous forms the spec names.

## 3. What stays byte-identical, and what may not

| Claim | Standing |
|---|---|
| Re-indexing the PAs leaves every lateral choice unchanged where the relative order of the non-zero entries is kept: zeros do not move the cumulative bound (`lineage.rs:76-87`). True for all four species on the chain above | checked by reading |
| Unused zones appended after today's keep every zone key (`ZONE + z`) | checked by reading |
| Integer lifespans, bounds and buds take no extra draw, so their draws are today's | checked by reading (hashed draws) |
| **Continuation keys.** A move to the next age sprouts a new axis keyed `lineage.child(CONTINUATION)` (`grow.rs:249-255`, checked). Passing an age at lifespan 0 must therefore not sprout an axis, or every key above it changes. A walk that takes such an age from L = 0 to L = ε does sprout one, and every key above it moves at that crossing: a **pop**. | checked by reading; the fix is a design question (section 5) |
| Presence windows: the expected log-lengths are per PA (`closed_form.rs:289-301`); a passed-through age adds a row of no length. Byte-identical if the passed age contributes exactly nothing to the memoised walk | unknown, to be run |
| Dormant buds age along `next` while they sleep (`dormant.rs:128-137`, `grow/wake.rs:50`, checked). With pass-through ages and the continuation share, this aging must pass the same ages | checked that it reads `next` and `lifespan`; the behaviour under pass-through is unknown |

## 4. Cost

- **Code:**
  - `species.rs`: the types, the validation and `table`.
  - `grow.rs`: lifespan, continuation and buds.
  - `grow/wake.rs` and `dormant.rs`: aging along the chain.
  - `closed_form.rs`: fractional units and buds, continuation.
  - `lineage.rs`: real Uniform bounds.
  - `shed.rs`: real delay.
  - `geometry.rs`: real whorl spacing.
  - `presence.rs`: if the windows need it.
  - The four species files rewritten on the chain.
  - The refusal and walk tests.

  These are the same files phase E's worker is editing in `.worktrees/light` (`grow.rs`, `species.rs`), as the host noted. Size unknown until written; my estimate is several hundred lines changed, most of them the species files' re-indexing.
- **Run time:** the lateral tables grow from 9 or 10 entries to 15 and the presence table to age × 15. Both are small next to the phytomers. A passed-through age costs nothing. Unmeasured.
- **Byte identity:** with the continuation-key fix in section 5, option b, every passed species is expected byte-identical at its neutral values. With option a, not (section 5). The oak is not passed.
- **Tests:** A's oracle (`tests/oracle.rs`) and the closed form (`tests/closed_form.rs`) are rerun on the chain. B's walks gain lifespan, continuation, Uniform bounds, buds and shedding. R3's species strips are bare stills on one dressing, as the spec says.

## 5. Questions for the host

1. **Continuation keys (the pop).** Two ways out, not chosen:
   - (a) Key every development axis by its chain position from the seed's lineage, e.g. `root.child(CONTINUATION + age)`. An age appearing at ε moves no other key, but every existing continuation key changes, so the beech and spruce are not byte-identical (their look should hold; to be shown).
   - (b) Keep `lineage.child(CONTINUATION)`, but key a passed-through age's would-be axis so the next age reuses the key the passed age would have handed on. Byte-identical, but the key of an age depends on whether the age before it is present, which needs care to stay continuous.
2. **`continuation` as a new setting.** "`next` is always the next age" leaves no way to stop. Is a share the form you want, or a stop expressed through `abortion` at the end of the lifespan?
3. **Zone semantics in a walk.** Zones are matched by index. The beech's branch has 4 zones (bare, medial, top, and one more), the others 3, so zone 3 of one species walks against an empty zone of another, and the roles (bare, medial, top) line up only by index. Is matching by index enough, or should zones carry a role?
4. **The chain's order where species disagree.** The oak forks below its leader (3 → 4), the beech above (4 → 5); the chain holds both as separate ages. A walk from the beech to the oak therefore grows one fork in as the other fades. Is that the intended reading, or should a fork be one age at one place?

## Host decisions (2026-10-05)

1. **Keys:** each axis is keyed by its chain position. Continuity is the programme's claim; byte identity is only a preference under AGENTS.md's "Generator evolution". The beech and the spruce are re-judged on look: R1 is "unchanged in look", with a pixel diff recorded.
2. **Stops:** a probability of continuing at the end of the lifespan: 1 where an age moves on today, 0 where it stops. Reuse the abortion machinery if it expresses this exactly; otherwise the `continuation` share. One mechanism either way, with the choice and the reason stated.
3. **Zones carry fixed roles:** 0 the bare base, 1 medial, 2 the top whorl, 3 a spare. Each species maps its zones onto these roles, so a walk mixes like with like.
4. **The forks stay separate ages:** the oak's low fork (3) and the beech's top fork (5).
5. **fn-196's gate** is accepted. The beech memory-ceiling test derives its ceiling from free memory and fails under load; it is on the friction list for the end-of-run report.
