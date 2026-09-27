# R1: girth along the plane candidate's thickest limbs

2026-09-27, worker, fn-177.1. Tool: `crates/telperion-core/examples/limb_girth.rs`
(`LIMB_FAMILY=<overlay> limb_girth oregon-white-oak <seed>`), skeleton only, on
fn-170's `plane-candidate.json` over the oak, seeds 1 to 8. Raw lines are in
`raw/r1-*.jsonl` (ignored).

**Method.** A limb is one of the four thickest axis starts per seed (a lateral,
a codominant sibling, or a fork's primary). Its path runs from that start
through the thickest structural child at every fork to a structural tip, which
is the limb to the crown's edge. The path is 31 m long (median), and its base is
0.24 m. The radius is read at each tenth of the path as a share of its base. It
is split into the pipe part, which is the same family rebuilt with `lengthTaper`
0 (tip count plus fn-170's shares), and the taper part (radius / pipe).
`tips^.5` is the pure pipe model from the structural tip count. On the candidate,
the topology matched the `lengthTaper` 0 rebuild at all 8 seeds, so the split is
exact.

## The candidate (lengthTaper 0.2, lateralShare 0.45, forkBalance 0.9)

Medians over 32 limbs:

| path share | radius | pipe part | taper part | tips^.5 |
|---|---|---|---|---|
| 0.0 | 1.000 | 1.000 | 1.000 | 1.000 |
| 0.2 | 0.637 | 0.658 | 0.967 | 0.657 |
| 0.4 | 0.233 | 0.252 | 0.933 | 0.236 |
| 0.5 | 0.142 | 0.154 | 0.931 | 0.154 |
| 0.6 | 0.111 | 0.122 | 0.917 | 0.120 |
| 0.8 | 0.049 | 0.056 | 0.887 | 0.056 |
| 1.0 | 0.025 | 0.030 | 0.855 | 0.029 |

- **The tip count thins the limbs.** At mid-path the limb has 14% of its base
  girth. The pipe part accounts for 96% of that thinning in log terms, and
  `lengthTaper` for 4%. The pipe part follows the square root of the tip count
  to within 0.02, so fn-170's shares barely move the thickest path.
- **The thick wood is stems, not limb systems.** On 29 of the 32 paths (7 of 8
  seeds), the path starts on a codominant stem (`Node::stem`). It leaves the
  stems for a limb at a median 0.43 of its length (range 0.25 to 0.51), by
  which point it has already fallen to 25% of its base girth (range 11 to 45%).
  Seed 6 did not fork; its thickest limbs are limb systems off one trunk.

## Existing rows tried as the fix (same method, 8 seeds)

| path share | lengthTaper 0 | lateralShare 0.01 | lateralShare 0.01, forkBalance 0.1 |
|---|---|---|---|
| 0.2 | 0.658 | 0.809 | 0.817 |
| 0.4 | 0.273 | 0.612 | 0.700 |
| 0.5 | 0.156 | 0.585 | 0.672 |
| 0.8 | 0.056 | 0.424 | 0.483 |
| 1.0 | 0.030 | 0.304 | 0.147 |

- `lengthTaper` 0 changes nothing that matters: the curve is the pipe part.
- `lateralShare` at its floor holds the girth, but the limb never falls to leaf
  level. It ends at a structural tip with 30% of its base girth (about 70 mm on
  a 0.24 m limb), which is a blunt end with twigs on it. Every lateral also
  leaves with 10% of its pipe, and the thinnest structural wood falls from
  0.26% to 0.20% of a limb's base. Radius feeds growth, so the topology changed
  at 3 of 8 seeds, and that setting would also regrow the tree. Adding
  `forkBalance` 0.1 holds more to mid-path and still ends at 15%.
- None of the existing rows gives the shape the owner asked for: held girth
  over most of the reach, then a short fall to leaf level.

## What this means for the spec's design

1. The spec's rows are the right kind of fix: a profile over reach, and not a
   value of `lengthTaper` or `lateralShare`.
2. The spec's scope, "per limb system (fn-61's limb bound gives each system its
   reach)", misses the plane's thick wood. fn-61 bounds only first-order axes off
   a stem, and a stem keeps the crown's own bound. On this candidate the girth
   is lost on the codominant stems, over the first 43% of the path. A row that
   acts only on limb systems would start from limbs that already have 25% of the
   girth. Which systems the profile acts on, and what "reach" means for a stem,
   are host decisions.
