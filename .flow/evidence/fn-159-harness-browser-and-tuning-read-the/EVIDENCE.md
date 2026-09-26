# fn-159 evidence

## R1: harness and browser metadata from the catalogue
- `catalogue::browser()` renders `src/browser/parameters.generated.ts`: the `Family` type, each row under its meaning (deprecated rows tagged), and `PARAMETERS` (kind, unit, bounds, tuning window and step, dormancy, reach). `the_browser_reads_the_catalogue` holds the file to the catalogue. `presets.generated.ts` imports this type and no longer infers it from preset values.
- The panel draws one control per row the direct build reads. That is 238 controls on the date palm, and 120 of them carry a dormancy note. Growth-path rows appear only under `?growth=1`, and deprecated rows never appear. The seed box owns `/skeleton/seed`, and the density adapter owns `/skeleton/attractors`. Torsion sits in the supernatural group.
- Tests: `harness/rows.test.ts` checks that the metadata names every wire row of every preset with its type, plus the span, notch and admit rules. `harness/family.test.ts` checks that every shown row reaches `toFamily` through a sentinel, including stems and lateral orders. It also checks both adapters and that every preset round-trips.

## R2: tuning reads generated metadata
This was verified against stage 1, and no change was needed. `crates/telperion-jev/data/dials.json` and `dials.excluded.json` are gone, and `tuning::table` builds the table from the catalogue. No active config embeds a dial copy:
- the beech replay fixture has `dials: []` (all of the table);
- the palm fixture names none;
- the three configs that embed dials are fn-68 pilot evidence, and `pilot-config.json` still replays (`an_embedded_dial_config_replays_on_its_own_copy`);
- `experiments/fn58-tuning-loop/arms*.json` is a closed probe with its own format.

Windows and steps are authored per row in the catalogue (`tuned(..., window, steps, basis)`), apart from the bounds. They are not a separate file (see Open).

## R3: behavioural dormancy
`crates/telperion-core/tests/dormancy.rs` covers 11 representative rows:
- stem divergence;
- lateral pitch;
- attractors;
- writhe;
- pendulous radius;
- max taper exponent;
- flare falloff;
- short-shoot leaves;
- leaf cup under card;
- palm skirt length;
- palm leaf-base radius.

For each row, the test moves the row while its condition holds and checks that none of these changes: skeleton, wood, element, leaves, structure, field wood, field leaves or field plan. It also moves the same row with the condition lifted and checks that the build changes. The 36-row `SWITCHES_AT_ZERO` list is gone.

Finding: `/surface/lobeDepth` claimed to be dormant at `lobes` zero. The test showed the wood still moves, because a junction's contained ring divides by `1 + lobeDepth` (`surface/samples.rs:116`), and the leaf box and plan scale by it. The claim was withdrawn in the catalogue, and the generator is unchanged.

## R4: harness, driven once with `npm run dev`
This used no forest capture and took 2 screenshots.
- **Date palm, seed 1:** 32,320 wood tris and 6,554 foliage instances. Moving stem divergence at one stem left the build unchanged (dormant, as its note says). Three stems (divergence 100, lean 15) gave 96,560 wood tris and 19,662 foliage instances, and setting it back restored the numbers. The habit group shows each meaning and dormancy note.
- **Oregon white oak, seed 1:** 7,227,360 wood tris and 715,065 foliage instances. Lateral orders at 1 gave 125,160 wood tris and 7,800 foliage instances, and orders back at 3 restored the numbers. Torsion 2 rebuilt the tree unchanged, because the oak's supernatural switch is off.
- No console or build error was shown.
