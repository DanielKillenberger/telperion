# Growth rule: sapling form, thickening by age, a shedding floor

## Conversation Evidence

> user (on the fn-30 age strips, 2026-09-14): "a bare pole doesn't sound very appealing?"
> user (2026-09-14): "the 10yo one is way too low quality and i'm not convinced that the 26yo looks like that either? This calibration task seems to have missed the mark by not being able to touch the generator"
> user (2026-09-14, asked whether to record the rejecting verdict and capture a growth-rule spec that may touch fn-11's machine): "yes"
> user (2026-09-14, asked how the spec relates to fn-21): "New spec, fn-21 untouched"
> user (2026-09-14, on the visual digest of this spec): "we also want 1yo saplings to look like actual saplings yea?"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 25% [user], 45% [paraphrase], 30% [inferred] -->

Calibrated growth (fn-30) fitted the oak's and the spruce's growth traits to open-grown height references and rendered each species at six ages. The owner judged the strips and rejected the young ages: the ten-year oak is one stem with a handful of leaves, the twenty-six-year oak is a bare scaffold with sparse foliage, and the five-year spruce is a stem with no needles. In the owner's words, the calibration task "missed the mark by not being able to touch the generator". [user]

fn-30 could set only trait values, and its boundary forbade any change to fn-11's growth rule. Fitting rate and shape moved how fast each tree reaches its envelope; it could not change what the tree looks like on the way. The calibration therefore met its height numbers and exposed three defects in the rule itself. [paraphrase]

- **Early form.** Laterals and foliage lag the stem by decades, so a young tree reads as a pole and a middle-aged one as a scaffold. [paraphrase]
- **Thickening.** The trunk stays a fixed fraction of the live envelope height, so the height-to-diameter ratio never changes with age; fn-30 measured a ratio near 28 for the oak at every age against references that fall from 125 young to 44 mature, and near 35 for the spruce against 47 to 35. [paraphrase]
- **Shedding.** The vigour proxy decays with node age with no floor, so any nonzero shedding threshold strips every preset to about thirty nodes by maturity through the growth path. fn-30 hid this by zeroing the threshold on the ordinary tree and the Two Trees, which the owner did not accept as a fix. [paraphrase]

This spec owns those three defects. It may change fn-11's growth rule, and it is judged on the same strips at the same ages, with fn-30's checksummed references as its curves. fn-30 stays open until this spec's strips earn the owner's accepting verdict. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The rule stays one rule.** Every change lands inside fn-11's yearly slice, as numeric traits every family carries and the blend walks; no species branch and no preset-specific path enters the growth code. The chronicle, the stamps, the read-at-age contract and byte-identical replay are unchanged in kind: a tree at any age at or below the frontier is still a filter over the record. [strategy:Growth and botanical fidelity]
- **Sapling form from year one.** A one-year specimen is a seedling at seedling scale, a stem of tens of centimetres carrying its first leaves for the oak and a few centimetres with a whorl of needles for the spruce, and from there a young specimen carries laterals and foliage in proportion to its size, so the tree is a branched, leafed sapling at every age the strips show, and the crown fills as the envelope expands rather than arriving decades after the stem. The mechanism is the implementer's to choose within the rule; the strips judge it. [inferred]
- **Thickening by age.** Trunk and limb radius follow the tree's age and history as well as the live envelope, so the height-to-diameter ratio falls with age toward open-grown values. Radius keyframes stay monotone and the pipe-model split at forks stays as fn-11 pinned it. [inferred]
- **A floor under vigour.** The vigour proxy keeps a floor for a shoot that still holds light, so a mature exposed branch never sheds by age alone, while a shaded interior shoot still falls below a nonzero threshold and dies as a stamp. The presets' authored nonzero thresholds return; a zero threshold remains a valid row that disables shedding by choice, not as a workaround. [inferred]
- **Derived ages move.** The derived mature age of every preset follows from the changed rule and is documented, never authored; fn-30's routing of production through growth to that age is kept as is. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **fn-30's instrument is the judge.** The age strips open at year one and then run fn-30's ages (oak 1, 10, 26.7, 56.1, 112, 200 and the derived mature age; spruce 1, 5, 14.1, 26.6, 36.9, 60 and the derived mature age), the year-one frame framed to its own size so a seedling is not lost in a mature tree's camera with fn-30's command, seed and protocol, within the owner's capture budget: small before large, at most four images viewed per capture, and one full capture per commit. [paraphrase]
- **The references are fn-30's.** The composed curves, their sources and checksums are reused, not re-sourced. The owner's doubt about the young-age diameter composition stands and is recorded; Stein's mature open-grown figures are the independent check the owner may prefer. [paraphrase]
- **One re-pin.** Identity, audit and look pins move once, after convergence against fn-30's numbers is stated, with the reason recorded; a pin moving by an amount the numbers do not explain is a defect. [paraphrase]
- **Cost is bounded by fn-30's measurement.** A mature build through growth costs no more than fn-30 measured (oak 2,463 ms, spruce 867 ms at seed 7), and the report records the result against the half-second dial-move target that both species already miss. [inferred]
- **Every family still grows.** The Two Trees and the ordinary tree build and grow at every age under the same rule with their nonzero thresholds restored, and their crowns hold at maturity. [paraphrase]
- **Renderer fixtures grow too.** fn-30 left four renderer tests red because two fixture parameter sets grow no wood or no placements through the growth path. That defect is fn-30's to close before its PR; this spec inherits a green tree and keeps it green. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The oak at 1, 10 and 26.7 years and the spruce at 1, 5 and 14.1 years, rendered with fn-30's strip protocol, read as actual saplings: at year one a seedling at seedling scale with its first leaves or needles, and at the later young ages branched saplings carrying foliage, and every later age reads as a continuation of the same tree; the owner judges the two strips beside fn-30's and records an accepting verdict in the owner's own words. [paraphrase] Errors: a rejecting verdict stops the spec with the owner's words and a one-paragraph blocker.
- **R2:** The measured height-to-diameter ratio of the oak and the spruce falls with age, and trunk diameter at fn-30's three fit ages lands within 15 percent of the reference the owner accepts for that age, with any accepted deviation recorded beside the number. [inferred] Errors: a miss outside the tolerance stops the spec with the number unless the owner's recorded judgment accepts it.
- **R3:** With the presets' authored nonzero shedding thresholds restored, the ordinary tree and the Two Trees hold their mature crowns (node counts at the derived mature age within the convergence the report states, never a collapse toward tens of nodes), and shading still sheds interior shoots as death stamps; a test asserts both on the fixture presets. [inferred] Errors: a preset that collapses, or a shaded shoot that never sheds, fails the test naming the preset.
- **R4:** fn-11's contracts hold: same seed and parameters give a byte-identical chronicle on native and wasm, every reader is a filter over stamps, growth traits stay numeric rows the blend walks, and no species or preset branch enters the growth code; the existing determinism and parity tests pass unchanged. [strategy:Growth and botanical fidelity] Errors: a parity or determinism failure names the preset and the age.
- **R5:** Production routing through growth is kept, the derived mature ages are re-documented, the pins are re-pinned once with the convergence numbers stated before the move, and the mature build cost stays at or under fn-30's measurement for both species. [paraphrase] Errors: a cost over fn-30's number stops the spec with the number.
- **R6:** The report, in fn-14's shape, carries the re-rendered strips beside fn-30's, the ratio and diameter tables, the shedding evidence, the cost, and the owner's verdict slots; fn-30's rejecting R3 slots are re-judged from this report. [inferred] Errors: no error surface beyond the protocol.

## Boundaries
<!-- scope: business -->

- No within-species variation, bud-fate modelling, apical-dominance traits or light-response scenarios; those stay with fn-21, which is untouched. [user]
- No environment, competition, pruning, breakage, death or seasons; those stay with fn-16 and the lifecycle specs. [paraphrase]
- No fork anatomy, root collar or junction geometry; that is fn-20's. [paraphrase]
- No smoothing between years and no page work; that is fn-28's. [paraphrase]
- No new reference sourcing; fn-30's curves, sources and checksums are reused. [paraphrase]
- No change to the chronicle's shape or the read-at-age contract; the rule changes inside the slice. [inferred]

## Decision Context
<!-- scope: both -->

### Motivation

- The owner rejected fn-30's young ages on sight and named the cause: the calibration could not touch the generator. The realism the owner values first lives in the rule, not in the trait values. [user]
- fn-30's diagnosis, routing, references and strips are kept as the instrument; this spec is the fix they call for. [paraphrase]

### Implementation Tradeoffs

- Fixing the rule over hiding the defects in the value tables: fn-30's zero-threshold workaround disabled shedding for three presets and left the ageing proxy in place; the owner did not accept it, so the threshold returns and the proxy gains a floor. [paraphrase]
- A new spec over extending fn-21: the owner chose a new spec so fn-21 keeps its scope as the variation and light-response proof and this spec stays a defect fix judged on existing strips. [user]
- Building on fn-30's branch: fn-30's routing, report and strips land first with its spec left open on the rejecting verdict, and this spec continues from that state; fn-30 closes when this spec's strips are accepted. [inferred]

## Strategy Alignment

- Follows "Growth and botanical fidelity": one continuous tree structure whose lifecycle behaviour is judged against real trees, with reference-based visual QA exposing structural mistakes that counts alone cannot catch. [strategy:Growth and botanical fidelity]

## Parked unknowns

- Whether the composed young-age diameter reference (stand-grown yield table plus an open-grown integration from a 7 cm anchor) is trustworthy, or whether Stein's mature open-grown figures should anchor the diameter judgment; the owner decides at R2 with the numbers in front of them.
