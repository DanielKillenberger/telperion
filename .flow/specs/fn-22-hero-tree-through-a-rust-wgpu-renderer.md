# Hero tree through a Rust wgpu renderer

## Goal

After fn-13 grew into a 230-commit TypeScript prototype, the owner asked for "more manageable specs that can be reliably completed", composing "to a generator and renderer that's fast and responsive and has the fidelity of basically real trees". This first spec of the series puts a lean Rust wgpu renderer on master that draws one hero tree from generator output in the browser, with the parameter panel live, and in a headless native target for timing and stills; the Three.js renderer is removed so the repo has one renderer.

## Decisions

> user (turn 2): "And as i said no special code for tree templates it should all be parameterized and certain trees should just be configurations of the generator."
> user (turn 3): "each new species shouldn't necessarily be values only. If the generator isn't capable to produce a palm tree yet with parameters we need to extend the generator to allow for palms to be generated. Once that's in we the palm template is just a list of numbers yes."
> user (turn 4): "are we sure we want to merge this code? i want the code to be lean as possible. Elegant."
> user (turn 5): "should we also while we're at it port the renderer to wgpu in rust?"
> user (turn 8): "i don't think we need pixel perfect parity from the old renderer. We'll have to iterate on the generator and renderer anyway."
> user (turn 9): "well we should have the panel though? I do want the renderer in browser with generator parameters being changeable in browser?"
> user (edit cycle 1): "The nice thing would be is if you could import telperion lib and have trees generated in your engine that look good and perform well."
> user (edit cycle 1): "so i don't want to have to preexport trees"
> user (plan, 2026-09-08): "wouldn't it make sense to just rewrite to rust completely so we can delete the three.js stuff?"

- Rewrite from scratch in Rust; the fn-13 branch stays an unmerged reference prototype (two ideas ported: clay hemisphere lighting and the GPU timing protocol).
- A template is only a row of parameter values; where the generator cannot express a species, add a general parameter, never a species branch.
- Owner's-eye sign-off over image parity; neutral clay scene; inline generation; panel stays React; native target is headless only; the fn-19 benchmark rig is retired with a note.

## Acceptance Criteria

- **R1:** Browser page renders a hero oak or spruce through the Rust renderer with every parameter live and three views; failures visible — met.
- **R2:** Native headless target renders a still from preset id and seed — met.
- **R3:** Renderer draws the core's mesh in place; counts and bounds equal the core's; oversize rejected — met.
- **R4:** Every preset and twenty random parameter sets through one path, no species branch — met.
- **R5:** GPU timing native and browser with conditioning and validity verdicts recorded — met.
- **R6:** Five-minute idle soak keeps one canvas and one device — met.
- **R7:** Owner verdict on one still per species — accepted.
- **R8:** Three.js removed; rigs run on the Rust renderer — met.

## Result

Hero vegetation GPU at seed 7, 1600x1000, RTX 3080: oak 17.87 ms native / 17.86 ms browser; spruce 94.18 ms native / 62.91 ms browser, all valid. A resolution probe showed the spruce frame is vertex-bound (7.9 M needles of a 56-triangle element; wood alone 0.71 ms), which handed fast-hero (fn-23) its target.

Owner verdicts (2026-09-09):
- Oak: "it looks good, almost the same as the previous render in JS, just slightly darker."
- Spruce: "reads as a spruce."
- Framing decision: "the camera is too far away in these stills; tighten the hero pose." The hero pose now frames the crown's inscribed ellipsoid with a 1.15 margin; stills and native records were re-rendered.

## Open

Parked for the placement-path successor: the density dial is ignored by the spreading and tiered habits; the oak-named lobed outline and a scaffold audit naming presets remain in the core; the panel's failure flows lost automated coverage; QA stills carry the ground disc and scale figure; the spruce browser/native gap.

## Evidence

- `.flow/evidence/fn22/REPORT.md`
- `.flow/evidence/fn22/oak-hero.jpg`, `spruce-hero.jpg` (verdict stills)
- `.flow/evidence/fn22/*-timing.json`, `soak.json`
