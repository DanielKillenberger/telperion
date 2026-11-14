# The literature step reads mature sizes, keeps appearance sources, and stops first

## Goal

The palm's first pass through fn-118's requirements (fn-80 live run, 2026-09-23) filed seven `requirements-unmet` and four `structural-unmet` decisions although its sources held the data (A1: leaflets "½ m (18 inches) long", fronds "5-7 m", "Width: 20 - 50 feet"). Three defects: mature sizes were judged as age-indexed curves and scored `none`; four appearance values were copied with no source, which `verify` refused; and the conductor went past the owner's stop to the gap loop. Owner (2026-09-23): "yes pls fix the frictions if they're obvious."

## Decisions

- Host design: the requirements table marks crown width and leaf, needle, frond and leaflet sizes `mature`; a mature field needs no age and is judged by a mature-size question set with labelled cases and a no-match answer. Height and trunk diameter stay age-indexed.
- `select` records the chosen span's source on every appearance value; no source, no value.
- The conductor acts on an open owner-only decision before the gap loop, stages or tuning.

## Acceptance Criteria

- **R1:** A red test on A1 reproduces `none`; after the fix a mature field reaches `proxy_only` or better — met (leaflet length, crown width, frond length `partial`).
- **R2:** Every appearance value carries its source and `verify` holds — met.
- **R3:** With `requirements-unmet` open, the conductor pauses for the owner — met.
- **R4:** Mature-size question set with labelled cases and no-match — met (14 cases, 4 held out, 4 negative).
- **R5:** Gate green — met (958 passed).

## Result

Landed in `28341393`; carried by the fn-149 rewrite (#121). Each R test was red on base.

## Open

Follow-ups noted then (taken up by fn-128 and later runs): `select` admits only size-classified screen rows, so a mature field passing quality may still be unavailable; appearance spans now reach verify's citation checks; `crown_base_m` stays age-indexed.
