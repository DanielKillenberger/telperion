# The literature step judges what it read, and never passes a gap silently

## Goal

Companion to fn-130 from the host's whole-step audit of the palm's literature (2026-09-23). Once the stages read whole sources, screen, select, appearance, verify and quality still lost correct values (organ sizes classed `not_about_tree_size`, cultivar fronds dropped, "6–10m" unparsed), accepted wrong ones (spans credited to the wrong sentence, no confidence floor, levels averaged over unordered categories so zero-probability levels won), ignored decisions (`drop-value`, `replace-source` consumed by nothing; superseded decisions reopened), missed ages stated in sentences, passed gaps silently, and kept stale stage keys after code changed. Owner (2026-09-23): "yes pls fix the frictions if they're obvious."

## Decisions

Host design (2026-09-23):
- Screen gains organ-size (leaf, leaflet, frond, needle, cone) and cultivar classes; select reads per field the rows quality counted, with whole-word terms.
- Select: one document per field, spans keyed to sentence and source, a confidence floor from labelled cases, one number/unit grammar shared with the extractor.
- Levels are the most probable choice above a floor, never an average; `unstated` wins when most probable.
- Verify names the field, keys claims per pointer; `drop-value` removes the value and re-files the requirement; `replace-source` goes to fn-129's search; both routine.
- A superseded decision ignores a stale resolution; ages are parsed from sentences (`unstated` condition weighted 0.5); an unfillable required field files `requirements-unmet`; a stage key carries the build identity (digest of `src/` and `data/`).

## Acceptance Criteria

- **R1:** P4/P5/A1 leaflet and frond sentences reach select; M1 cultivar fronds count; "6–10m" parses — met.
- **R2:** Frond length credited to F1's frond sentence; a pick below the floor is not filled — met (F1 states only 20 feet: [6.096, 6.096]).
- **R3:** Appearance levels are the most probable or `unstated` — met.
- **R4:** `drop-value` leaves the packet and re-files; claims per pointer; the appearance claim can fail — met.
- **R5:** A superseded decision stays resolved — met.
- **R6:** A1's "reaching 5 m in 15 to 20 years" is a height point at 15-20 years — met.
- **R7:** Unfilled required field files `requirements-unmet`; `no_mature_size` fails a mature field — met.
- **R8:** A code change expires a stage's key — met.
- **R9:** New classes and floors have labelled cases and a no-match — met.
- **R10:** Gate green — met (1018 passed).

## Result

Landed in `adf16058`, `4379bfcc`; carried by the fn-149 rewrite (#121).

## Open

At close: the labelled live picks did not separate right from wrong (selection floor calibrated to 0.34 caught none of the wrong picks) — resolved by fn-133 (no floor until calibrated); sufficiency levels take no floor (no labelled set); height terms "reaches"/"grows to" also match diameter sentences.

## Friction

- Thin labelled sets for the new floors -> addressed in fn-133.
- The gate at ~500 s against the 600 s tool cap -> run it in the background.
