# Comparative botanical geometry benchmark

## Goal

The owner asked for Telperion to "become sota" and for a species template so the catalogue can grow "specimen after specimen for new species in parallel". This spec built the evidence to judge that: a frozen benchmark protocol comparing Oregon white oak and Norway spruce against real references and competing generators (whole crown, bare structure, trunk base, fork, attached shoot), structural measurements, a baseline assessment for the woody-anatomy (fn-20) and developmental-shoot (fn-21) work, revision replay, and a reusable species-onboarding workflow.

## Decisions

> user: "i want this to become sota. i think we can do it. check current uptodate state and screenshots and open specs and evaluate what's needed to get there"
> user: "yes having a template on how to expand our species catalogue would be excellent i want to add a pipeline where we just start pumping out specimen after specimen for new species in parallel"
> user: "so very good make that explicit."

- Oak and spruce are the first cohort, not a catalogue limit; a template is a botanical family, a specimen one seeded realization, and every admitted species shows several specimens.
- Freeze the protocol before tuning: 12 mature specimens (historical seeds 1-3 plus three newly frozen unused seeds per species); once inspected they are regression cases, never fresh holdouts.
- Reference observations, generated measurements and reviewer judgments stay distinct; an absent value is never zero; unsupported measurements stay unavailable rather than passing.
- No superiority claim without matched comparators and independent botanical assessment; no reviewer contacted, no assets bought.
- Costs are separate domains and inconclusive without an exclusive measurement window; native `generation` is skeleton branching only.
- Production generation untouched; geometry fixes belong to fn-20/fn-21.

## Acceptance Criteria

- **R1:** Matched reference sets per anatomical scale, real and competitor, with gaps explicit — met: qualitative references for both species; trunk base, close oak forks and bare spruce remain reference gaps; no matched competitor asset exists.
- **R2:** Reproducible views, held-out seeds and structural measurements, frozen before tuning — met (protocol fn19-v1).
- **R3:** Baseline assessment with ranked discrepancies and independent botanical assessment — partial: engineering baseline and ranked findings delivered; independent assessment unassessed.
- **R4:** Revision comparison without hiding failures; costs separate — met; costs inconclusive.
- **R5:** Reusable onboarding template, parallel-agent workflow, two packets and a later-cohort admission — met (Scots pine fixture stays `unsupported-anatomy`).

## Result

All 12 mature cases measured and all 84 required views captured and inspected (0 failed). Oak 17.6-19.6 m tall, 495,781-585,533 leaves; spruce 15.0 m, 7.5-8.0 M needles. Ranked findings handed to fn-20/fn-21: FN19-G01 smooth rotational trunk flare without buttresses (high), FN19-V01 foliage hides forks/shoots (high), FN19-G02 plug-like fork shoulder (medium), FN19-D02 regular spruce tiers (medium), FN19-D01 no developmental evidence (medium). A separate visibility-v2.2 diagnostic exposed local anatomy in 24 admitted views. Replay of a small cohort reproduced all metrics and images with zero drift. Completion review SHIP (round two). The report is `.flow/evidence/fn19/REPORT.md`.

## Open

- Independent botanical assessment (R3) and matched competitor comparison remain open; no SOTA verdict.
- Qualified timing needs an exclusive measurement window.

## Evidence

- `.flow/evidence/fn19/REPORT.md`, `PROTOCOL.md`, `protocol.json`, `references.json`, `REFERENCES.md`
- `.flow/evidence/fn19/final/` (numeric, sources, costs, inspection, gates, controls and review-packet receipts)
- `.flow/evidence/fn19/visibility-v2/protocol.json`
- `.flow/evidence/fn19/onboarding-examples/` (packets and `verify.py`)
