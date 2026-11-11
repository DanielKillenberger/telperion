# fn-354 R6: the girth and the sag as product, by the host, 11 Nov 2026

The rule is `PROBE-BRIEF.md`'s, which the probe built and the host accepted (`PROBE-VERDICT.md`). This file fixes how it becomes product. The probe's code is read for what touches what; nothing is copied from it without a rewrite and a test.

## The rows

Per growth stage, as every trait is (a species gives every stage the same value, as it does `inverse_height`):

| Row | Unit | Meaning | Neutral |
|---|---|---|---|
| `primary` | m | the radius of a shoot as it extends. Today's `pipe`, renamed; values unchanged | |
| `sapwood` | cm² per m² of leaf | sapwood area a unit of leaf needs: the reciprocal of the measured leaf area per sapwood area | 0: the stage lays no wood beyond its primary radius (the palm) |
| `sapwood_rings` | years | how many years wood stays sapwood before it is heartwood | |

- **Deleted** from the species, every preset, the scale table, validation, the wire formats and generated tables, the docs: `exponent`, `ripening`, `secondary`, `retained`, `leaf_girth`, and everything that only they reach (`girth/disused.rs`, the tree-as-grown pass that fed it, `Frozen`'s girth sums, the leaf-mean factor).
- **`sag` stays one row per stage,** per metre. Its values change (below).
- **A new phytomer's section** is `π × primary²` at its scale, with no ripening: `established` loses its `ripening` term.

Values, each with its source in the preset's comment, or the word "estimated" and why:

| Species | `sapwood` | `sapwood_rings` | `sag`, every stage |
|---|---|---|---|
| Oak | 1/0.30 = 3.33 (Phillips et al. 2003, Table 1: 0.23 and 0.37 m² per cm² at 10 and 25 m; the middle) | 20 (Hollstein in Haneca et al. 2009, Table 1: 20.4 at 100 to 200 years, European oak; estimated for *Q. garryana*) | 5.0e-6 (4ρg/E at 1,060 kg m⁻³ and 8.3 GPa; Lavers 1983 through Niklas & Spatz 2010; Hamilton 1975) |
| Beech | 1/0.331 = 3.02 (Bartelink 1997) | 50 (estimated: the middle of Gebauer et al. 2008's 25 to 73 conducting rings) | 4.1e-6 (1,030; 9.8) |
| Spruce | 1/0.36 = 2.78 (McDowell et al. 2002, Table 1) | 30 (estimated: no ring count read) | 6.1e-6 (960; 6.2) |
| Birch | 3.02 (estimated: the beech's, no source found) | 40 (estimated: no source found) | 3.7e-6 (930; 9.9) |
| Laurelin, Telperion | the beech's, their recorded base species | the beech's | the beech's |
| Ordinary tree | the beech's (estimated) | the beech's (estimated) | the beech's (estimated) |
| Palm | 0 | any (unread at 0) | as today (estimated; a palm's stem is not wood of this kind) |

Before a value is written, its source line is checked against the note that recorded it (`PHASE2-SURVEY.md`); a value whose note says "extractor" or "secondary" is marked so in the comment.

## The state and the pass

- Each phytomer carries its section `T` and its sapwood `S` (two `f32`), from the year it is laid until it is let go. The settle computes no girth: `radius = sqrt(T / π)`, times a fading branch's kept share as the probe did.
- Each year, after the cycle's growth and layout, one pass from the tips down computes the leaf above every held phytomer and applies the rule. It belongs with the upward pass the balance already makes each year (`Working::totals`): if one walk can serve both, it does; if not, say why. It runs on the grower's threads as the other passes do.
- **The leaf it reads comes from one function,** `leaf(phytomer, year)`, which today returns what the grower deposits into the light for that phytomer. R5 will change that function's body and nothing else.
- **The last year's layout always runs.** A phytomer laid in the last cycle has a scale and a leaf like any other. (It also removes one of the reasons a growth cannot go on past its horizon; fn-351's P4.) Its cost is measured.
- **A retired subtree** contributes nothing after it closes and needs no frozen girth; `Frozen` shrinks to what is still read, or goes.
- **Relay joins** take the radii of the wood on either side of the join as the rule gives them.

## The drawing eases a change of section

Along one stem (an axis, and the axis that continues or relays it), where the section falls from one phytomer to the next going up, the drawn radius falls no faster than 1 in 1: the thinner side is drawn at `max(its own radius, the radius below − distance along the stem)`. So a drop of half a metre in radius is a shoulder half a metre long, not a rim.

- It applies only along a stem's own line, never from a bearer to a lateral.
- It changes the drawn radius only. The grower's `T`, the sag's load and stiffness, and the measures read the wood.
- **Estimate, named:** the slope of 1 in 1 is the host's, from what reads as a shoulder; no measurement was found.
- The flat-topped cylinder under a living fork is not this rule's to fix (fn-279).

## Tests

- A stem whose crown is removed keeps its section the next year (wood is never lost).
- A section carrying constant leaf for N years and more holds `N`-th-free arithmetic: its sapwood settles at `leaf × sapwood` and its heartwood grows by one Nth of that a year.
- A stage with `sapwood` 0 draws its primary radius whatever its leaf.
- A shed branch leaves its bearer's section unchanged in the year it goes.
- The walks hold for `sapwood`, `sapwood_rings` and `primary` (by degree, bound unchanged), and a species blend passes through no step.
- A year is repeatable: grown straight through and paused and resumed give the same `T` and `S`.
- The drawn radius along a stem never falls faster than 1 in 1; a lateral's base is untouched by it.

## Measured before it is called done

- One thread, seed 1001: the oak at 40, 120 and 300, seconds and peak memory beside this branch before the rule, on a quiet machine or with the load stated; the yearly pass's own share.
- The oak at 40, 80, 120, 200, 300 on seeds 1001, 1007, 1012 through the scoreboard's runner as label `fn354-product`, sheets beside base and the probe's G2: the product should draw the probe's trees but for the eased steps and the last year's layout. Differences are explained.

## Not in this task

The leaf function's new body (R5), the beech's branching (R7), the twelve-seed scoreboard (R8), the gate and the review (R9).

## The fidelity dial (host, answer to the builder's stop)

Host decision on the coarse pipe row: option C.

Delete the coarse pipe row and its factor: a tree's primary radius does not move with fidelity, and the two fitted values (oak 3.742, spruce 2.2) go. Reason: under the rule nothing sums primary radii, so the factor has nothing left to compensate; the per-species coarse rows are ruled out by the owner and are deleted whole in a later phase, so no new value or renamed row is added for them now. The other coarse rows (factor, leaf) stay untouched in this task. If a test of the fidelity dial pinned trunk medians at fidelity above 0 against the old girth, it keeps checking what it promised with new numbers measured on this branch; list each such number with before and after. If deleting the row breaks the dial in a way that is not a number moving (a tree refused, a walk over fidelity failing its bound), stop and return with it.

Your list of smaller things is accepted as you propose: the sag load's stand-in becomes primary × scale; the supernatural override of pipe is renamed the override of primary with its values kept; the phytomer's light field, Reads::lights, the presence.rs assignment and the invariant go with leaf_girth; tolerance::respond's leaf_girth line goes; closed_form/kept and boundary are cut down to the count column the predictor's tests hold, their section and mass columns and those tests deleted; the examples' leaf_girth and retained lines go. /radii/forkExponent and the unrelated "secondary" words are not this task's.
