# The tree grows in while it loads

## Goal & Context

The owner (2026-10-13): "when growing a mature tree we do generate a young tree right? could we as the tree loads in render each young version?" The device cycle grows year by year, so each cycle holds the tree on its way to the requested age. Shown as it builds, a sapling grows into the mature tree over the build's second and hides its latency. This overrides AGENTS.md's "growth is never the default view" for the load only, on the owner's word. Two facts shape it: the settle (pipes, sag, leaf placement) runs once at the end (about 60 ms GPU on Telperion), and retirement drops subtrees that will not survive to the requested age, so a frame at year k is the tree on its way to the final age, not the tree grown to age k.

## Requirements

- **R1:** while a build runs, the harness and the site draw a frame of the growing tree every few cycles (a stated cadence), each through a settle the frame budget allows (the full settle where it fits, a stated cheaper finish otherwise), the final frame the mature tree exactly as today; no second grower.
- **R2:** frame cost and the build's added time measured per preset; the total build no more than 10% slower than without frames.
- **R3:** a recording of Telperion and the oak growing in, viewed; the owner's verdict.

## Boundaries

The load animation. After the release (owner, 2026-10-13: "i'd really like to get the release done").
