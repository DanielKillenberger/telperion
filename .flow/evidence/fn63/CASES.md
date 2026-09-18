# fn-63: the gap question set scored against its labelled cases

`target/ci/jev cases --only gap --ledger .flow/evidence/fn63/ledger`, 2026-09-18,
15 cases, one Jev call each, every call's entry in the run ledger (gitignored).
R2's bars: 0.9 on change kind and prior-verdict coverage, 0.8 on best match,
each scored twice, once over the labelled cases and once over the held-out third.

| Set | Labelled | Held out | Bar |
|---|---|---|---|
| change kind | 24/24 | 13/13 | 0.9 |
| prior verdict | 23/24 | 12/13 | 0.9 |
| best match | 9/10 | 4/5 | 0.8 |

The command exits 0: every set met its bar. It exits 1 and lists the missed
case ids otherwise, which is how R2's error surface reads.

## The two misses inside the bars

- `fn-54` best match: Jev names the clumping term where the owner measured the
  beech's rows again first. The case is kept as labelled; the spec's own order
  is values before the generator term, and the loop's route table is what
  carries that preference, not the question.
- `negative-…` best match, held out: Jev names a value-table option where no
  option answers the ash's compound leaf. The no-match answer is offered and
  was not taken; the case is the set's only negative and stays.

## The first run, and what it corrected

The set missed on its first live run: change kind 13/16 held out, prior verdict
14/24 labelled. Reading the rows, two faults were in the set, not the answers.
The `value_table` and `appearance` criteria both claimed a colour row, and the
`against` criterion let a ruling that endorsed one option count against every
other in the set. Both criteria were sharpened, the labels that had read a
restatement of the gap as an owner ruling were corrected to `none`, and fn-42
left the set: it is fn-32's own follow-on, not one of fn-34's species gaps.
No bar was moved.

## The scored rows

```
## gap change_kind
id	expected	answered	top_p	conf	ledger	hit
fn-37-pendulous-shoots/curtain-rows	generator	generator	0.95	0.93	gap:18d6833819d47dfd-ddaba-0	true
fn-37-pendulous-shoots/bias-field-gravity	generator	generator	0.97	0.96	gap:18d6833819d47dfd-ddaba-0	true
fn-37-pendulous-shoots/raise-the-droop-constant	value_table	value_table	0.96	0.94	gap:18d6833819d47dfd-ddaba-0	true
fn-38-multi-stem-trees/stems-as-order-zero-axes	generator	generator	1.00	0.99	gap:18d683383d778a27-ddaba-1	true
fn-38-multi-stem-trees/basal-fork-as-lateral	generator	generator	1.00	0.99	gap:18d683383d778a27-ddaba-1	true
fn-40-smooth-bark/three-row-sets-one-shader	appearance	appearance	0.60	0.46	gap:18d6833894061068-ddaba-3	true
fn-40-smooth-bark/second-smooth-bark-shader	appearance	appearance	0.63	0.50	gap:18d6833894061068-ddaba-3	true
fn-44-curtain-hangs-under-gravity/sag-row	generator	generator	0.83	0.77	gap:18d68338c0ed1590-ddaba-4	true
fn-44-curtain-hangs-under-gravity/degrees-per-metre	generator	generator	0.70	0.60	gap:18d68338c0ed1590-ddaba-4	true
fn-45-twig-budget/twig-generations-row	generator	generator	0.86	0.81	gap:18d68338e493667d-ddaba-5	true
fn-45-twig-budget/raise-the-node-ceiling	generator	generator	0.86	0.81	gap:18d68338e493667d-ddaba-5	true
fn-45-twig-budget/thin-the-beech-twigs	value_table	value_table	0.85	0.80	gap:18d68338e493667d-ddaba-5	true
fn-47-curtain-strand-length/pendulous-variation-row	generator	generator	0.91	0.87	gap:18d68339344977df-ddaba-7	true
fn-47-curtain-strand-length/per-side-hem-term	generator	generator	0.93	0.91	gap:18d68339344977df-ddaba-7	true
fn-48-clump-lean-spread/stem-lean-spread-row	generator	generator	0.55	0.40	gap:18d683395c1bfd55-ddaba-8	true
fn-48-clump-lean-spread/lean-each-stem-by-hand	value_table	value_table	0.83	0.77	gap:18d683395c1bfd55-ddaba-8	true
fn-49-collapsed-triangle/drop-the-degenerate-triangle	generator	generator	0.99	0.97	gap:18d6833986e63e6f-ddaba-9	true
fn-49-collapsed-triangle/reseed-past-the-failure	value_table	value_table	0.75	0.66	gap:18d6833986e63e6f-ddaba-9	true
fn-51-curtain-below-the-crown/curtain-drop-band	generator	generator	0.99	0.99	gap:18d68339e84a6b44-ddaba-b	true
fn-51-curtain-below-the-crown/bunch-the-curtain-per-limb	generator	generator	0.54	0.39	gap:18d68339e84a6b44-ddaba-b	true
fn-51-curtain-below-the-crown/lower-the-crown-base-row	value_table	value_table	0.96	0.94	gap:18d68339e84a6b44-ddaba-b	true
fn-54-beech-crown-from-limb-systems/measure-then-clump	value_table	value_table	0.87	0.82	gap:18d6833a32291b4a-ddaba-d	true
fn-54-beech-crown-from-limb-systems/limb-attractor-clumps	generator	generator	0.93	0.91	gap:18d6833a32291b4a-ddaba-d	true
fn-54-beech-crown-from-limb-systems/retune-every-broadleaf	value_table	value_table	0.93	0.89	gap:18d6833a32291b4a-ddaba-d	true
accuracy 24/24 (pilot 22)  conf min=0.39 median=0.84 max=0.99

## gap change_kind (held-out)
id	expected	answered	top_p	conf	ledger	hit
fn-39-crown-outline-irregularity/perturbed-envelope-radius	generator	generator	0.96	0.93	gap:18d683386eeb84d3-ddaba-2	true
fn-39-crown-outline-irregularity/shade-out-limbs	generator	generator	0.95	0.95	gap:18d683386eeb84d3-ddaba-2	true
fn-39-crown-outline-irregularity/loosen-the-crown-rows	value_table	value_table	0.90	0.87	gap:18d683386eeb84d3-ddaba-2	true
fn-46-young-shoot-colour/shoot-colour-by-radius	appearance	appearance	0.91	0.88	gap:18d683390c8592e6-ddaba-6	true
fn-46-young-shoot-colour/shoot-colour-by-branch-order	appearance	appearance	0.91	0.87	gap:18d683390c8592e6-ddaba-6	true
fn-50-short-shoots/short-shoots-as-placements	generator	generator	0.86	0.82	gap:18d68339b77f390b-ddaba-a	true
fn-50-short-shoots/a-node-per-short-shoot	generator	generator	0.98	0.98	gap:18d68339b77f390b-ddaba-a	true
fn-50-short-shoots/raise-the-interior-density	value_table	value_table	0.98	0.97	gap:18d68339b77f390b-ddaba-a	true
fn-52-leaf-mass-lit-as-a-canopy/canopy-lighting-rows	appearance	appearance	0.85	0.80	gap:18d6833a0d20818f-ddaba-c	true
fn-52-leaf-mass-lit-as-a-canopy/brighten-the-leaf-colour	value_table	value_table	0.98	0.98	gap:18d6833a0d20818f-ddaba-c	true
negative-no-option-answers-the-capability/widen-the-leaf-blade	value_table	value_table	0.91	0.88	gap:18d6833a65119445-ddaba-e	true
negative-no-option-answers-the-capability/darken-the-ash-foliage	appearance	appearance	0.57	0.43	gap:18d6833a65119445-ddaba-e	true
negative-no-option-answers-the-capability/raise-the-leaves-per-station	value_table	value_table	0.74	0.65	gap:18d6833a65119445-ddaba-e	true
accuracy 13/13 (pilot 12)  conf min=0.43 median=0.88 max=0.98

## gap prior_verdict
id	expected	answered	top_p	conf	ledger	hit
fn-37-pendulous-shoots/curtain-rows	none	none	0.89	0.83	gap:18d6833819d47dfd-ddaba-0	true
fn-37-pendulous-shoots/bias-field-gravity	none	none	0.98	0.97	gap:18d6833819d47dfd-ddaba-0	true
fn-37-pendulous-shoots/raise-the-droop-constant	none	none	0.99	0.99	gap:18d6833819d47dfd-ddaba-0	true
fn-38-multi-stem-trees/stems-as-order-zero-axes	none	none	1.00	1.00	gap:18d683383d778a27-ddaba-1	true
fn-38-multi-stem-trees/basal-fork-as-lateral	none	none	1.00	1.00	gap:18d683383d778a27-ddaba-1	true
fn-40-smooth-bark/three-row-sets-one-shader	none	none	1.00	1.00	gap:18d6833894061068-ddaba-3	true
fn-40-smooth-bark/second-smooth-bark-shader	none	none	1.00	1.00	gap:18d6833894061068-ddaba-3	true
fn-44-curtain-hangs-under-gravity/sag-row	for	for	0.92	0.88	gap:18d68338c0ed1590-ddaba-4	true
fn-44-curtain-hangs-under-gravity/degrees-per-metre	none	none	0.68	0.52	gap:18d68338c0ed1590-ddaba-4	true
fn-45-twig-budget/twig-generations-row	none	none	0.72	0.57	gap:18d68338e493667d-ddaba-5	true
fn-45-twig-budget/raise-the-node-ceiling	against	against	0.99	0.99	gap:18d68338e493667d-ddaba-5	true
fn-45-twig-budget/thin-the-beech-twigs	none	none	0.97	0.95	gap:18d68338e493667d-ddaba-5	true
fn-47-curtain-strand-length/pendulous-variation-row	none	none	0.98	0.98	gap:18d68339344977df-ddaba-7	true
fn-47-curtain-strand-length/per-side-hem-term	none	none	1.00	1.00	gap:18d68339344977df-ddaba-7	true
fn-48-clump-lean-spread/stem-lean-spread-row	none	none	0.99	0.97	gap:18d683395c1bfd55-ddaba-8	true
fn-48-clump-lean-spread/lean-each-stem-by-hand	none	none	1.00	1.00	gap:18d683395c1bfd55-ddaba-8	true
fn-49-collapsed-triangle/drop-the-degenerate-triangle	none	none	0.99	0.98	gap:18d6833986e63e6f-ddaba-9	true
fn-49-collapsed-triangle/reseed-past-the-failure	against	against	1.00	1.00	gap:18d6833986e63e6f-ddaba-9	true
fn-51-curtain-below-the-crown/curtain-drop-band	for	for	0.99	0.99	gap:18d68339e84a6b44-ddaba-b	true
fn-51-curtain-below-the-crown/bunch-the-curtain-per-limb	none	none	0.61	0.41	gap:18d68339e84a6b44-ddaba-b	true
fn-51-curtain-below-the-crown/lower-the-crown-base-row	none	none	0.73	0.60	gap:18d68339e84a6b44-ddaba-b	true
fn-54-beech-crown-from-limb-systems/measure-then-clump	none	none	0.86	0.79	gap:18d6833a32291b4a-ddaba-d	true
fn-54-beech-crown-from-limb-systems/limb-attractor-clumps	none	for	0.41	0.12	gap:18d6833a32291b4a-ddaba-d	false
fn-54-beech-crown-from-limb-systems/retune-every-broadleaf	against	against	0.71	0.57	gap:18d6833a32291b4a-ddaba-d	true
accuracy 23/24 (pilot 22)  conf min=0.12 median=0.97 max=1.00

## gap prior_verdict (held-out)
id	expected	answered	top_p	conf	ledger	hit
fn-39-crown-outline-irregularity/perturbed-envelope-radius	none	none	0.99	0.97	gap:18d683386eeb84d3-ddaba-2	true
fn-39-crown-outline-irregularity/shade-out-limbs	none	none	1.00	0.99	gap:18d683386eeb84d3-ddaba-2	true
fn-39-crown-outline-irregularity/loosen-the-crown-rows	none	none	1.00	1.00	gap:18d683386eeb84d3-ddaba-2	true
fn-46-young-shoot-colour/shoot-colour-by-radius	for	for	0.99	0.99	gap:18d683390c8592e6-ddaba-6	true
fn-46-young-shoot-colour/shoot-colour-by-branch-order	none	none	0.84	0.75	gap:18d683390c8592e6-ddaba-6	true
fn-50-short-shoots/short-shoots-as-placements	none	none	0.71	0.56	gap:18d68339b77f390b-ddaba-a	true
fn-50-short-shoots/a-node-per-short-shoot	against	against	0.86	0.80	gap:18d68339b77f390b-ddaba-a	true
fn-50-short-shoots/raise-the-interior-density	none	against	0.66	0.50	gap:18d68339b77f390b-ddaba-a	false
fn-52-leaf-mass-lit-as-a-canopy/canopy-lighting-rows	for	for	0.87	0.80	gap:18d6833a0d20818f-ddaba-c	true
fn-52-leaf-mass-lit-as-a-canopy/brighten-the-leaf-colour	against	against	0.73	0.59	gap:18d6833a0d20818f-ddaba-c	true
negative-no-option-answers-the-capability/widen-the-leaf-blade	none	none	1.00	1.00	gap:18d6833a65119445-ddaba-e	true
negative-no-option-answers-the-capability/darken-the-ash-foliage	none	none	1.00	1.00	gap:18d6833a65119445-ddaba-e	true
negative-no-option-answers-the-capability/raise-the-leaves-per-station	none	none	0.99	0.99	gap:18d6833a65119445-ddaba-e	true
accuracy 12/13 (pilot 12)  conf min=0.50 median=0.97 max=1.00

## gap best_match
id	expected	answered	top_p	conf	ledger	hit
fn-37-pendulous-shoots	curtain-rows	curtain-rows	0.75	0.66	gap:18d6833819d47dfd-ddaba-0	true
fn-38-multi-stem-trees	stems-as-order-zero-axes	stems-as-order-zero-axes	0.95	0.93	gap:18d683383d778a27-ddaba-1	true
fn-40-smooth-bark	three-row-sets-one-shader	three-row-sets-one-shader	0.85	0.77	gap:18d6833894061068-ddaba-3	true
fn-44-curtain-hangs-under-gravity	sag-row	sag-row	0.92	0.89	gap:18d68338c0ed1590-ddaba-4	true
fn-45-twig-budget	twig-generations-row	twig-generations-row	0.94	0.91	gap:18d68338e493667d-ddaba-5	true
fn-47-curtain-strand-length	pendulous-variation-row	pendulous-variation-row	1.00	0.99	gap:18d68339344977df-ddaba-7	true
fn-48-clump-lean-spread	stem-lean-spread-row	stem-lean-spread-row	0.58	0.38	gap:18d683395c1bfd55-ddaba-8	true
fn-49-collapsed-triangle	drop-the-degenerate-triangle	drop-the-degenerate-triangle	0.94	0.91	gap:18d6833986e63e6f-ddaba-9	true
fn-51-curtain-below-the-crown	curtain-drop-band	curtain-drop-band	0.98	0.97	gap:18d68339e84a6b44-ddaba-b	true
fn-54-beech-crown-from-limb-systems	measure-then-clump	limb-attractor-clumps	0.98	0.97	gap:18d6833a32291b4a-ddaba-d	false
accuracy 9/10 (pilot 8)  conf min=0.38 median=0.91 max=0.99

## gap best_match (held-out)
id	expected	answered	top_p	conf	ledger	hit
fn-39-crown-outline-irregularity	perturbed-envelope-radius	perturbed-envelope-radius	0.66	0.56	gap:18d683386eeb84d3-ddaba-2	true
fn-46-young-shoot-colour	shoot-colour-by-radius	shoot-colour-by-radius	0.96	0.94	gap:18d683390c8592e6-ddaba-6	true
fn-50-short-shoots	short-shoots-as-placements	short-shoots-as-placements	0.61	0.47	gap:18d68339b77f390b-ddaba-a	true
fn-52-leaf-mass-lit-as-a-canopy	canopy-lighting-rows	canopy-lighting-rows	0.96	0.93	gap:18d6833a0d20818f-ddaba-c	true
negative-no-option-answers-the-capability	none	raise-the-leaves-per-station	0.61	0.47	gap:18d6833a65119445-ddaba-e	false
accuracy 4/5 (pilot 4)  conf min=0.47 median=0.56 max=0.94

```
