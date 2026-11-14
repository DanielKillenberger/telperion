# The literature step retires stale decisions and cites the sentence an appearance came from

## Goal

On the palm's second pass (fn-80, 2026-09-23) quality passed crown width, frond and leaflet sizes, yet select blocked them on seven stale `requirements-unmet` decisions; and `verify` filed four `obligation-unmet` and two `claim-unsupported` on appearance values whose cited "sentence" was page navigation (A1's "rough gray" trunk sentence was never read). Owner (2026-09-23): "yes pls fix the frictions if they're obvious."

## Decisions

- Host design: `quality`, `select` and (R5, added by the host) `verify` call `retire_unfiled` for their stage after filing, as `gate` does; a superseded decision stays resolved.
- An appearance trait is judged one sentence at a time over sentences carrying its `terms` (new per-trait list in `species-requirements.json`) and cites the sentence its level came from.
- `verify` asks an appearance value `appearance_supported` instead of `measurement_not_invention`.

## Acceptance Criteria

- **R1:** A field that now passes has its stale decision superseded and select fills it — met.
- **R2:** `bark_colour` judged from A1's "rough gray" and cites it — met.
- **R3:** `verify` holds a supported appearance value and files a claim on an unsupported one; support question with labelled cases — met (13 cases, 4 held out, 5 negative).
- **R4:** Gate green — met (975 passed).
- **R5:** `verify` retires its own unrefiled decisions — met.

## Result

Landed in `abbb19e7`, `88f85dab`, `ad591973`, `9e858aea`; carried by the fn-149 rewrite (#121). `quality`/`select` key the retire guard on their header inputs (a select decision records only the manifest checksum).

## Open

None.
