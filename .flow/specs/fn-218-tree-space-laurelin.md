# Tree space: Laurelin

## Goal

Laurelin, the younger of the Two Trees, on the engine path per the host's brief (fn-198 VISION-TWO-TREES.md): on the beech's architecture, broad; young green leaves with a gold rim; hanging racemes of golden horn-shaped flowers; warm gold light falling as a golden rain; waxing as Telperion wanes. With the group off, a convincing old beech. Looks better than today's harness Laurelin (owner, 2026-10-07: full creative freedom for the Two Trees).

## Decisions

- Light, cycle and dew are after the release (fn-232), so no light-cycle strip here (host, 2026-10-08, fn-217 DESIGN.md section 6).
- Values: the `european-beech` rows at `/age` 80 (at 40 the beech is a 10 m sapling), then the supernatural group: scale 2.5 (at 3 the crown read twiggy, at 2 only a big beech), young green leaves with a thin gold rim, flowering 0.7 on age 14 only, 14 horn florets on a 0.24 m rachis with curl 0.45, gold petal albedo 0.75/0.32/0.015 (fn-234's 0.85/0.55/0.05 read lemon close up), amber eye.
- Judging: host confident and Astra agreeing on the same sheet. Astra disagreed in rounds 6 and 7 (whole crown too gold; yellow-green at distance) and agreed in round 8; its round-8 polish notes (10 % less rim, more raceme length variation) are optional and not applied.
- Owner's species verdict (2026-10-15, fn-237 R3 table): flowers too samey, not magical enough; accepted for the release: "we accept the flowers for now". fn-286's take-over 0.5 approved.

## Acceptance Criteria

- **R1:** Laurelin's values in the `laurelin` preset; builds natively and in the harness — met (native and Wasm node buffers byte-identical).
- **R2:** Sheets with the group off and on at whole, mid and close views, host confident and Astra agrees — met (Astra AGREE, round 8).
- **R3:** Built from fn-217's group and fn-234's petal albedo: racemes hang as gold horn strands, leaves green with gold edges at whole-tree view, scale chosen and contact error measured against fn-125's screen — met.
- **R4:** Gate and npm test green; build cost at her scale measured on the harness's native route — met; the device route's fixed cost hides her own.

## Result

- Off hashes to the beech's mesh at the same age (`supernatural::tests::the_two_trees_with_the_group_off_are_the_beech`).
- At scale 2.5, seed 1: 44.5 m tall, 43 x 48 m across.
- Contact at her scale (age 80, seed 42): 11.4M leaves, code step 1.36 mm, worst error 0.42 mm (0.50 % of the smallest leaf), inside fn-125's screen.
- Cost, CPU route: 8.3-12.8 s with 6.1M-11.4M leaves and 8.6M-14.8M florets, 2.1-2.5x the beech at 80. Device route about 140 s and 5.46 GB for any beech (fixed cost of the route).
- Host reading: off an open-grown old beech; on a fresh green crown with gold through it, laburnum-like strands of amber-throated golden horns, each leaf edged in gold.
- Gate fixes: conformance counts florets beside leaves; `validate` also judges natural rows an override stands in for. npm test 143 passed.

## Open

- Harness default seed 42 costs about 11 s on the CPU route; another seed or age for cost is the host's question.
- The device route's fixed cost (~140 s) makes her device cost unreadable.
- Owner: the flowers are too samey and not magical enough (accepted for now).

## Friction

- The command guard refused variable-path redirects and process substitution -> local setup; spell paths literally.
- The device route takes ~140 s for any beech, stage metrics print 0 -> reported to the host for a check of where device prepare spends its time.

## Evidence

- `.flow/evidence/fn-218-tree-space-laurelin/SHEET.md` (values table, Astra rounds, contact, cost; cited by `laurelin.values`)
