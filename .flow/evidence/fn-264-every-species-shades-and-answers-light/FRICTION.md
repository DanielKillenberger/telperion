# fn-264 friction

## 2026-10-12, worker: flowctl not on PATH

- **Doing:** reading the spec with `flowctl cat` from F4.
- **Slowed:** `flowctl: command not found` in the dispatched shell; found by searching the plugin cache.
- **Cost:** about 1 minute.
- **Would remove it:** the dispatch naming the flowctl path, or the plugin's `scripts/` on the agent shell's PATH.

## 2026-10-12, worker: `jev select` cannot recall a coefficient or an area

- **Doing:** R1a, Jev screening the extinction coefficient in Bréda (2003).
- **Slowed:** `jev select`'s recall regex (`quantity::unit_re`) recalls lengths and ages only, so a dimensionless k ("k equals 0.5") or a leaf area in cm² is never a candidate and Jev can only answer `none`. Worked around with `jev ask` over a candidate list a scratch script extracts (every number span in the passage) and a choice with a `none` answer.
- **Cost:** about 10 minutes, and a scratch extractor outside the crate.
- **Would remove it:** `select --units area,ratio` (or a numbers-only recall mode) in `telperion-jev`, with its coverage case in `data/cases`.
