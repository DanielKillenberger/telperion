# FN30: calibrated growth, as the machine measured it

The production tree now grows to the family's requested age. The oak and spruce
height fits meet the three chosen reference heights within 15 percent. Their
trunk diameters do not. R1 stops on those numbers pending the owner's judgment;
the implementation and new pins do not constitute botanical acceptance.

## Protocol

Native release builds, seed 7, full preset geometry, annual fn-11 slices and
pinned libm arithmetic. The host measured the trees with `growth_curve`; its
JSONL files are in the scratch directory named below. Height is the highest
wood node above the root. DBH is the diameter interpolated at 1.3 m above the
root along the thickest structural continuation. It is a trunk measurement,
not a maximum branch radius. Skeleton bounds exclude bark and foliage; mesh
bounds in the identity table include both. The reference ages have decimal
labels, but the simulator completes whole yearly slices with no interpolation.

The host's `curves.py` fits the continuous envelope fraction. This report uses
measured wood for the acceptance comparison. Reference values and deviations
below use the rounded output of that script, so the last decimal is approximate.
The three fitted ages are where the reference height reaches one third, two
thirds and all of the preset envelope. The third point is labelled envelope age.
Derived maturity is also reported; it is a numerical saturation age, not an
observed biological maturity. It must not silently replace an acceptance age.

No GPU was used by this implementer. CPU timings are observations on the shared
host, with no reserved exclusive window. They are three-build medians measured
by the core's existing `Instant` timers, excluding surface and foliage creation.
The host is an AMD Ryzen 9 5950X (16 cores, 32 logical CPUs), x86_64, Linux
7.2.3-arch1-3, with rustc 1.98.1 (48a229cea, 2026-09-01). Measurements ran on
2026-09-14. No CPU timing is a GPU frame-time claim.

Host input directory:
`/tmp/claude-1000/-home-daniel-Projects-telperion/0c1f978b-1e20-4a9d-8dc7-e404f4a42a41/scratchpad`.

Reproduce the numeric trees with:

```sh
cargo run --release -p telperion-core --example growth_curve -- --preset oregon-white-oak --seed 7 --ages 26.7,56.1,112 --envelope
cargo run --release -p telperion-core --example growth_curve -- --preset norway-spruce --seed 7 --ages 14.1,26.6,36.9 --envelope
python3 .flow/evidence/fn30/curves.py --oak-dbh 0.836 --spruce-dbh 0.422
```

## The references

The five files below are the host's fetched bytes in `scratchpad/refs/`.
SHA-256 and byte counts were recomputed locally. The PDF text extractions are
`ertrag.txt`, `gould.txt` and `utd.txt`. The script transcribes equations and
tables from those sources. Source documents remain outside the repository.

| ID | File | Source | SHA-256 | Bytes |
|---|---|---|---|---:|
| E1 | `ertragstafeln.pdf` | [Bavarian yield-table extracts, Jüttner 1955 oak and Wiedemann 1936/42 spruce](https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf) | `c6c7d6558fe6f8157c2fea3a67e8aaed133902b18446eb32275972fc2a5f9885` | 351,827 |
| G1 | `gould2011.pdf` | [Gould, Harrington and Devine 2011](https://cascadiaprairieoak.org/wp-content/uploads/2014/07/Gould-P.J.-C.A.-Harrington-and-W.D.-Devine.-2011.-Growth-of-Oregon-White-Oak-Quercus-garryana.pdf) | `3bac24bce07da85b47b280a7a78c0e6a220de2e86930e5ddbe56249d63916341` | 605,406 |
| S1 | `silvics-oregon-white-oak.html` | [Stein 1990, Silvics](https://research.fs.usda.gov/silvics/oregon-white-oak) | `21d0db9ce11e8ee1966c5adc97bb3140f64ef2efc8744de0be72f587fdbeeb5f` | 103,163 |
| V1 | `pmc2987550.html` | [Vospernik, Monserud and Sterba 2010](https://pmc.ncbi.nlm.nih.gov/articles/PMC2987550/) | `3193a0d83020a90fa21da2e2f49b0bab6bfde3304b04fbaf2de1ce5214bb06ad` | 314,453 |
| U1 | `utd-gtr253.pdf` | [Urban Tree Database report](https://research.fs.usda.gov/download/treesearch/52933.pdf) | `0765570cf3513a8cc91ba07b4abd7702611b5e765b07e93cde67555b4097b1c7` | 10,558,386 |

U1 lists Quercus garryana only as a biomass-equation assignment and does not list
Picea abies. It supplies no direct age curve used here. No complete accessible
open-grown age-indexed height and DBH pair was found in the inherited research.
The following curves are explicit compositions, with stand-grown fallbacks.

## The curves and their composition

Oak height uses E1 Jüttner oak site class II, a Quercus robur/petraea proxy.
S1's forest-grown 24 m at 95 to 135 years motivates that class. This remains a
stand-height proxy, with no measured conversion to open-grown height. Oak DBH
starts at E1 site class I's 7.2 cm at age 30. Thereafter G1's open-grown case
sets BA and BAL to zero and integrates one tenth of its decade change in
squared DBH each year, at Douglas-fir site index 35 m. The anchor and integration
are composition choices, not measurements of a 30-year-old open-grown Garry oak.
Before age 30 both oak dimensions extend linearly from zero to their first
anchor. Gould DBH uses the nearest integer year. Its worked 40 cm case yields
2.82 cm per decade, matching the paper's rounded 2.8 cm.
G1 calls this the large-tree equation and leaves the under-8-cm growth model
outside its revision. Integrating from the 7.2 cm anchor therefore also extends
the large-tree equation into the small-tree range; that is an additional limit
of this composition, not an open-grown observation.

Spruce height uses E1 Wiedemann site class I. Spruce DBH doubles that class's
stand mean diameter following Sterba 1975 via V1. Height retains the stand-grown
proxy; doubling DBH does not establish an open-grown height-age curve. Both
spruce dimensions interpolate linearly between ten-year rows and extend from
zero below the first row at age 20. E1 also has a class III diameter column;
class I is the selected composition choice. The script comments now distinguish
transcriptions from composition choices; calculations are unchanged. At age 100 the composition
gives 0.752 m DBH against V1's Lässig observation of 0.91 m, a 17.4 percent miss.

The script extrapolates the final height segment past age 200 for oak and 120
for spruce, and spruce DBH past 120. The derived-mature rows below expose those
extrapolations; they are not published observations at 432 or 158 years. V1 also
records five open-grown spruce at age 300 averaging 23 m and 0.81 m DBH, which
shows why indefinite linear extrapolation cannot serve as a maturity target.

| Species / point | Age y | Reference height m | Reference DBH m | Reference H/DBH | Measured height m | Measured DBH m | Measured H/DBH | Height error % | DBH error % | Provenance at this age |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---|
| oak / young | 26.7 | 8.00 | 0.064 | 125.0 | 7.575 | 0.266618 | 28.4 | -5.3 | +316.6 | E1 II linear extension to age 30; E1 I DBH anchor linear extension |
| oak / middle | 56.1 | 16.00 | 0.112 | 142.8 | 16.603 | 0.580425 | 28.6 | +3.8 | +418.2 | E1 II 50–60 interpolation; G1 open-grown integration from E1 I anchor |
| oak / envelope | 112.0 | 24.00 | 0.236 | 101.8 | 22.605 | 0.789969 | 28.6 | -5.8 | +234.7 | E1 II 110–120 interpolation; G1 integration from E1 I anchor |
| oak / derived-mature | 432 | 39.48 | 0.902 | 43.8 | 23.730 | 0.835595 | 28.4 | -39.9 | -7.4 | E1 II extrapolation beyond 200; G1 integration from E1 I anchor |
| spruce / young | 14.1 | 5.00 | 0.106 | 47.3 | 4.800 | 0.122191 | 39.3 | -4.0 | +15.3 | E1 I linear extension to age 20; E1 I DBH ×2 from V1 |
| spruce / middle | 26.6 | 10.00 | 0.203 | 49.3 | 10.500 | 0.299797 | 35.0 | +5.0 | +47.7 | E1 I 20–30 interpolation; E1 I DBH ×2 from V1 |
| spruce / envelope | 36.9 | 15.00 | 0.285 | 52.6 | 12.900 | 0.369860 | 34.9 | -14.0 | +29.8 | E1 I 30–40 interpolation; E1 I DBH ×2 from V1 |
| spruce / derived-mature | 158 | 40.08 | 1.144 | 35.0 | 15.000 | 0.421595 | 35.6 | -62.6 | -63.1 | E1 I height and DBH extrapolation beyond 120; DBH ×2 from V1 |

## The calibration and derived mature ages

The grid search minimizes squared log height error at the first three ages,
with rate steps of 0.001 and shape steps of 0.1. It leaves the host's selected
traits unchanged. It does not fit DBH independently because fn-11 gives the
trunk radius the live envelope's scale. Changing rate and shape moves height
and diameter together. The reference ratios in the table vary with age;
the generated ratios stay near 28 for oak and 35 for spruce after establishment.

| Family | growth.rate | growth.shape | Derived mature age y | Shedding threshold |
|---|---:|---:|---:|---:|
| Oregon white oak | 0.032 | 2.0 | 432 | 0 |
| Norway spruce | 0.091 | 3.4 | 158 | 0 |
| Ordinary | 0.08 | 2.0 | 173 | 0 |
| Telperion | 0.08 | 2.0 | 173 | 0 |
| Laurelin | 0.08 | 2.0 | 173 | 0 |

Maturity is the first integer year whose Chapman-Richards fraction rounds to
all 250,000 lifetime work units. `GrowthTraits::mature_age` exposes that existing
bisection; every preset derives `family.age` from its final traits. Callers can
still request any supported age. No growth, chronicle or determinism rule changes.
Ordinary and the Two Trees retain provisional growth traits without species
calibration. Site-class sensitivity puts the oak's 24 m reference age at 81.7 y
in class I and 148.3 y in class III; spruce reaches 15 m at 45.4 y in class II
and 56.1 y in class III. The selected classes are assumptions the owner may judge.

## The convergence numbers

This section was written before any identity literal moved. The host's
`gc-oak.jsonl` and `gc-spruce.jsonl` compare derived-mature growth with the legacy
one-shot build at seed 7 and identical calibrated parameter values.

| Species / build | Nodes | Crossover | Min node bounds m (x,y,z) | Max node bounds m (x,y,z) | Height m | Trunk DBH m |
|---|---:|---:|---|---|---:|---:|
| oak / growth | 196,901 | 5,394 | (-12.571036278, 0.000000000, -13.000525755) | (13.162611849, 23.729754716, 12.981341126) | 23.729754716 | 0.835594895 |
| oak / one-shot | 139,040 | 3,602 | (-13.104248136, 0.000000000, -13.189332654) | (13.154735324, 23.463097053, 12.882966674) | 23.463097053 | 0.836387092 |
| spruce / growth | 76,386 | 21,442 | (-3.859580435, 0.000000000, -4.160924095) | (3.818372645, 15.000000000, 4.068296043) | 15.000000000 | 0.421595017 |
| spruce / one-shot | 90,439 | 17,573 | (-3.883032539, 0.000000000, -4.190694609) | (4.332099577, 15.000000000, 3.794309475) | 15.000000000 | 0.426707275 |

Oak nodes change by +57,861 (+41.61 percent), crossover by +1,792 (+49.75 percent).
Spruce nodes change by -14,053 (-15.54 percent), crossover by +3,869 (+22.02 percent).

Annual frontier visits under an expanding envelope produce a different amount
and placement of wood from one-shot generation. Zero shedding leaves those
branches alive. Oak's largest endpoint shift is 0.533 m at min-x; spruce's is
0.514 m at max-x. Whole-axis spans change by about -2.00%, +1.14%, -0.35% for
oak and -6.54%, 0.00%, +3.06% for spruce (x, y, z). Comparing individual endpoints
against their distance from zero would instead show spruce max-x at -11.86%;
these are different denominators, not a six-percent bound on every coordinate.

Ordinary's legacy audit also changes because its threshold moves from 0.45 to
zero. The earlier 0.45 audit hash is 9848876633805652422; the host measured the
zero-threshold hash as 4584312898131064280. Oak and spruce already had zero
thresholds, so their legacy audit hashes do not move. The production identity
pin will hash the grown skeleton, and will retain both element hashes exactly.

## The re-pin

The production skeleton and placement hashes change because the grown tree
has the node populations and positions measured above. Every assertion remains
exact. Surface sweeps follow the changed segment population, so oak wood vertices
rise about 40.00 percent and spruce vertices fall about 12.89 percent. Living
shoots determine cohort placements; oak instances rise 35.19 percent and spruce
instances fall 22.09 percent. Bark/leaf-inclusive bounds follow the new wood and
placements. The unchanged leaf elements keep their old hashes.

| Pin | Oak old | Oak new | Spruce old | Spruce new |
|---|---|---|---|---|
| wood_vertices | `4262170` | `5966860` | `2888144` | `2515982` |
| wood_triangles | `8255000` | `11564840` | `5580040` | `4852280` |
| instances | `869310` | `1175265` | `7012326` | `5463221` |
| min | `[-13.163122928115051, -0.09600000083446503, -13.242490423042556]` | `[-12.660554941030515, -0.09600000083446503, -13.064155719625399]` | `[-3.89500647744516, -0.05999999865889549, -4.197530933827597]` | `[-3.8689955989331346, -0.05999999865889549, -4.169345860968122]` |
| max | `[13.217684715842124, 23.557227415847606, 13.003187181590542]` | `[13.276412718982542, 23.817030705282615, 13.087342970966304]` | `[4.337495164451377, 15.0, 3.8062214356137005]` | `[3.8295214987058834, 15.0, 4.083653705781007]` |
| skeleton | `14986275773972546726` | `2069374647478841013` | `12735573889651776723` | `9830532764016443315` |
| placement | `15624359871475047912` | `15826952556907210550` | `8171270653015517335` | `10177991485176080717` |
| element | `4207404028969543471` | `4207404028969543471` | `7287062639823569932` | `7287062639823569932` |

`identity.rs` now hashes `Specimen::build(&family).read().tree`, the production
skeleton, at family.age. It previously hashed `branching::generate()` while
mesh::build already used growth, which compared two different trees in one test.
The seed remains 7. The fn-24 look image is a separate GPU pin left to the host.

`branching/audit.rs` retains its one-shot stage audit at preset seed 42.
Ordinary's zero threshold retains 49,262 nodes versus 9,240 at 0.45, an increase
of 40,022 local nodes, with crossover 2,484 in both. Both bounds are exactly
min (-4.92996169732822, 0, -5.941603822258199), max (5.945149868893464,
22.107918292195386, 5.55847480242771). This explains its sole hash move from
9848876633805652422 to 4584312898131064280. The oak audit stays
11389017044164293456 and spruce stays 17659250574543300401. All pin values were
collected once in `/tmp/fn30-pins.log` before changing any literal. The temporary
collector was removed after recording them.


## What fn-11 measured

Native three-build medians from [fn-11's report](../fn11/REPORT.md), carried
without rerunning or relabelling them as calibrated costs.

| Operation | Oak | Spruce | Commit |
|---|---:|---:|---|
| One-shot envelope | 67.8 ms | 37.1 ms | 2fd2668 |
| Mature growth, 173 yearly slices | 1,273.5 ms | 932.9 ms | 972bace, 2fd2668 |
| Sparse yearly advance | 13.4 ms | 20.6 ms | 2fd2668 |
| Of which change record | 5.5 ms | 12.7 ms | 2fd2668 |
| Sparse record plus packed read | 6.6 ms | 13.3 ms | 2fd2668 |
| Few-change yearly advance | 6.6 ms | 4.9 ms | 2fd2668 |
| Earlier-age read, mature tree | 262.3 ms | 2,314.7 ms | 2fd2668 |
| Snapshot round trip | 320.7 ms, 121.9 MB | unmeasured | 972bace |

Before the chronicle, monthly mature oak builds cost 5.906 s against 0.775 s
with yearly slices. fn-11 chose yearly slices on that evidence. STATUS.md holds
earlier intermediate timings of 18.9 s oak and 9.15 s spruce; those precede
the finished chronicle cost work and are not the final baseline.

## The mature build cost after calibration

The existing `tolerance_cost_report` measures `Specimen::build` directly at
the traits' derived maturity, seed 7, with the production resize tolerance of
0.0001 m. The command and its unedited output are in `/tmp/fn30-cost.log`.

```sh
FN11_TOLERANCE=0.0001 cargo test --release -p telperion-core --lib tolerance_cost_report -- --nocapture
```

| Species | Age y | Build samples ms | Median ms | Nodes | Radius keyframes |
|---|---:|---|---:|---:|---:|
| Oak | 432 | 2544.243810, 2446.199166, 2463.322165 | 2463.322165 | 196,901 | 1,996,669 |
| Spruce | 158 | 877.680822, 866.292811, 867.473470 | 867.473470 | 76,386 | 243,028 |

Oak costs 1.93 times fn-11's 1273.5 ms mature build; spruce costs 0.93 times
its 932.9 ms. The longer oak history carries more nodes and radius keyframes.
These are chronicle/wood timings, excluding the owned foliage read, meshing,
Wasm overhead and GPU submission. They do not establish a dial-to-frame latency.
The unchanged half-second target is missed by both species.

## The strips and the stills

The host rendered every still through the headless example on an NVIDIA GeForce RTX 3080 at 4 samples a pixel, seed 7, with `--age` selecting the production growth path. Frames: `target/release/examples/headless --preset <id> --seed 7 --age <years> --size 700x1000 --out .flow/evidence/fn30/strips/<id>-age-<years>.png`, six per species (oak 10, 26.7, 56.1, 112, 200, 432 years; spruce 5, 14.1, 26.6, 36.9, 60, 158 years), composed with `montage -tile 6x1 -geometry +4+0` into the two strips below; the individual frames stay on disk under `strips/` and are gitignored. The mature heroes use fn-14's command (`--size 1600x1000`, no `--age`, so the preset's derived mature age), and the beside-fn-14 montages place fn-14's committed hero on the left. The host inspected four images: the two strips and the two beside-fn-14 montages. Both mature stills also carry fn-26 and fn-27 appearance work, so a difference from fn-14 is not growth alone.

| Artifact | File | Host inspection |
|---|---|---|
| Oak age strip | `strips/oregon-white-oak-strip.png` | 10 y: a bare pole with a few leaves; 26.7 y: an open sapling with sparse leaves, 2,704 placements; 56.1 y onward: the full rounded crown, broader through 112, 200 and 432 y with little visible change after 112 y |
| Spruce age strip | `strips/norway-spruce-strip.png` | 5 y: a bare stem with no needles; 14.1 y: a sparse whorled sapling; 26.6 y onward: the tiered cone filling in, the leader visible at every age |
| Mature oak | `oregon-white-oak-hero.png` | 432 y, 5,966,860 wood vertices, 1,175,265 leaves |
| Mature spruce | `norway-spruce-hero.png` | 158 y, 2,515,982 wood vertices, 5,463,221 needles |
| Oak beside fn-14 | `oregon-white-oak-beside-fn14.png` | the grown oak is denser and taller in the crown with more limbs reaching low and to the right; the fn-14 still is rounder |
| Spruce beside fn-14 | `norway-spruce-beside-fn14.png` | the grown spruce keeps the leader and the tiers; its lower tiers are shorter than fn-14's and the crown reads narrower |

The fn-24 look pin `.flow/evidence/fn24/ordinary-hero.png` was regenerated once with the test's own command (`headless --preset ordinary --seed 7 --size 1600x1000 --view clay`) and the reason recorded in `crates/telperion-render/tests/look.rs`. Four renderer tests still fail after routing (`bark_detail`, `bark_distance`, `bark_resolution`, `conformance`): their fixtures build parameter sets through `mesh::build`, which now grows them, and two of those sets grow no wood or no placements; they are recorded as open in the blocker, not fixed here. Owner verdicts on the strips and the stills are open; the slots are in the verdict section.

## Deviations the owner may judge

R1's DBH misses remain visible in the curve table, including the envelope-age
misses. At numerical maturity the extrapolated height targets also fail.
A fitted stand-height proxy and a mixed-species oak diameter anchor do not
establish open-grown biological age. The owner may reject the reference
composition, the model's fixed height-to-diameter coupling, or both.

The vigour proxy divides exposure by `1 + rate * node age` without a floor.
With a nonzero shedding threshold, every inactive shoot eventually falls
below it. The host measured Ordinary falling from 28,401 nodes at age 25 to
33 at 100, Telperion from 115,602 at 50 to 30 at 125, and Laurelin from 149,102
at 50 to 169 at 173 with threshold 0.45. The rule is outside this spec's boundary.
The value-table workaround sets those three thresholds to zero. At age 173 and
seed 7 they retain 44,235, 190,340 and 194,517 nodes respectively. This disables
shedding for those production presets and leaves the ageing proxy defect for a
follow-up. Tests of shedding set 0.45 explicitly and keep every assertion.

The sparse oak and spruce cost fixtures and spruce apical-loss fixture also
state their original 0.08/2.0 growth schedule explicitly. Calibration moved the
old hard-coded test ages to a different growth phase; fixture independence
preserves the mechanisms those assertions test. The sweep now walks rate,
shape and derived age, and records sheddingThreshold among held values. The
browser parameter test updates its explicit Ordinary threshold from 0.45 to 0,
matching that same documented value-table change. The earlier implementation
had relaxed mature foliage assertions in `tests/mesh.rs`; this pass restores
them because the zero-threshold presets retain their crowns.

## Gates

The implementer ran these commands. Full unedited logs are at the absolute
paths below. The renderer crate was linted but its tests were left to the host.

| Command | Exit | Result / log |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | No output; `/tmp/fn30-fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `/tmp/fn30-clippy.log` |
| `cargo test --release -p telperion-core -p telperion-wasm` | 0 | 224 passed, 0 failed, 7 ignored across all targets; `/tmp/fn30-core-wasm-test.log` |
| `npm run wasm:build` | 0 | Built Wasm and regenerated Rust family metadata; `/tmp/fn30-wasm-build.log` |
| `npm run --ignore-scripts typecheck` | 0 | `/tmp/fn30-typecheck.log` |
| `npm run --ignore-scripts test` | 1 | 67 passed, 10 parity tests skipped after fixture failure; `/tmp/fn30-npm-test-final.log` |

The final JavaScript run completed in 236 ms within the ten-minute limit. Its
only failed suite is `harness/parity.test.ts`, whose `beforeAll` cannot spawn
Cargo in this sandbox (`Error: spawnSync cargo EPERM`). The earlier Ordinary
threshold expectation was corrected with the documented preset change; its
24 parameter tests pass. This does not establish native/Wasm byte parity for
the new production path. The host must rerun `npm run --ignore-scripts test`
with subprocess access. No parity assertion was changed or disabled.

The source-curve script was rerun with `--oak-dbh 0.836 --spruce-dbh 0.422`.
Its output is byte-identical to the host's `curves-out.json` after the comment
corrections. Source checksums match all five supplied hashes.

## Verdicts

R1 is blocked. All three fitted heights meet 15 percent; DBH does not. No owner
acceptance of these misses has been supplied. Derived-maturity extrapolations
also miss, and remain contextual rather than validated open-grown targets.

R2 has production routing, convergence evidence and one exact core re-pin.
The production identity test passes with both leaf elements unchanged. The host
owns the remaining GPU validation and the final verdict.

R3 awaits the host's strips and mature stills and the owner's words.

R4's report is supplied with references, per-age composition, fn-11 costs,
calibrated CPU costs, convergence, pin results, deviations and GPU command/file
placeholders. The host supplies its visual evidence and the owner's words.
No empty owner slot is an acceptance.

## Owner verdict

- Reference composition and R1 diameter deviations:
- Derived-maturity interpretation and convergence:
- Oak age strip and mature still beside fn-14:
- Spruce age strip and mature still beside fn-14:
- Zero-threshold workaround and production build cost:
