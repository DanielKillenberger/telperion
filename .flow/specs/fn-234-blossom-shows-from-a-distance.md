# Blossom shows from a distance

## Goal

fn-217 task 3 measured blossom at the whole-tree view: rasterisation keeps 92-96 % of the flowers' finest coverage, yet a beech flowering as densely as a cherry read as a green crown with grey speckle beside the cherry photograph's white mass, and tripling the florets raised visible coverage only to 2.9 % of the frame. Laurelin (fn-218) and Telperion (fn-219) need their blossom to read from the whole-tree view; so do real flowering trees. Host decision (2026-10-08): transferred from fn-217 R2 as its own capability spec. Blossom placement and visibility only.

## Decisions

- R1 measured (instrument `crates/telperion-render/src/generation/fine/blossom_measure.rs`): florets stand at the leaves' own depth; leaves hide 36-54 % of them; with no leaves at all the cherry-dense florets reach 5.4 % of the frame against a 6.2 % crown silhouette. The main loss was colour: a "white" petal at 0.1 drew sRGB (82, 92, 75), tinted by the leaves' transmission.
- **Host design for R2 (2026-10-09):** petals transmit their own colour in the organ block; petal colour is an albedo like the leaf colour (white about 0.8, gold about 0.85/0.65/0.1), the doc's "clip past 0.15" claim corrected; measure again before touching placement.
- **Host decision after R2 (2026-10-09), amending R3:** white blossom now shows across the whole crown on both executors. The remaining gap is phenology: *Prunus avium* blooms before leafing, so the photograph is flowers on bare twigs. No phenology row here (Laurelin and Telperion bloom in full leaf); "leaf presence while flowering" is its own post-release spec. Laurelin's straw-yellow is a values question for fn-218 (a deeper gold near 0.85/0.55/0.05).

## Acceptance Criteria

- **R1:** Where florets stand against the leaves that hide them, and the petals' rendered brightness at distance, measured and checked against code — met.
- **R2:** The fix the measurement names, chosen by the host, neutral by default, on both executors — met (petal's own transmission colour; albedo rows).
- **R3:** A cherry-dense tree shows a distinct white across the crown and Laurelin's racemes a distinct warm colour (amended), coverage before and after on both executors, CPU/device fixtures, neutral rows byte-identical, cost measured — met.
- **R4:** Gate and npm test green — met.

## Result

- Petals at the hero camera, cherry tree: (82, 92, 75) before, (168, 170, 168) after; close up (214, 215, 210). White share of the crown 0.015 -> 0.162 (dense tree 0.228, photograph 0.219). Laurelin's gold share 0.000 -> 0.196.
- `tests/petal_white.rs` red on the base (backlit petal (126, 193, 139)), green after ((188, 190, 188)).
- `blossom_parity::the_flowering_tree_draws_alike_on_both_executors`: 461,958 florets, coverage 0.02220 CPU / 0.02222 device, petals within 2 sRGB. Shipped presets without flowers render byte-identical (beech, oak, spruce CPU; oak device).
- Cost: no WGSL, draw or buffer added.
- Stills viewed by the host beside `reference/prunus-avium-hohenwettersbach.jpg` (Wikimedia Commons, H. Zell, CC BY-SA 3.0).

## Open

- Phenology ("leaf presence while flowering"): post-release spec.
- The measurement's depth quartiles leave about 0.8 % of florets in no band (review P3).
- `blossom_parity` ran over 60 s under the gate's parallel load (about 5 s alone).
- The device's tree reaching 0.23 m below ground: recorded for fn-198's release checks.

## Friction

- Shell guard refused variable-path and new-directory redirects -> write logs to the scratchpad with literal paths.
- The cherry photograph lived only in fn-217's ignored `raw/` -> tracked here as `reference/prunus-avium-hohenwettersbach.jpg`.
- A shipped beech took 219 s on the device path in `generation_gpu` (one cold run) -> a host check of device prepare time for shipped presets.

## Evidence

- `.flow/evidence/fn-234-blossom-shows-from-a-distance/reference/prunus-avium-hohenwettersbach.jpg` (the whole-crown reference the verdict compared against)
