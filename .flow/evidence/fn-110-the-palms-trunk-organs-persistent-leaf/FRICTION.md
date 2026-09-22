# Friction - fn-110, the palm's trunk organs

## 2026-09-22 - the design's spiral test named no frame to measure in

Writing R3's spiral test. The handoff asks for "the azimuth of base k about its
stem's axis equals `(rosette_fronds + k) * rosette_divergence` within a
tolerance". A palm's trunk wanders, so the axis a base leaves on is not the
axis its apex stands on, and the same radial reads 60 degrees apart in the two
frames: the first run of the test was 161 degrees off and told me nothing about
whether the placement was right. Three scratch runs went on finding out that
the measurement, not the placement, was what moved. Cost about fifteen minutes
and one design change - the crown's frame is now carried onto the local axis by
rotation rather than squared onto it, and the arithmetic is stated on a stem
grown straight, where the two frames are one.

What would have removed it: a verification line that names the frame a claimed
angle is measured in, whenever the surface it is measured on can tilt.

## 2026-09-22 - two size assertions carried a cap their own code had raised

Adding the eight dial rows. `tuning_engine.rs` asserts the proposal state is
under a literal 24,576 bytes in two places, while `PROPOSAL_CAP` next door is
32 KiB - fn-109 raised the constant and left the literals. The eight new rows
land at 24,683 bytes, so a test failed against a number the repo had already
superseded. Cost about ten minutes reading the fn-109 diff to be sure the
literal was stale rather than a second, tighter budget. Both assertions now
read `PROPOSAL_CAP`.

What would have removed it: a size assertion that reads the declared constant.

## 2026-09-22 - the generation-limit guard is only visible from a full gate run

The gate ran red on `generation_limit_guard`: five clamp-and-max sites in the
new module and two `0..3` column loops each needed a line in
`docs/generation-limits-inventory.json`. Nothing on the way there says a new
module owes the inventory an entry per numeric site, so the cost was a second
full workspace run, about six minutes, after the first had been green
everywhere else.

What would have removed it: naming `cargo test -p telperion-core --test
generation_limit_guard` in the design's verification table, or in the code
rules beside the 400-line rule, for any change that adds a module.

## 2026-09-22 — merge agent: master moved the table this branch had just widened

Merging `origin/master` into fn-110 conflicted in three files, all for the same
reason: PRs #55 and #58 lifted `CanopyParams` and `TwigPlacement` out of
`foliage/placement.rs` into a new `foliage/canopy.rs` and put `placement`,
`place_rosette` and the rest of the geometry exports behind
`#[cfg(feature = "geometry")]`, while this branch had added eight canopy rows,
their defaults and their rails to the file master emptied. Git could only
report the whole struct as taken on both sides, so nothing merged: the eight
rows and their defaults were re-applied by hand into `canopy.rs`, the
placement conflict resolved wholly to master, and `rosette.rs` re-gated row by
row — `frame` kept ungated and public because `branching::leaf_bases` is built
without the geometry feature and `tests/leaf_bases.rs` calls it, `spine` gated
because its only caller `fan` is. About 35 minutes, most of it deciding which
side of each cfg line the new code belonged on rather than transcribing it.
What would have removed it: nothing in the conflict itself — a move-only commit
(fields lifted verbatim to `canopy.rs`) landed separately from the cfg-gating
commit would have let git rename-follow the struct and leave only the gates to
judge, and either branch rebasing the moment the other's move landed would have
cut the window this sat open.
