## Goal

Every leaf-on pair in fn-34 rendered the leaf mass far darker than its photograph (birch centre 35 against 83 under sun, beech 45 against 80 under overcast; 1% of leaf pixels above half brightness against the photograph's fifth), with leaf colours already matched. This spec diagnoses why a leaf facing the camera received almost no light and adds canopy lighting as rows, inside fn-29's frame budget, so the crowns read as lit masses.

## Decisions

- user (2026-09-15, on whether the look is achievable): "are we making good progress? optimistic we can achieve the look?"
- host (2026-09-15): "Even a perfectly shaped beech will look like a dark scatter until the canopy is lit as one mass."
- user (2026-09-15): "makes sense"
- worker, fn-46 round 8 (2026-09-15): "The leaves there sit in shadow. Even pure-white leaves reach only 78, or 81 with sky occlusion also off. Leaf pixels above half brightness are 1% of the crop in ours against 20% in the photograph ... Casting every leaf reads 31.8 against 32.0 ... Full transmission reads 35.9 ... What would reach it is ... finding out why the leaves facing the camera get no sun under a side sun."
- Diagnose first; a fault is fixed as a bug, not tuned around. No new light, no GI, no shadow-map change, no leaf geometry. Rows on the family only.
- The sheen stays on the face normal: a first version on the bent normal turned the crown's silhouette into a grey mirror.

## Acceptance Criteria

- **R1:** per-term diagnosis on S-WHOLE and B-WHOLE recorded with numbers; faults fixed — met (no fault found, no pin moved).
- **R2:** canopy rows railed, neutral at today's frame, refused by name, wire, blend, metadata, harness, native and browser — met (25 stills hash identical with rows at zero).
- **R3:** beech and birch rows set so leaf brightness approaches the photograph (within about 10); pairs rendered; visual QA; owner judges in fn-34 — partial: S-WHOLE centre 35 to 58.5 (photo 83), leaf pixels above half 2.4% to 17.7%; B-WHOLE 45 to 65.6 (photo 80). The rest is the birch's curtain wood in the centre crop and the beech's crown shape, leaf colour and the shot's exposure, outside this spec.
- **R4:** frame cost within fn-29's bound — met (oak 3.9836 ms p50 with every term on, against accepted 3.9823 and a same-session base 3.9785; browser orbits hold 60 fps).
- **R5:** tests for neutral identity, rails, each term on a synthetic leaf, native/browser agreement — met (`tests/canopy_terms.rs`, `tests/canopy_light.rs`, `tests/material_detail.rs`).

## Result

Diagnosis: shading normal, back face, single tone curve and the overcast row were all correct; each leaf was lit as a lone card. Transmission only along the sun's line (factor 0.03 at a 90° sun), no sky through or off a leaf, cameras below the crown see 66 to 79% undersides, and only 16% (S) / 7% (B) of leaf pixels both face the sun and are unshadowed. Five material rows in `canopy.wgsl`: `canopyNormal`, `lightWrap`, `diffuseTransmission`, `leafSheen`, `crownShade`. Beech 0.8/0.4/1.0/0.08/0.15, birch 0.8/0.5/1.0/0.06/0.2. Host read: the birch's crown now reads as one lit mass, bright on the upper shell and sunward side; the beech's leaves are lit but its crown does not read as one mass (its shape, fn-50). The owner accepted the silver birch at fn-34 round 25 (2026-09-16); the European beech's verdict moved to fn-62.

## Open

None.
