# fn-35 friction

## 2026-09-18 — the ash run is on an unmerged branch

Assembling `catalogue/european-ash/` meant reading the fn-56 pipeline run's
manifest, provenance, decisions and resolutions, which exist only on
`fn-56-european-ash-as-a-real-species`. Every artifact came out through a
separate `git show <branch>:<path>`, and the spec's own "Resolved via Codebase"
section named the wrong branch (`pipeline-run`, which carries none of it).
Cost: about ten minutes of branch archaeology.

What would have removed it: nothing in the repository — a spec that moves
another spec's in-flight evidence should say which branch holds it and whether
the move waits for that spec to land. Worth the owner's word before the next
one, because the duplicate reappears when fn-56 merges.

## 2026-09-18 — serde_json parses a seventeen-digit bound a bit off

The identity pins compare bit for bit against the same literals in test source.
`serde_json` without `float_roundtrip` parsed `-13.163122928115051` one ULP
away from the Rust literal, so the first run of the new pins test failed on a
difference that was not a difference. Cost: about fifteen minutes to tell a
real disagreement from a parser artefact.

What would have removed it: `serde_json`'s `float_roundtrip` feature on from
the start wherever the crate reads numbers a test compares exactly. It is on
now for `telperion-core`'s dev-dependency; the same trap is open in
`telperion-jev`, which compares fitted curve values.

## 2026-09-18 — the workspace suite outruns the tool timeout

`npm run rust:test` did not finish inside the 600-second limit on either the
baseline or the verification run, because a second session was compiling the
same workspace at the same time. Each gate run therefore costs a background
hand-off and a round-trip rather than a straight answer. Cost: two round-trips,
about twenty minutes of wall clock.

What would have removed it: fn-76 (the test suite under three minutes) is the
standing fix; nothing new is needed here beyond finishing it.

## 2026-09-18 — every stage took a bare directory

Taking the run's scratch out of the catalogue meant giving `Paths` a second
directory, and because every stage's entry point took `dir: &Path` rather than
the `Paths` it immediately built, the change reached forty call sites across
the crate and its three test files. Cost: about twenty-five minutes, almost all
of it mechanical.

What would have removed it: passing the value the stage actually uses. The
stages now take `&Paths`, so the next path that belongs to a run rather than to
a species is one field.

## 2026-09-18 — the local command guard blocked three ordinary shell forms

`dcg` refused a redirect to a path held in a variable, an `rm -rf` of a
directory this task had just generated, and a heredoc whose text contained
backticks inside a Rust doc comment. Each cost a retry in another form. Cost:
about five minutes.

This is a local setup matter on the owner's machine, reported and not specced.
