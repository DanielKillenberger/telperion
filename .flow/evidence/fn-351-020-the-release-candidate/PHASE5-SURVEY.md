# Phase 5 survey: counted foliage and the faint population, 12 Nov 2026

By a Fable agent, read-only, at the top of the work (`fn-355-a-year-is-a-state-the-grower-reads-no`); condensed by the host. [code] read in source, [evidence] a recorded file, [ruling] the owner's, [inferred], [U] not determined. Paths are under `crates/telperion-space/src/` unless given in full.

## The finding that matters most

The faint population (fn-356) is neither leafy by kind nor confined to a cell. No count class defined by physiological age or by cell removes it for free, and no evidence says which kinds of bearer it sits on. That split (M1 below) decides between the candidates.

## What the earlier proofs established

- The count proofs are fn-339 and fn-340 (fn-341 is about girth by rings and posture).
- **Counted:** whole lateral subtrees, keyed by the root lateral's PA and birth year, held on the bearer's growth unit; a count is its whole part plus one member at its fraction's presence [evidence: `fn-339/R1-ANSWERS.md`].
- **Which PAs:** those whose expected reach is under the 0.5 m cell: the oak's twig (0.313 m) and short (0.059 m), the spruce's shoot (0.068 m), the beech's short; the beech's shoot (0.450 m, potential 0.62 m) is over it [evidence: `fn-339/R1-LAW.md`].
- **The law:** the species' own tables in expectation at the cell's light; the class deposits leaf, enters its bearer's balance, holds the working set open and joins the girth [evidence: `fn-339/RESULT.md`].

| Measure | Number | File |
|---|---|---|
| Share of the cycle's units inside the cohorts at 40 | oak 0.925, spruce 0.820, beech 0.937 | `fn-339/R3-RESULT.md` |
| Law over cycle on the same wood (replay) | new shoots 1.006 to 1.012; living apices 0.73 to 0.79; oak leaf 1.143, oak section 0.912 | `fn-339/R3-RESULT.md` |
| Coupled probe, oak at 40 / 80 / 120, trunk | 1.135 / 1.361 / 1.258 | `fn-340/RESULT.md` |
| Same, current-year leaf | 1.073 / 1.054 / 0.812 (0.453 to 1.121 across seeds) | `fn-340/RESULT.md` |
| Spruce and beech at 40 | within 2%, the beech's trunk 1.055 | `fn-340/RESULT.md` |
| Time, cycle over probe | oak 3.88 / 2.49 / 2.63; spruce 2.99; beech 2.94 | `fn-340/RESULT.md` |
| Peak memory, oak at 120, seed 1001 | 2.18 to 0.61 GiB | `fn-340/RESULT.md` |

**Unproven:** the trunk's bias (10 to 36%) and the leaner oak at 120 were not isolated; cohort leaf was never drawn, so no twig was ever built from a key; sleeping buds of a cohort age, relays on cohort ages, promotion of a member that outgrows its cell and the boundary by degree stayed open; both proofs ran before fn-349, fn-354 and fn-355. The owner accepted "3x cheaper is pretty good already" [ruling].

## What a count must replace, reader by reader

| Reader | Reads | Linear in presence? |
|---|---|---|
| Light deposit | tip, scale, size, leaf (`grow/deposit.rs:17-40`, `girth.rs:48-51`) | Yes. Each leaf is rounded per corner to 2^-20 m² (`light/field.rs:166-182`): the faintest leaves already round to nothing [inferred]. |
| Light read | the living tip's position (`grow/sketch.rs:100-115`) | Per element. |
| Balance | presence x light; charge by path (`grow/cycle/passes.rs:50-61, 150-153`) | Income and charge are; the test with memory and a keyed draw per living subtree is not. |
| Allotment | presence, lit presence, the size response (`allocation.rs:139-183`) | The sums are; the size is non-local. |
| Wood pass | leaf above; first section; the ring's `max(0, need - kept)` (`girth.rs:79-121`) | Leaf handed to the bearer is; a class's own sections are a second moment; the clamp is not. |
| Waking, relays | `asleep` by parent, node presences (`grow/wake.rs:18-57`) | Per element; absent on the three species' leafy kinds. |
| Settle, drawing | products of presences, radius a square root, one drawn node a phytomer, nothing under 0.2 mm drawn (`telperion-core/src/pipeline/space.rs:1-13, 44`) | Per element. |
| Closed form, windows | PA and cycle only (`presence/windows.rs:41-90`) | Yes; an element counts at its share. |
| Retirement | a dropped subtree's root keeps one record in its bearer's fade (`grow/cycle/retire.rs:1-21, 33-43`) | A summary already stands in for a subtree here. |

Records: an axis is 232 bytes, its draws 144, a phytomer about 136; at 200 the tree holds 4.7 phytomers an axis, so per-axis records weigh about as much as the phytomers; peak memory is 1.0 to 1.2 KiB a held phytomer. How the peak splits between the grower's steady state, the settle and the pipeline is [U].

## The boundary of a countable class

- **By kind:** a kind closes when it and all it bears have a bounded life, no sleeping bud and no relay (`shed::lasting`, `shed.rs:21-50`). Oak: twig, shoot, short. Beech: shoot, short (a long lateral's kind reads the cell's light since fn-354). Spruce: shoot (its needles outlive the apex).
- **By presence:** the faint population is 47% of phytomers at 120, rising with branch order; it includes faint branches and boughs, which are not closed by table [inferred]. Its split by kind is [U].
- **By cell:** a faint limb's subtree spans many cells [inferred].
- So the class closed by table is kind x cell x bearer unit, and the faint class is a different one: whole faint subtrees of any kind.

## Constraints, each held by a test or a ruling

Growth is the product and a year is a state (fn-355's tests); paused equals one call; by degree, branches entering under 1e-6; independence of draws; the closed form's oracles; a child never above its bearer (the extent bound, the shed-certain shortcut); no artificial limits (a count carries, it does not cap); one general law and no coarse rows (`species/coarse.rs` and `/space/coarse` are deleted in this phase); one pipeline; a twig is the same twig whenever seen (a drawn node's identity is its phytomer's key); the same tree on any cores; native and Wasm equal (the test that holds it was not found [U]).

## Candidates

| | A. Cohorts by table | B. Exact draws, compact state | C. Faint subtrees as scaled templates |
|---|---|---|---|
| What is carried | laterals of closed kinds as counts per bearer unit, by kind and birth year | the same laterals with their keyed draws, without axis, draws or phytomers: about 32 bytes a member, place regenerated from the key | a lateral subtree of any kind under a faintness: (key, kind, birth, f) on the bearer node; its contents are the closed form's expected subtree scaled by f |
| Yearly law | the carried expectation (fn-340's law) | today's draws | f times the template's leaf, lit presence, charge and leaf above, all linear |
| Faint population | only where its kind is closed | the same blind spot | the only one that carries it at constant state a faint root |
| A view's twigs | regenerated from key and birth year; the rule that keeps one twig stable while a fraction moves is unbuilt | the same keyed twig | draws nothing (most of it is under the drawing's floor already) |
| Exact | new shoots within 1.2% | counts and draws | expected counts |
| Approximate | oak trunk +10 to 36%, leaf -19% at 120, spread -14% | reads that depend on geometry, if the rough layout is not re-run | light and size responses inside a template; the realised subtree against its expectation |
| Cost, oak at 120 | time 2.6 times down, memory 3.6 times down (seed 1001) | memory for leafy kinds 10 to 20 times down, time less; unmeasured | held phytomers -47% at 120 if units under 0.05 are templated; unmeasured |
| Build | large: a law, hooks in six passes, a drawing path | medium | medium to large; composes with A or B |
| First slice | the spruce's shoot and the beech's short | the oak's short | a replay probe as fn-339's |
| Refuted by | the bias surviving at 120 on today's grower; closed kinds holding under half the faint phytomers; a walk jumping at a whole member | the regenerated light read moving leaf or trunk past a twentieth; a gain under 1.5 times; any oracle moving | a template missing realised sums by over a tenth a bearer; the hand-over failing a walk |

## Measurements that decide, all on the existing census probe, oak at 80 and 120, seed 1002

- **M1.** Held phytomers and axes by kind x presence bin (under 0.05, 0.05 to 0.2, over) x open or closed.
- **M2.** The grower's footprint by store at 120 beside the process's peak: is the memory the grower's steady state, or the settle's and the pipeline's transient?
- **M3.** For faint lateral roots, realised subtree leaf and phytomers over f times the closed form's expectation, by kind.
- **M4.** The share of leaf deposits that round to nothing in the light.
- **M5.** fn-340's patch against today's head on the oak at 40 and 120, three seeds, if it still applies.
