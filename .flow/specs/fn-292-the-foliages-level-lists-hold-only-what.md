# The foliage's level lists hold only what each level draws

## Goal & Context

fn-257 task 8 (2026-10-16, SPRUCE.md) named the 80.8 MB the spruce's kept tree charged without a label: the foliage's level lists (60.4 MB), which give every detail level room for every leaf, and their selection scratch (20.4 MB), both read every frame. Laid end to end, the levels would hold about 15 MB. The owner (2026-10-16): "if we can compress it further even better"; compact layouts are defined where the data is made, not converted at the end.

## Requirements

- **R1:** each detail level's list is sized from what it can hold and the levels laid end to end in one buffer (or the selection scratch shared where lifetimes allow), written in that layout where they are made; the foliage draws read the new layout on the device; the CPU path unchanged.
- **R2:** every preset's kept-tree memory before and after, labelled; frame time whole and close not worse; stills of every preset at its preset age unchanged to the eye, and the drawn leaves byte-identical per frame where the view is fixed.
- **R3:** gate and npm test green; codex review.

## Boundaries

Release item (fn-237), after fn-257.
