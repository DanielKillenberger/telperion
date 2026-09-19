# fn-86 measurements: what the twelve-byte leaf cost and saved

## R4: the numeric gates, and the leaves the cull kept

`npm run species:qa`'s measurement half, run whole at both ends of the change
and by the same command. Ninety-six cases: four species, twenty-four seeds
each - the protocol's twelve fixed seeds and the twelve drawn ones - across
the two profiles files the catalogue species are frozen in.

```
node tests/species.mjs --measure-only --profiles .flow/evidence/fn9/profiles.json
node tests/species.mjs --measure-only --profiles .flow/evidence/fn34/profiles.json \
  --seeds .flow/evidence/fn34/seeds.json
```

Every one of the ninety-six passed at `3345b07f`, the commit before the
encoding, and every one passes now. No gate moved, none was widened, and the
frozen profiles were not touched.

The retained leaf count is the cull's own answer and the number quantisation
could flip at the shell boundary, where half a position code decides whether a
leaf's last vertex lies inside the shell. Counts read from each case's
`output_bytes.retained_matrices`, divided by the bytes a leaf occupies at that
end - sixty-four before, twelve now.

| Species | Seeds | Seeds whose count moved | Worst single seed | Total before | Total after | Total move |
|---|---|---|---|---|---|---|
| `oregon-white-oak` | 24 | 0 | +0.00000% (715,065 -> 715,065) | 20,414,537 | 20,414,537 | +0.00000% |
| `norway-spruce` | 24 | 0 | +0.00000% (7,353,754 -> 7,353,754) | 171,041,910 | 171,041,910 | +0.00000% |
| `european-beech` | 24 | 24 | +0.00170% (5,004,170 -> 5,004,255) | 118,162,318 | 118,162,033 | -0.00024% |
| `silver-birch` | 24 | 20 | -0.00177% (225,635 -> 225,631) | 5,428,483 | 5,428,480 | -0.00006% |

R4 allows 0.1 percent. The worst single seed moves 0.0017 percent, which is
fifty-eight times inside it, and the worst is a beech: eighty-five leaves in
five million. The oak and the spruce do not move at all on any of their
twenty-four seeds. The birch moves by ones, twos and threes on twenty seeds
of twenty-four.

The shape is the shell test, not the encoding drifting: a beech leaf is small
against a 32 m box and its vertices lie densest at the shell, so it has the
most leaves within half a code step of the boundary; a spruce needle sits deep
inside the shell and a quarter millimetre never reaches the decision.

## R7: peak RSS, one test per process

The same poll both ends: `/proc/<pid>/status` VmHWM, read until the process
exits, one test to a process on the `ci` profile. `/usr/bin/time` is not
installed on this machine, so the poll reads the status file directly.

```
target/ci/deps/species-<hash> <test name> --exact --nocapture
```

| Test | After fn-85 | Now | Move |
|---|---|---|---|
| `fixed_spruces` | 3,244 MB | **1,763 MB** | -46% |
| `fixed_beeches` | 3,945 MB | 2,589 MB | -34% |
| `fixed_oaks` | 1,469 MB | 1,271 MB | -13% |
| `fixed_birches` | 778 MB | 704 MB | -10% |

R7 asked for `fixed_spruces` under 1,800 MB. Each test holds four specimens at
once, and the spruce at these seeds carries about 7.35 million leaves apiece:
fifty-two bytes a leaf saved over four specimens predicts about 1,530 MB, and
1,481 MB came off. The beech saves less against its leaf count because its
wood mesh, which this spec does not touch, is the larger half of what it
holds.

## R5: the digests that moved, before and after

Every stored byte changed, so every committed geometry digest changed with it.
The pairs below are seed 1 for each species, read from `3345b07f` and from the
encoding commit; the full set is `crates/telperion-core/tests/species/digests.json`,
recommitted in `5b49227c` alongside the four `catalogue/*/pins.json`.

| species | seed | before | after |
|---|---|---|---|
| `european-beech` | 1 | `f160e7b5d26e8b5f` | `20f49a7c4e806880` |
| `norway-spruce` | 1 | `ff7036aa4bcde448` | `91d4655c94fa66ac` |
| `oregon-white-oak` | 1 | `648a3aaf8592e555` | `d2e207bdf8d77c11` |
| `silver-birch` | 1 | `e20e3d6368f80ac2` | `cac3ef4d4fd5ca39` |

**49 of 49** committed digests moved. A re-encoding that left any of them
standing would mean a leaf whose bytes the change did not reach, so the count
matching the total is the check, not a formality. The digests are a change
detector rather than an identity pin: fn-53's `--pin-note` governs preset value
tables and does not apply here, and the owner ruled on 2026-09-19 that bytes are
expected to move while the generator is in development.

## The European ash

The fifth catalogue folder has no preset, an empty `pins.json`, and no entry
in either profiles file; `species_measure`'s own help says it "is not a
catalogue species". It has no numeric gate to run and no still to draw.
