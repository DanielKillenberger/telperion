# Real-species profiles and procedural templates

## Goal

Research real species, freeze botanical profiles for them, and build procedural templates whose every seed reads recognisably as that species, geometry and foliage first. Initial scope: one broadleaf and one conifer (chosen: Oregon white oak, Norway spruce), with supernatural attributes separated from natural ones and off by default (owner, 2026-09-05/06).

## Decisions

Owner (verbatim):
> E1: "yes i'd agree with fidelity first"
> E2: "it'll also require procedural leavees right? to make needles and different shapes and stuff"
> E3: "alright i think the spec should be first finding species and reference images to work off of and then for each species create a template where every seed looks recognizably like the reference species."
> E4: "also figure out the actual \"specs\" of a tree. And see if we're in the right ball park in terms of branches and leaves, height etc. etc."
> E5: "so each species has a profile that we work towards with the template"
> E6: "I think we should have all \"supernatural\" attributes be separated from natural ones. and by default have them off."
> E7: "ofc the supernatural templates will have the supernatural traits on.."

- Profiles (cited ranges, definitions, confidence) stay separate from runtime family parameters; Rust is the source of behaviour; no runtime rules language.
- Natural tropism stays botanical; authored writhe/spiral form a separate supernatural group, neutral on ordinary templates, enabled on Telperion/Laurelin.
- Protocol: 12 fixed + 12 fresh seeds per species; targets frozen before tuning; failures retained, never resampled.
- Plan review omitted at the owner's request.

## Acceptance Criteria

- **R1:** Two contrasting species selected with attributed whole-tree, branching and foliage references — met (`.flow/evidence/fn9/REFERENCES.md`).
- **R2:** Anatomy and target ranges defined and generated measurements compared in matching units — met (48/48 cases pass gating checks).
- **R3:** One named template per species through native and browser selection; seeds vary while keeping identity; supernatural separate and off on ordinary templates — met.
- **R4:** Procedural foliage supplies each profile's leaf/needle geometry and attachment — met (lobed alternate oak blades; four-sided spruce needles on pegs).
- **R5:** Fixed/fresh seed protocol in neutral views with numeric and visual results recorded separately — met (84 captures, 59 inspected; 35 numeric-only unassessed).
- **R6:** Shared generation limitations corrected and natural/supernatural templates rechecked — met (occupancy transforms replaced by retained crowns; flat structural tips fixed).

## Result

At production `0a8f343`: oak height 16.4-20.4 m, crown 20.7-24.5 m, 480-585 k leaves; spruce height 15.0 m, crown 8.3-8.9 m, 7.5-8.0 M needles. 68 native tests, 106 harness tests, Clippy, typecheck and browser suite green. Implementation review SHIP (same model); spec-completion review never ran (default main ref unresolved, then stopped after the owner questioned same-model review). Owner, 2026-09-06, on the served spruce viewer: "but good job on the implementation looks good" — implementation feedback, not per-seed approval. Earlier seven passes archived in `experiments/fn9-iterations/`.

## Open

- No owner per-seed approval; completion review unassessed.
- Lifecycle, GPU, LOD, wind and engine adapters belong to their follow-up specs.

## Evidence

- `.flow/evidence/fn9/REPORT.md` (completion report)
- `.flow/evidence/fn9/REFERENCES.md`, `profiles.json`, `seeds.json` (frozen targets; loaded by CI and tools)
- `.flow/evidence/fn9/final/preview/norway-spruce-1-whole.png` (referenced by replay fixtures)
