# Growth continues from any age, in less memory, with leaves that unfold

## Goal

The owner (2026-10-13) found Telperion's 4,209 MB device peak too much and asked for a tree grown to an age that can be passed back in to grow on, so a consumer renders the previous tree while the next is generated, and for leaves that grow instead of appearing full size. This spec made growth continuable under an explicit horizon, cut its memory one lever at a time, made leaves, shoots and the seasons unfold within the year, and made growing-in how the harness shows a tree. Release item (fn-237). Growth is the product (AGENTS.md).

## Decisions

- Owner (2026-10-13): Telperion's peak "seems like way to much peak memory"; "If we could just ask for a tree at an age and pass that tree back into the generator as input to grow from => the consumer can render the previous one while the next one is generated"; "alright add this to release, but make sure to test/fail early if something doesn't hold up. Maybe council first?"; "Can we have growing leaves? because right now having instantly large leaves makes it much less convincing."
- Council round 8 (Astra): the growth law reads the requested final age (`presence::Windows`), so a tree grown to 40 inside an 82-year request differs from a 40-year request. Host (2026-10-13): the final age becomes an explicit **horizon** given at `begin`; growth past it is refused by name; open-ended growth is a later spec. Owner on the horizon: "is there a way to not have to fix maximum age? but i guess yea that works."
- Host (after task 1): the premise stop reads the live-set **peak** (it sizes the arena); per-cycle rises (up to +91 %) feed R4 lever 4. Old organs between years blend from year c−1 to year c keyed by identity (R5a); owner: "can we at some point interpolate between years if that were necessary?"
- Host (after task 2): the hierarchy's shape, counts and lists are state, not scratch; owned parameters wait for a consumer (no `unsafe`).
- Host (task 3): lever 2 merges into lever 4 (tables start from the closed-form hint and grow by observed counts, no per-parameter ratio fit); lever 3 freezes a retired subtree's radius and two leaf-mean sums at its close; compactor sharing (at most 3.7 %) dropped under the 10 % rule; the Two Trees' 2,000 MB target released (measured floor about 2,745 MB) provided the standalone peak falls at least 30 % and the grown-on peak is under 4 GiB; no CPU lever for Wasm this release.
- Owner (2026-10-13): palm fronds "option 1 seems good" (R5b: fronds aged by their place in the spiral). Host (2026-10-14, R5b'): this year's fronds are the crown's turnover over a sourced frond life (3-7 years; 5 used), last year's stay whole.
- Owner (2026-10-13), on the oak unfolding in last year's leaf: "they don't look like they're growing from buds though?", then "i mean that sounds awesome.. possible without too much cost?" -> R5c, the seasonal year (autumn, leaf fall, winter, budburst). Host rulings (2026-10-14): leaf lifespan is a new sourced per-PA row, apart from `leafYears`; a whole age sits at full leaf; missing sources take a declared stand-in (congener or centroid), autumn colour a named-colour table; palm 18 fronds/year and 139-day expansion marked derived; the spruce's leaf lifespan equals its `leafYears` this release.
- Owner (2026-10-14): "leaves didn't fall they shrank..?" -> host: a falling leaf stays whole and colours until its own abscission day. Owner: "why is there shadows dancing when the leaves are full?" -> host: old organs move only along the elongation course, still through summer, autumn and winter.
- Host (2026-10-14, gate): a fall past the next budburst is compressed to end at it, never refused; a build whose least tables cannot fit is refused before any submit.
- Host (2026-10-14, task 6, the owner's spruce at 80 crashing the tab): fn-235's budget rule applies to a grow-in's draws (one admission path, `budget::room`): fits beside -> keep the drawn tree; fits only alone -> let it go first ("making room"); else refused by name. A `grow_to` step is still refused by name beside the drawn tree.
- Owner (2026-10-14), on the first recordings: "the video isn't very appealing with leaves flickering in and out" -> host: recordings draw whole years at full leaf, crossfaded; seasons stay in the harness's "play the year".

## Acceptance Criteria

- **R1:** premises on the CPU, stop on failure: final-age reads listed, horizon prefix, continuable retirement byte-equal, 0->H == 0->a->H — met (kept tree 24/24 byte-equal; live-set peak +11.6 to +18.1 %).
- **R2:** `begin/grow_to/settle` API with committed and presentation ages (R2a, R2a'), failure contract (R2b) and admission while drawing (R2c) — met on CPU and device.
- **R3:** retirement without the final age — met (fading set on CPU, `fading_main` on device, verdicts equal).
- **R4:** memory one lever at a time, census after each — met with measured misses (below).
- **R5:** leaves and shoots unfold from sourced curves (with R5a blend between years, R5b/R5b' palm fronds, R5c seasonal year); whole years draw today's tree — met.
- **R6:** the harness grows the tree year by year as the product; frame time while growing measured — met.
- **R7:** device and Wasm/WebGPU entries take the state; costs reported; gate green; recording viewed; owner's verdict — met.

## Result

Memory (task 3, decimal MB, base → after): Telperion standalone 4,209 → 2,484 (−41 %), continuation 5,876 → 3,072; Laurelin 3,853 → 2,281; spruce 2,445 → 1,522; oak 1,272 → 782; birch 800 → 520; beech 304 → 282; ordinary 274 → 169; palm 32. Missed targets (floors measured): beech 250, birch 450, oak 650, spruce 1,200, Two Trees 2,000 (released). Girth shift from lever 3 at most 5.4e-5, no visible change. The oak at 80 builds under 4 GiB by margin halving only where refused. Wasm Telperion 3,114 → 2,813.

Unfolding (task 4): logistic courses per PA from sourced elongation/expansion days (UNFOLD.md); CPU and device agree within 1.4e-5 m; whole years unchanged; Telperion's in-year draw peaks 4,160 MB beside the drawn tree.

Growing in (task 5, GROWING.md): grow per year 12 ms (palm) to 116 ms (Laurelin); a draw costs about twice a year's growth (the settle is the next lever); every preset up to the spruce holds 60 Hz p95 while growing, the Two Trees miss it at p95 (19 ms). Recordings of the oak and Telperion viewed. Task 6: the spruce at 80 under 6 GiB makes room for draws 75-78 and is refused by name at year 79; every other preset grows in at its age and at 80 without making room. Final gate 1,373 passed; `npm test` 169/169; codex SHIP.

Owner verdict (2026-10-14), after the Tint fix: "yea it grows now. nice".

## Open

- The Two Trees' p95 frame while growing and single 100-250 ms stalls (the settle lever; an incremental settle is a later spec).
- Open-ended growth without a horizon (later spec, a model change for the owner).
- The spruce's bare inner wood under a 6-7-year needle life (later spec for the owner).
- Wasm CPU peak (no lever this release); fn-257's field query has no device consumer yet.

## Friction

- Council round 8 not committed when the spec cited it -> commit council rounds before marking a spec ready.
- Premise walks and `tests/walks.rs` slow (Telperion walk 419 s; walks 427 s of the crate's run) -> walk premise moved out of the gate; fast filter / incremental settle proposed.
- Paused device build's census lived only in the recorded cycle -> census held by the build (R4 lever 1).
- Moved-build digest hashed the arena slot -> digest skips index columns (`growth::tests::digest`).
- Re-pinning catalogue digests by hand -> helper digesting with named fields stripped proposed.
- `telperion-heavy` admission queues and unmeasured declarations (gate, `npm test`, probes waited 15 min to over an hour; first-come lock blocks small jobs) -> measured peak per run kind; admission letting small jobs pass (fn-274).
- `telperion_core` unit tests peak about 5.5-5.8 GB each and OOM side by side -> fn-274's cut / one lock over whole-preset tests.
- Two design calls surfaced only by the end gate -> gate R5c/R4 changes before building on them.
- No native frame-loop harness for timing frames while growing -> polling native harness proposed.
- Fresh worktree compile wait -> worktree kit reflinking `target/` proposed.
- Card shared with other processes blocked the 6 GiB native repro; `pgrep -f` killed own shell -> ledger-only dry run proposed; never pkill by pattern.

## Evidence

- `.flow/evidence/fn-270-growth-continues-from-any-age-in-less/UNFOLD.md` (unfold rule, curves and sources, seasonal rows)
- `.flow/evidence/fn-270-growth-continues-from-any-age-in-less/GROWING.md` (device growth entry, costs, frame time, harness, owner verdict)
