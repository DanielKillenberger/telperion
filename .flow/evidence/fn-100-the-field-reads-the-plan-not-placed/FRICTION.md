# fn-100 friction

## 2026-09-22 — R7's feature build reaches into the growth path

R7 asks the descriptor layer to build with the wood surface and the leaf
placement compiled out. The spec's architecture section says the layer
"compiles with no wood-surface and no leaf-placement code", which is true of
the layer, but the crate's direct build (`branching::generate`) runs through
`Specimen::grow`, and the specimen's timeline carries the growth path's
per-shoot leaf placement (`foliage::timeline`, which calls
`station::place_run` and builds an `AttachmentSurface`). Compiling placement
out therefore meant gating the specimen's aged reads (`read`, `read_at_age`,
`read_packed`, `advance`, `buffers`, `changes_between`) and the timeline's
placing methods behind the `geometry` feature, and moving `paths` (which the
specimen's contact tracking reads) out from under the builder. Cost: about
forty minutes, one cargo-check loop of nine errors, and about twenty `cfg`
attributes in the growth path that this spec's boundaries call untouched.

What would have removed it: the readiness check naming the modules that
`branching::generate` pulls in (`grep -rn "foliage::\|surface::"
crates/telperion-core/src/branching/specimen`), so the spec could either
scope R7 to "the plan and the field build without the surface builder and the
crown placement" or budget the growth-path gating explicitly.

## 2026-09-22 — the 8-tree forest no longer exists in the repository

R3 and Boundaries name "the 8-tree forest" for the wood byte comparison. No
script, fixture or harness in the checkout defines it (`grep -rn forest8\|
FOREST_COUNT\|8-tree` finds only fn-13 task text and CLAUDE.md). Cost: ten
minutes of searching. The comparison set used instead is eight single trees:
oak, birch and spruce at seeds 1 and 7, plus the ordinary and Telperion
presets at their own seeds; it is named in the evidence note.

What would have removed it: CLAUDE.md or the spec pointing at the file that
defines the forest, or saying it is gone.
