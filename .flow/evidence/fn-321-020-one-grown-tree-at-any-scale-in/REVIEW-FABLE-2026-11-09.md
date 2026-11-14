# Fable's review of the route, 9 Nov 2026

Asked by the owner on 9 Nov 2026: "release 0.2.0 didn't perform how i wanted and we were working to fix that ... i'd like you to review what can be done here again". Everything below is the host's recommendation. None of it is owner-approved scope, and no experiment named here has run. Read from the goal's log and the results of fn-323 to fn-333, plus `crates/telperion-space/src` where a claim says "read from code".

## Where the run stands

Thirteen of 20 children are used and one criterion, R1, is met. The numbers that matter:

| | Today | Bar |
|---|---|---|
| Oak, coarsest, Wasm, one core (fn-329) | 164 ms | 50 ms, and 0.1.4's 233-260 ms |
| Birch | 219 ms | 50 ms, and 239-304 ms |
| Spruce | 726 ms | 50 ms, and 203-212 ms |
| Oak, full cycle, age 300 (fn-330) | 210 s, 1.95 GiB | grows, with the form of a real oak |
| Oak's form past 80 (fn-332) | crown 8 to 16 m off the base at 200 | Silvics' broad rounded crown |
| The look beside 0.1.4 (R10) | owner: 0.1.4's oak "looks better" | owner accepts every species |
| Field download (R9) | 834,077 Brotli bytes | 250,000 |

## The profile, taken 9 Nov after this review was written

The owner set `perf_event_paranoid` to 1. `samply` still fails ("mmap failed"; `perf_event_mlock_kb` is 516 on 32 CPUs), so the samples come from a debugger-driven sampler at about 4 ms on a standalone probe of `grow_cycle`, one thread, at `1a84b2af0`, while the gate ran beside it. Oak at 40, lit, seeds 1 to 5 in full (1,769 samples) and 30 seeds with the test's coarse rows (2,144 samples). Shares of samples:

| | Full | Coarse |
|---|---|---|
| The year loop | 75% | 78% |
| - growing live shoots (`grow_live`) | 25.5% | 22.8% |
| - retirement (`retire`) | 17.5% | 15.3% |
| - sketch, drawing and placing wood | 15.5% | 17.2% |
| - light | 5.5% | 5.4% |
| The settle after the last year (`Paused::finish`) | 23.9% | 22.1% |
| - laying geometry again (`settle_lay`) | 14.6% | 13.3% |
| - shedding (`settle_shed`) | 8.6% | 8.2% |
| Cutting across the passes: allocator and copies | about 18 to 23% | about 14 to 19% |
| Cutting across the passes: transcendental math, self time | 14.8% | 16.3% |

**This does not support item 1's estimate of 3 to 8 times.** Removing every allocation and copy is worth at most 1.3 times, and halving the math about 1.08. A flat layout may add cache gains the sampler cannot see, but the evidence in hand says 1.3 to 1.5 times, so item 1 drops from first to a modest, safe gain. The grower does real work, about 1.1 µs per phytomer grown, and 65% of the phytomers the full oak grows by 40 are shed before the tree is read (990,412 grown, 348,504 kept on seed 1). The factor has to come from growing and settling less: items 2 and 4.

One new lead: geometry is drawn during the years (sketch, 15 to 17%) and laid again in the settle (13 to 15%). What the settle recomputes that the years already hold is unread.

Probe and sampler: session scratch, not kept. To rerun, build an example that calls `grow_cycle` and sample it; `sudo sysctl kernel.perf_event_mlock_kb=4096` would likely let `samply` do it (unchecked).

## The owner's bar, 9 Nov 2026

"I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast. That's the goal. Strategy states the goal also about many trees in game engines etc." This answers question 1 below: the aim is the ratio, not a fixed 50 ms.

## What I would change

### 1. Measure and rebuild the grower's inner loop before anything else (estimate superseded by the profile above)

No child has taken a sampling profile of the cycle. fn-327 used hand timers, tried one lever worth 0 to 3%, and its own result says it does not bound a rewrite of the computation inside each pass.

What the measurements say: 3,037 to 4,119 ns per living apex-cycle (fn-323), and about 0.9 s per simulated year for a live set near 2 million phytomers past age 80 (fn-330), which is about 450 ns per live phytomer per year. One oak at 40 makes 3.4 million allocation calls, 17 per axis in retirement (fn-327).

What the code says (read from code): each `Axis` owns a `Vec<Phytomer>`, a `Vec<f64>` of units and an optional boxed `Join` (`structure.rs:112`). The grower keeps `Vec<Vec<usize>>` successors and `Vec<Vec<Sleeper>>` sleepers (`grow.rs:262-267`), and the working set a `Vec<Vec<u32>>` by node (`cycle/working.rs:30`). A `Phytomer` is about 150 bytes of f64 fields. At the coarsest the oak has two phytomers per axis, so nearly every axis is a separate heap block the passes chase.

A flat profile on a layout like this is the usual sign of a cost paid everywhere, which no single lever removes. Phytomers and per-axis lists in flat arenas indexed by range, with the same arithmetic in the same order, keep every digest. My estimate of the gain is 3 to 8 times; it is inferred from the layout and nobody has measured it. The profile decides.

It is first because it multiplies everything else and needs no owner decision and no change to any tree. At 4 times the oak's coarsest is about 40 ms, the birch's 55 ms, the spruce's 180 ms (under 0.1.4's 212), and the full oak at 300 takes about 50 s.

- **Needs from the owner:** `sudo sysctl kernel.perf_event_paranoid=1` (it reads 2; `samply` is installed). fn-327's friction asked for it on 2 Nov.
- **Stop rule to freeze first:** convert one pass to the flat layout, digests equal, and go on only if that pass gains at least 1.5 times.
- **Cheap companions, same child:** a faster allocator in the Wasm build (fn-327 measured 8 to 19% native from a bump allocator and never measured Wasm's), and f32 for stored geometry, which changes bytes and so needs the evidence "Generator evolution" asks for.

### 2. Derive the dial's rows from the species; tune nothing

The owner ruled against per-preset coarse rows on 3 Nov. `species/coarse.rs` has seven free rows per species (first, factor, leaf, pipe, bud, hazard, shedding), and the test rows show why they needed a search: factor 2 with leaf 16. The leaf row is standing in for the leaves of laterals the coarse wood never grows.

That stand-in is computable. The expected leaf area, pipe and count of a lateral's descendants follow from the species' own zone tables, the same expectation `closed_form` already evaluates. So leaf and pipe can be derived from factor and first, with factor itself derived from the requested metres over the internode length. One input, no rows, and it works at any point in the tree space, Laurelin and the beech included, which have no rows today and run slower at the coarsest than in full (fn-329's defect). Hazard and shedding are survival terms and do not follow from conservation; whether one general correction holds them is unknown.

- **Test:** all eight presets, zero tuned values, R4's six medians on tuning seeds. Freeze the acceptable departure before the run.

### 3. Old oaks: three rules, and fn-332 names two

I agree with fn-332's two rules, wood that rights itself and several scaffold limbs sharing the crown. Its numbers show a third it does not name. With sag off the oak is 32.1 m at 200, with tropism tripled 35.0 m, and as shipped seed 1 reaches 33.0 m at 160, all past Silvics' 15 to 27 m. The shipped tree is inside the range at 200 only because its crown hangs. Fixing the bend without a limit on height trades one R7 failure for another. Real trees stop gaining height as the water path lengthens; no rule in the species tables does this (grep, no match for a path-length or hydraulic term). One general rule, vigour falling with path length from the root, is the candidate.

Order matters here: these land before any stand-in is trained, since a stand-in learns from these trees.

### 4. Old ages cheaply: step the scaffold in longer strides before training a network

Past age 80 the live set is flat near 2 million phytomers and each year costs the same 0.9 s (fn-330). Years 80 to 300 are 93% of the 210 s. The twig shell turns over yearly while the scaffold changes slowly, and the council found the silhouette needs the coupled years, the mid view the last few, and the close view no history.

So grow k full years, read each scaffold axis's own rates from them (extension, girth, limb loss), advance the scaffold n years on those rates, and repeat. With n at 0 it is the full cycle, so it is one grower and a dial by degree, with nothing trained and nothing per species. It is untested. Its known risk is the one that sank forms (a) and (c): events in the skipped years (a limb shed, a relay) taken as means. The difference is that the rates come from this tree's own simulated years, not a table.

- **Test, offline:** the oak from 80 to 120 in strides of 5 against the full cycle on held-out seeds, R4's six medians, plus stills at 120.

The learned stand-in goes after this, not before. Its references past 80 are trees the owner called bad, its weights count against the 250 KB download, and the council's evidence against a closure (shedding and light depend on the twigs it replaces) is still unanswered.

### 5. Two measurements nobody has taken

- **Why 0.1.4's oak looks better.** R10 fails on the look whatever the speed, and the goal still lists crown fullness as an unmeasured suspect. At grid 36, 0.1.4's field against the new one on the same grove plan: occupied cells, foliage cells, silhouette area by view, height. An hour's measurement that decides whether the gap is values, age, or the renderer's reading of the field.
- **The coarse build's download.** R11 says a coarse-only site downloads no device code, and the 834 KB is the render module with its WebGPU stack. The Brotli size of a CPU-only field build is unmeasured, so nobody knows how far R9 is.

### 6. Many trees are a different problem from one tree

Threads inside one tree gain 1.3 to 1.7 times (Decision Context). Across trees the gain is the core count. R3's 1,000 oaks at 151 ms each are 151 s on one thread and about 9.4 s on the 5950X's 16 cores (arithmetic, unmeasured). The same holds for the site's grove on Web Workers. Whether the benchmark or the site already grows trees in workers is unchecked.

## Questions for the owner

1. **The 50 ms.** It is the host's number (goal log, 1 Nov). Your words were "strictly better" than 0.1.4 and "high-performable with the many trees". Keep 50 ms per tree as a bar, or hold each species to 0.1.4's time and judge the grove by its completion?
2. **Order.** I recommend 1, then 5, then 2 and 3 side by side, then 4. That is six children of the seven left.

## Not checked

The site's and the benchmark's use of workers; the dial's hazard and shedding terms against conservation; whether `closed_form`'s expectation covers every row item 2 needs; the Brotli size of a CPU-only build; any gain claimed in item 1.
