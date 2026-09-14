# FN31: sapling form, thickening and retained crowns

Round 3 continues from bf92380 after the owner rejected the preceding strips.
The current evidence is below. `round3/ROUND2.md` preserves the rejected report.
The owner verdict slots remain empty.

## Protocol

Native release builds on the Ryzen 9 5950X and NVIDIA RTX 3080, seed 7.
Production still grows the same yearly rule to each preset's derived age.
`measure.py` runs `cargo run --release -p telperion-core --example growth_curve
-- --preset <id> --seed 7 --ages <ages> --envelope`. The JSONL files in
`measurements/` are the current measurements; command logs are in `logs/measure-<id>.log`.
The instrument now also counts lateral shoots and structural laterals, while
retaining the inherited height, DBH, bounds and placement measurements.
DBH follows the thickest structural continuation and interpolates at 1.3 m.
Height and bounds describe wood above the root, excluding leaves and bark.
Decimal ages complete integer slices. Read-at-age remains a filter over stamps.

## References reused

These are fn30's URLs and SHA-256 values, reused without re-sourcing or additions.
`logs/checksums.log` records the inherited source-byte checks; Round 3 rechecks
`../fn30/curves.py` in `round3/logs/checksums.log`. Its SHA-256 remains
`9beda29216a153a3ef5a882c1ca16fd938a60e1d457ed0c8912bfcf7e0568dc4`.
The seedling heights remain implementation values for the spec's centimetre-scale
form, rather than new sourced observations.

| ID | File | Source | SHA-256 | Bytes |
|---|---|---|---|---:|
| E1 | `ertragstafeln.pdf` | [Bavarian yield-table extracts, Jüttner 1955 oak and Wiedemann 1936/42 spruce](https://www.forstpraxis.de/sites/forstpraxis.de/files/2023-07/AFZ_FHJ_Kalender_2024_306_318_Ertragstafeln_ste_OK.pdf) | `c6c7d6558fe6f8157c2fea3a67e8aaed133902b18446eb32275972fc2a5f9885` | 351,827 |
| G1 | `gould2011.pdf` | [Gould, Harrington and Devine 2011](https://cascadiaprairieoak.org/wp-content/uploads/2014/07/Gould-P.J.-C.A.-Harrington-and-W.D.-Devine.-2011.-Growth-of-Oregon-White-Oak-Quercus-garryana.pdf) | `3bac24bce07da85b47b280a7a78c0e6a220de2e86930e5ddbe56249d63916341` | 605,406 |
| S1 | `silvics-oregon-white-oak.html` | [Stein 1990, Silvics](https://research.fs.usda.gov/silvics/oregon-white-oak) | `21d0db9ce11e8ee1966c5adc97bb3140f64ef2efc8744de0be72f587fdbeeb5f` | 103,163 |
| V1 | `pmc2987550.html` | [Vospernik, Monserud and Sterba 2010](https://pmc.ncbi.nlm.nih.gov/articles/PMC2987550/) | `3193a0d83020a90fa21da2e2f49b0bab6bfde3304b04fbaf2de1ce5214bb06ad` | 314,453 |
| U1 | `utd-gtr253.pdf` | [Urban Tree Database report](https://research.fs.usda.gov/download/treesearch/52933.pdf) | `0765570cf3513a8cc91ba07b4abd7702611b5e765b07e93cde67555b4097b1c7` | 10,558,386 |



Oak height uses E1 Jüttner site class II as a Quercus robur/petraea stand-height
proxy. The young DBH composition extends the 7.2 cm age-30 E1 class-I anchor
linearly below 30, then integrates G1's large-tree equation with BA and BAL zero
and site index 35 m. Spruce uses E1 Wiedemann class-I height and twice its stand
mean diameter, following Sterba via V1, with linear extension below age 20.
U1 supplies no age curve used here. The owner's doubt about the young-age
composition remains; Stein's independent mature form is available in S1.

## Ratio and diameter at the fit ages

Reference values are fn30's rounded composed-curve outputs, unchanged. Signed
percent deviations use those values. H/D here uses measured height and DBH;
the separate radius-history test uses live-envelope height and root diameter.

| Preset | Age | Height m | DBH m | Reference DBH m | DBH deviation | H/DBH | Reference H/DBH | Ratio deviation |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| oregon-white-oak | 26.7 | 7.736033 | 0.057017819 | 0.064 | -10.91% | 135.677 | 125.0 | +8.54% |
| oregon-white-oak | 56.1 | 16.692183 | 0.126274552 | 0.112 | +12.75% | 132.190 | 142.8 | -7.43% |
| oregon-white-oak | 112 | 22.440165 | 0.221363284 | 0.236 | -6.20% | 101.373 | 101.8 | -0.42% |
| norway-spruce | 14.1 | 4.893250 | 0.099784719 | 0.106 | -5.86% | 49.038 | 47.3 | +3.67% |
| norway-spruce | 26.6 | 10.630239 | 0.230683255 | 0.203 | +13.64% | 46.082 | 49.3 | -6.53% |
| norway-spruce | 36.9 | 12.984213 | 0.286738899 | 0.285 | +0.61% | 45.282 | 52.6 | -13.91% |

R2 tolerance misses: none against the inherited composed references. The owner chooses the accepted reference in the empty R2 slot.

The inherited extrapolated maturity references are oak H 39.48 m / DBH 0.902 m and spruce H 40.08 m / DBH 1.144 m. They extend beyond the published height tables, while the authored envelopes remain 24 m and 15 m. The mature measured dimensions below remain explicit; those extrapolations are not observed mature trees.

## Derived mature ages

| Preset | Rate | Shape | Derived years |
|---|---:|---:|---:|
| oregon-white-oak | 0.032 | 2 | 432 |
| norway-spruce | 0.091 | 3.4 | 158 |
| ordinary | 0.08 | 2 | 173 |
| telperion | 0.08 | 2 | 173 |
| laurelin | 0.08 | 2 | 173 |

The derivation finds the first integer slice where the Chapman-Richards fraction
snaps to one at `1 - 0.5/250000`. Re-evaluating the final traits gives the ages
above. Geometry and secondary thickening changed; that height-curve saturation
criterion remains unchanged. These are numerical saturation ages, not sourced
biological maturity observations.

## Young-age evidence

Laterals count nodes stamped with lateral fate; they include local woody shoots
and twigs. The focused test additionally requires woody lateral shoots and checks
that at least half of upper lateral shoots have foliage on themselves or their
descendants. It does not require a structural fork at the base of a seedling.

| Preset | Age | Nodes | Laterals | Structural laterals | Placements |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak | 1 | 7 | 1 | 0 | 16 |
| oregon-white-oak | 2 | 9 | 2 | 0 | 20 |
| oregon-white-oak | 10 | 153 | 76 | 0 | 1,382 |
| oregon-white-oak | 26.7 | 6,164 | 3,130 | 69 | 66,593 |
| norway-spruce | 1 | 7 | 1 | 0 | 6 |
| norway-spruce | 2 | 21 | 13 | 0 | 25 |
| norway-spruce | 5 | 44 | 24 | 0 | 184 |
| norway-spruce | 14.1 | 3,586 | 2,194 | 411 | 78,487 |
| ordinary | 1 | 7 | 1 | 0 | 21 |
| ordinary | 2 | 13 | 4 | 0 | 18 |
| ordinary | 3 | 17 | 6 | 0 | 37 |
| ordinary | 4 | 21 | 8 | 0 | 57 |
| ordinary | 5 | 39 | 21 | 3 | 149 |
| ordinary | 6 | 68 | 33 | 5 | 307 |
| ordinary | 7 | 118 | 48 | 12 | 444 |
| ordinary | 8 | 222 | 82 | 25 | 690 |
| ordinary | 9 | 409 | 143 | 52 | 1,131 |
| ordinary | 10 | 891 | 444 | 85 | 4,639 |

The all-five-presets test builds every whole year 1 through 10 and rejects an
empty placement vector or an empty production foliage mesh after shell culling. Its initial red named Ordinary 2–4, both Two Trees 1–10,
and the separate woody-lateral test was red for oak 10 with zero woody laterals.
`round3/logs/form-red.log` and `round3/logs/woody-laterals-red.log` retain those
failures. The latter uses an isolated bf92380 archive and separate target directory.
`round3/logs/cohort-spread-red.log` proves that the first needle cohort initially
covered only the proximal shoot base. The final full workspace gate reruns these
and the inherited seedling, sapling, expanding-crown, bole, ratio and survival tests.

The provisional aggregate upper-half leaf percentage is preserved in
`round3/provisional_form_test.rs`. It was replaced before committing with a
shoot-coverage test because cohort age affects aggregate leaf percentages. The
provisional structural-only lateral requirement likewise excluded local woody
branches, although those are part of the specified sapling form. No inherited
assertion, camera, shader or tolerance was weakened.

## Shedding evidence

Ordinary and the Two Trees retain the authored 0.45 threshold. Oak and spruce
retain their authored zero threshold. The common floor remains 0.75, applied to
exposure before the threshold test. Slice-start snapshots, identity order,
monotone radius records and the pipe-model fork split remain intact.

| Preset | Threshold | Mature age | Nodes | Placements | Height m | DBH m | H/DBH |
|---|---:|---:|---:|---:|---:|---:|---:|
| oregon-white-oak | 0 | 432 | 187,331 | 1,720,137 | 23.893020 | 0.836201333 | 28.573 |
| norway-spruce | 0 | 158 | 73,369 | 5,213,939 | 15.000000 | 0.426393152 | 35.179 |
| ordinary | 0.45 | 173 | 25,455 | 207,714 | 16.529927 | 0.927728835 | 17.818 |
| telperion | 0.45 | 173 | 142,591 | 1,094,995 | 98.845711 | 14.697044177 | 6.726 |
| laurelin | 0.45 | 173 | 165,285 | 1,199,211 | 111.998631 | 14.465549676 | 7.742 |

The fixture-presets survival tests assert retained mature crowns and a shaded
interior death stamp while a lit sibling survives. With the frozen synthetic
siblings, the shaded shoot (birth 1, NodeKey 2v1) dies at slice 174 and the lit
shoot (birth 2, NodeKey 3v1) survives on all three presets. The new run is recorded
in `round3/logs/survival.log`; the original failing floor test remains in `logs/`.

## Convergence and the re-pin

`CONVERGENCE.md` states node counts, crossover counts, placements and both bounds
for fn30, the rejected Round 2 state, and the current rule. It is written before
the one additional re-pin authorized by the owner in Round 3.

| Preset | fn30 nodes | Current nodes | Node deviation | fn30 placements | Current placements |
|---|---:|---:|---:|---:|---:|
| oregon-white-oak | 196,901 | 187,331 | -4.86% | 1,175,265 | 1,720,137 |
| norway-spruce | 76,386 | 73,369 | -3.95% | 5,463,221 | 5,213,939 |
| ordinary | 44,235 | 25,455 | -42.46% | 266,838 | 207,714 |
| telperion | 190,340 | 142,591 | -25.09% | 1,371,700 | 1,094,995 |
| laurelin | 194,517 | 165,285 | -15.03% | 1,240,525 | 1,199,211 |

Primary shoot planning now uses the pipe allocation before secondary thickening,
so juvenile thinness cannot permanently shorten the mature crown. The emitted
radii retain the annual secondary scale. Crown-room planning caps a future crown base at the branch's birth station,
which admits the young branch without granting permanent room below its attachment. The leader keeps its structural terminal;
other axes keep the local terminals that fill the crown. One existing lateral bud
can form a short leafy shoot without being allocated twice. Twig length fits
current room. Cohort sites interleave along the shoot, with stable identities.
Annual scheduling and preset values remain unchanged from Round 2.

Production skeleton, placement, mesh-count and bound pins move for those geometry
changes. Legacy envelope audit hashes and element hashes remain unchanged.
The Ordinary clay look reference moves with the grown geometry. The exact HELD
trait inventory, renderer cameras, shaders and drift thresholds remain unchanged.

## Mature build cost

The authoritative current cost is the median of the three samples per species
from one clean measurement window. The prebuilt test measures chronicle and wood
generation through growth, as fn30 did; it excludes foliage derivation, meshing
and rendering. `round3/logs/cost-idle.log` includes `nvidia-smi` and `top` before
the measurement and the exact command. No other task build or render runs in that
window. Desktop activity visible in those commands remains part of the record.

| Preset | Current idle median ms | fn30 ceiling ms | Difference ms | Half-second target |
|---|---:|---:|---:|---|
| oregon-white-oak | 6885.527 | 2463 | +4422.527 | missed |
| norway-spruce | 718.209 | 867 | -148.791 | missed |

Earlier Round 2 medians were oak 2,132.316 / spruce 2,483.966 ms in a contended
run and 611.682 / 716.808 ms in its idle retry. They measured the rejected smaller
crowns. `round3/ROUND2.md` and the original cost logs preserve those observations;
the current table above is the single result used for this round's R5 judgment.

## Captures

The final capture uses `target/release/examples/headless --preset <id> --seed 7
--age <years> --size 700x1000 --out .flow/evidence/fn31/strips/<id>-age-<years>.png`.
Year one also passes the generic `--frame-min-y 0` parameter to frame the seedling
above ground. Oak ages are 1, 10, 26.7, 56.1, 112, 200, 432; spruce ages are
1, 5, 14.1, 26.6, 36.9, 60, 158. Each strip uses `montage -tile 7x1 -geometry +4+0`.
Mature heroes use `--size 1600x1000`, seed 7 and no `--age`, then a beside-fn30 montage.

Four small 350×500 previews preceded the full capture; four images were viewed.
The full-capture budget is one run after the code checkpoint and zero full-capture
images viewed by the implementer. Commands and exits are in `round3/logs/capture.log`.
`CAPTURE.json` records image hashes and dimensions. Individual frames stay on disk.

| Preset | Seven-age strip | Beside fn30 strip | Mature comparison |
|---|---|---|---|
| oregon-white-oak | [strip](strips/oregon-white-oak-strip.png) | [strips beside fn30](oregon-white-oak-strips-beside-fn30.png) | [mature beside fn30](oregon-white-oak-beside-fn30.png) |
| norway-spruce | [strip](strips/norway-spruce-strip.png) | [strips beside fn30](norway-spruce-strips-beside-fn30.png) | [mature beside fn30](norway-spruce-beside-fn30.png) |

The first Round 3 gate attempt caught a taper inversion in the spruce–Telperion
4/9 blend: a resumed run computed its next tip from a base that had thickened
ahead of its attachment. New extension now bounds its tip by that attachment.
The unchanged sweep passes, and the recorded oak/spruce identity pins pass
without another move (`round3/logs/sweep-identity-final.log`). This correction
does not alter valid captured preset geometry. The first five gate results
remain in `round3/gates-attempt1/gates.json`.


## Gates

| Command | Exit code | Absolute log path |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round3/logs/fmt-final.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round3/logs/clippy-final.log` |
| `cargo test --release --workspace` | 101 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round3/logs/workspace-final.log` |
| `npm test` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round3/logs/npm-final.log` |
| `npm run typecheck` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round3/logs/typecheck-final.log` |

## Round 4: the young spruce, the distance fixture and the build cost

Round 4 is a host round; the Codex bridge is out until its quota resets on
2026-09-19, so Claude Opus implemented in this checkout. It took the three items
the Round 3 note left open and nothing else. Round 3's evidence above stands as
written; the numbers below supersede it where they overlap.

### The young spruce reads as a needled sapling (R1)

The five-year spruce was a bare stem and the fourteen-year one a bare whorled
scaffold. Neither was short of shoots to hang needles on: the cause was the
cohort schedule. `visible()` fills a shoot's stations over `leafLifetime` annual
cohorts, so with the spruce's authored 6.0 a shoot showed one sixth of its
needles in its first year and reached its full complement only in its sixth.
Every shoot on a young spruce is younger than that, so the whole tree was
starved while the mature tree - where every shoot is long past six years -
looked right. The trait is the fill schedule, not a retention window: nothing in
the rule expires a cohort. A spruce shoot flushes its needles in one season, so
the spruce joins every other preset at `leafLifetime` 1.0.

The second row is `shootStep`, the juvenile internode and twig length as a share
of current height. At the 0.2 default a half-metre seedling takes 10 cm steps,
so it holds about five internodes at any age and reads as a whip. Eight percent
is the juvenile shoot of a Picea abies seedling and gives the same tree 172
nodes instead of 44.

| Preset | Age | Nodes | Laterals | Structural laterals | Placements |
|---|---:|---:|---:|---:|---:|
| norway-spruce | 1 | 14 | 1 | 0 | 25 |
| norway-spruce | 2 | 49 | 31 | 0 | 125 |
| norway-spruce | 3 | 98 | 59 | 0 | 320 |
| norway-spruce | 5 | 172 | 92 | 0 | 1,095 |
| norway-spruce | 8 | 266 | 145 | 5 | 3,622 |
| norway-spruce | 10 | 416 | 229 | 20 | 11,546 |
| norway-spruce | 14.1 | 3,555 | 2,152 | 377 | 252,407 |

Against Round 3 that is 1,095 needle placements at year five where there were
184, and 252,407 at year 14.1 where there were 78,487. The oak is untouched by
both rows and its identity pin is byte-identical, so the ages the owner accepted
at 1, 10 and 26.7 years are the same tree.

One named deviation stands for the owner's judgment. At five years the spruce is
a needled leader with short lateral shoots, not yet a whorled cone. A whorl is
borne when the leader has grown `leaderInternode` (0.9 m), which a five-year
seedling has not, and a lateral born below the authored crown base finds no room
in the planning envelope, which is measured against the mature envelope while
`crownBaseRetention` is 1.0. Dropping the spruce's retention to 0 does raise
whorls from year four and lifts year 14.1 to 436k placements, but it also pulls
the 14.1-year trunk diameter from -3.77 to -13.0 percent against the composed
reference, and bounding the station spacing by the juvenile shoot as well takes
it to -21.3 percent, outside R2. Round 4 kept the diameters.

### Ratio and diameter at the fit ages, round 4

Oak rows are unchanged from Round 3; only the spruce moved.

| Preset | Age | Height m | DBH m | Reference DBH m | DBH deviation | H/DBH | Reference H/DBH | Ratio deviation |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| oregon-white-oak | 26.7 | 7.736033 | 0.057017819 | 0.064 | -10.91% | 135.677 | 125.0 | +8.54% |
| oregon-white-oak | 56.1 | 16.692183 | 0.126274552 | 0.112 | +12.75% | 132.190 | 142.8 | -7.43% |
| oregon-white-oak | 112 | 22.440165 | 0.221363284 | 0.236 | -6.20% | 101.373 | 101.8 | -0.42% |
| norway-spruce | 14.1 | 4.837733 | 0.102006861 | 0.106 | -3.77% | 47.426 | 47.3 | +0.27% |
| norway-spruce | 26.6 | 10.573251 | 0.230936299 | 0.203 | +13.76% | 45.784 | 49.3 | -7.13% |
| norway-spruce | 36.9 | 12.934402 | 0.286952263 | 0.285 | +0.69% | 45.075 | 52.6 | -14.31% |

R2 tolerance misses: none. The spruce ratio still falls with age - 47.4, 45.8,
45.1, and 35.2 at the derived mature age.

### The oak bark-distance fixture (diagnosed, not closed)

The fixture's mask is a fixed 50x100 pixel strip at the image centre, described
by its own comment as a trunk strip that excludes the silhouette. Rendering the
4x pair at fn30's task base and at Round 3's rule and dumping the masked crop
shows what changed: at fn30 the strip is solid trunk, and on Round 3's rule it
holds the trunk's right silhouette edge, open background beyond it, and several
branches crossing in front. The trunk is the same thickness in both - the
maximum node radius between 1 and 2 m is 0.42124 m in each - but its path
differs, so the fixed strip no longer sits inside it.

Two measurements separate the contributions. With the bark relief zeroed and the
same geometry the 4x mean is 2.078/255 of the 3.074 the measured fix below
reaches; the rest is the bark field the test exists to measure. And the wood in
the strip is far thinner than fn30's: the median node radius between 1 and 2 m
is 0.0058 m on Round 3's rule against 0.0188 m at fn30, and between 2 and 3 m
0.0035 m against 0.0106 m.

That thinness is a unit defect, and it is the cause the item asked for. The
planner reads the pipe allocation divided by the year's secondary scale and
writes every radius multiplied by it again, which cancels for a pipe-derived
width but not for `twig.diameter`, an anatomical width in metres. Every shoot
born while the oak's `juvenileRadius` holds the scale at 0.21 is therefore
written at 21 percent of its own twig anatomy, and the ratio frozen at birth
carries that fifth all the way to maturity. Flooring new wood at its twig
anatomy takes the fixture from 3.976417 mean / 14.50 p95 to 3.074167 / 11.75 at
4x, and from 2.019375 / 7.75 to 1.317708 / 4.75 at 2x - the p95 inside its limit
and the mean 2.5 percent over it.

Round 4 did not ship that floor, because it collides with R5. Wood born five
times thicker grows five times thicker, and the mature oak's radius keyframes
rise from 7,153,119 to 18,616,203 with the idle build at 9.46 s. The owner has
the choice: the anatomy fix and a much worse R5, or the present thin juvenile
wood and a red fixture. A third path, clearing the bole so the strip is trunk
again, was measured too - restoring a 0.45 shedding threshold on the oak sheds
its bole shoots but costs 35 percent of the mature crown, 187,331 nodes to
121,779, which is the density the owner asked to keep.

### Mature build cost (R5)

The keyframe eligibility index recorded one B-tree insert and a fresh allocation
for every annual radius frame of every shoot - 5.2 million of them on the mature
oak. It is an append-only vector now, sorted by width on the first read after a
run of appends, which is stable and therefore hands a reader the same order the
map did. Geometry is untouched: the same 7,153,119 frames, the same node counts,
the oak's identity pin byte-identical.

| Preset | Round 3 idle median ms | Round 4 idle median ms | fn30 ceiling ms | Difference ms |
|---|---:|---:|---:|---:|
| oregon-white-oak | 6885.527 | 4883.112 | 2463 | +2420.112 |
| norway-spruce | 718.209 | 653.491 | 867 | -213.509 |

Samples: oak 5024.022 / 4883.112 / 4402.334, spruce 677.191 / 653.491 / 649.403,
one clean window, the GPU at 0 percent and the CPU 95.7 percent idle before the
run. The spruce is inside its ceiling; the oak is not.

The stage profile says where the rest is. Of 4,669 ms, the annual keyframe stage
is 3,266 ms and local growth 1,186 ms; every other stage together is under 210
ms. The keyframe stage is 7.15 million frames and 10.26 million width visits,
each a handful of scattered slot-map and per-shoot-vector loads. The frame count
is not an implementation choice: it is total radial growth divided by the
0.0001 m resize tolerance, and R2's secondary thickening multiplies the growth
fn30 never had. fn30's own oak records 1,996,669 frames for a larger crown of
196,901 nodes, and its whole build profiles at 2,778 ms.

Three levers exist and none is this round's to pull. Raising the resize
tolerance is editing a tolerance to pass a gate. Thickening less fails R2. The
third is structural and is the recommendation: a local shoot's width is already
a pure function of its supporting structural width, its birth ratio and its
birth floors - `Widths::sample` computes exactly that - so 6.6 million of the
7.15 million frames are derivable and need never be materialized. Deriving them
at read time would leave only the 528,525 structural frames to record, but it
moves the change-record contract, where `resized_runs` is selected from recorded
frames, onto the read path. That is a spec of its own rather than a fix inside
this one.

### Round 4 captures

One full capture after the code checkpoint, one image viewed. Three 350x500
previews preceded it. `CAPTURE.json` records round 4's hashes and dimensions for
all 23 images; `round4/logs/capture.log` records the commands and exits, all 0.

### Round 4 gates

| Command | Exit code | Absolute log path |
|---|---:|---|
| `cargo fmt --all -- --check` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round4/logs/fmt.log` |
| `cargo clippy --workspace --all-targets -- -D warnings` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round4/logs/clippy.log` |
| `cargo test --release --workspace` | 101 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round4/logs/workspace.log` |
| `npm test` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round4/logs/npm.log` |
| `npm run typecheck` | 0 | `/home/daniel/Projects/telperion/.flow/evidence/fn31/round4/logs/typecheck.log` |

The workspace suite stops on one test, the oak bark-distance fixture diagnosed above.
Every other test in the workspace passes, including the unchanged preset sweep, the
survival and crown-retention fixtures and the moved spruce identity pin. `npm test`
is 77 of 77 with native-to-wasm parity unchanged.

## Blocked

The owner has not recorded a current R1 or R2 judgment; both slots below remain
empty. All six fit diameters are within 15 percent of fn30's composed
references.

R5 is blocked by the clean idle oak median of 4,883.112 ms against 2,463 ms, cut
from Round 3's 6,885.527 ms without moving geometry. The spruce is inside at
653.491 ms against 867 ms. The Round 4 analysis above names the three levers and
why none of them belongs to this round.

The oak distance fixture is still red at 4x: 3.976417/255 mean against 3.0 and
14.50/255 p95 against 12.0, unchanged, because the measured fix for it collides
with R5. The 2x result passes at 2.019375 mean and 7.75 p95. No camera, mask,
shader or tolerance moved. Every other renderer gate, both grazing masks and
their resolution comparisons included, passes.

## Owner verdict

### R1: sapling form and continuity


### R2: accepted diameter references and deviations


### Re-judgment of fn30's rejected strips
