# A young palm's fronds grow to their size over its establishment

## Goal & Context

The owner (2026-10-14), watching the harness grow the date palm in: "also the date palm starts with full grown leaves". The rosette (`crates/telperion-core/src/pipeline/foliage/rosette.rs`) draws every frond the crown has made at the mature frond's length from year 1 (fn-270 R5b' made the first-year crown draw only the fronds it has made, not their size). A palm establishes: its first leaves are small and each new leaf is larger than the last until the crown reaches its mature size, and only then does the stem rise (establishment growth; whether the date palm's literature states it in these terms, and over how many years, is unknown until sourced). Growth is the product (AGENTS.md), so the grow-in shows a young palm wrong today.

## Requirements

- **R1 (sources first; stop on failure):** the date palm's establishment from the literature: the length (or a share of the mature length) of its early leaves at a stated age, the years until the crown's fronds reach mature size, and whether the stem's rise waits for it; each value checked against its source text before it reaches the preset (docs/typesafe.md for screening). A row with no source is a declared stand-in, named as such. Stop and report if the sources contradict the shape below.
- **R2 (one law, by degree):** each frond's size is set by the crown's age when that frond was made, along a sourced curve from the early share to 1 over the establishment years; a frond keeps the size it was made at (old fronds never grow or shrink). Every rosette species reads the same rows; a tree with no rosette is untouched (dormant). No species branch.
- **R3 (today's mature palm):** at the palm's shipped age, once every living frond was made after establishment, the drawn tree is byte-equal to today's; if the sources put establishment past the shipped age, the change is measured and shown, not hidden.
- **R4 (both executors):** the CPU and the device agree, as fn-270's rosette tests check them now; the year boundary shows no frond changing size.
- **R5 (proof):** stills of the palm at ages 1, 3, 6 and its shipped age, and the grow-in recording at whole years (fn-270 host ruling: whole years, crossfaded), viewed; the owner's verdict.

## Boundaries

Release item (fn-237), after fn-270. Juvenile leaf form (entire eophylls before pinnate leaves) is out of scope unless the sources make it the dominant read at the ages shown; then it is reported, not built.
