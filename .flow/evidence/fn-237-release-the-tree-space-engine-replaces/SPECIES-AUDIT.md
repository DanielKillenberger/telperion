# Species audit: fn-213, fn-218, fn-219, fn-236

This read-only audit, dated 2026-10-13, covers the four species specs still open in flowctl. It ran on base `af2d4ba8a`, the F4 integration branch fn-198, in the `fn-237-housekeeping` worktree. Each spec has one task, `in_progress` in flow-state, although the committed task JSON still says `todo`. Every species commit cited below is an ancestor of the base. Since the species' last judged rounds, fn-248, fn-253, fn-255, fn-256 and fn-264 have changed engine rows that these trees read. A gate met on an earlier round is therefore no evidence for the tree at this commit unless a later spec re-viewed it. "Owner" marks an item only the owner can tick. Nothing here closes or changes a spec; the host decides.

## fn-213: silver birch

Task `.1` covers R1 and R2. The birch was merged into F4 as `51c2edfcb` at round 14 (`713aa8807`).

| Item | Met on F4? | Evidence | Left |
|---|---|---|---|
| R1 MODEL-BIRCH.md, cited, Jev-screened | Yes | `fn-213-*/MODEL-BIRCH.md`; Jev screen at l.25 | none |
| R2 `/space` rows, native and harness build, refusal lifted | Yes | `silver-birch.values` (288 `/space` rows). `tests/space_presets.rs` lists `(SilverBirch, birch())`. `a39fff211` moved the field's refusal to Telperion. MERGE-F4.md reports `npm test` 143/143. | none |
| R3 gate: host + Astra, seeds 1/7/42 at 40 and 80 | Unknown, leaning no | Astra last judged round 10 (SHEET-10.md). Round 12 has "Not run: Astra". SHEET-14 leaves the crown colour to the host. fn-264 changed the birch's rows afterwards (`00956e4c7`, `c116854f3`). The owner's "yea looks good" was on fn-264's sheets, not on this gate. | A current sheet with host and Astra; owner |
| R4 walk from beech to birch, no pop; other sheets unchanged | Partly | `space_hash` 16/16 equal at the merge (MERGE-F4.md). No beech-to-birch strip exists. | The walk strips |
| R5 the site's field path for the birch, measured | Yes, on round 14 | fn-214 COARSE.md "Task 2" (`8777bd72`) | Re-measure only if the host wants post-fn-264 numbers |

**Left to close:** R3 on a current sheet with the owner's tick, R4's walk strips, then the task.

## fn-218: Laurelin

Task `.1` covers R1 to R4. Its codex review was SHIP (`9ebbb3eb9`); the values are `14bd745f3` and the sheet `ea19bd579`.

| Item | Met on F4? | Evidence | Left |
|---|---|---|---|
| R1 values in the `laurelin` preset; native and harness build | Yes | `laurelin.values`: 274 `/space` rows, 43 `/supernatural` rows, `/age = 80`. `supernatural/tests.rs` `the_two_trees_with_the_group_off_are_the_beech`. | none |
| R2 gate: off and on sheets at whole, mid and close; host + Astra | On round 8's values only | Astra round 8 AGREE (`astra/round8.md`). Her rows changed afterwards: fn-248 (`6c4d274bb`, `1bd9ad9de`, `e556e7251`), fn-253 (`09835ae0f`) and fn-264 (`00956e4c7`). fn-248's `astra/florets-r2.md` was a DISAGREE. fn-253's SEATS.md:241 was an AGREE on the mid and raceme views only. The owner said "flower shape looks good" (fn-248 TWIGS.md:584), and "looks good" on fn-264's sheets. | Host or owner: do those later verdicts stand for R2, or is the tree re-sheeted? |
| R3 gold racemes and rim; scale and contact error | Unknown at current values | 0.42 mm at scale 2.5 (`laurelin_contact_at_her_scale`). The `scale` row has since become per-age group rows (fn-248), and the test has not been re-run. | Re-run the contact test |
| R4 gate and `npm test` green; cost on the native route | Partly | Green at `238e3ca9f` and `ea19bd579`; CPU 8.3 to 12.8 s. The device cost was unreadable (a fixed cost of about 140 s, SHEET.md "Open for the host" §2). | Time the device route on the current preset |

**Left to close:** the host's ruling on R2, the contact test re-run, the device cost timed, then the task.

## fn-219: Telperion

Task `.1` covers R1 to R4. Its review was SHIP (`6889ed202`), the sheet is `0eeb4148d`, and it was merged as `8fbd0fa78`. fn-219 depends on fn-218.

| Item | Met on F4? | Evidence | Left |
|---|---|---|---|
| R1 values in the `telperion` preset; native and harness build | Yes | `telperion.values`: 274 `/space` rows, 43 `/supernatural` rows. `space_presets.rs` lists `(Telperion, beech())`. `7a62695c7`. | none |
| R2 gate as fn-218's | On round 14's values only | Astra AGREE at age 92 (`astra/r14.md`). His age is now 82 (`be719db02`, fn-248), and fn-253 (`2dc75fb1f`) and fn-264 changed his rows again. The owner said "Telperion at 82: sure fine" (fn-248 TWIGS.md:586), and fn-253 records the look as owner-approved. | The same ruling as fn-218 (owner) |
| R3 leaves and blossom; scale and contact | Unknown at current values | 0.47 mm at age 92 (`telperion_contact_at_his_scale`); not re-measured at 82 | Re-run the contact test at 82 |
| R4 gate and `npm test` green; cost | Yes at 92 | Gate 207/0 and 1,067/0; `npm test` 143; device route 4.0 to 4.5 s (SHEET.md) | Re-time at 82 if the host wants it |

**Left to close:** the R2 ruling (after fn-218's), the contact test at 82, then the task. The age open item has been superseded: he stands at 82 under the Wasm bound, and fn-209 is still open.

## fn-236: Oregon white oak crown

Task `.1` covers R1 to R6. Round 3 is `2c6854f39`; its review was SHIP (`1c091dfbd`, "ready for the owner's verdict"). It was merged as `965b94dd4`. fn-214 task 3 depends on this spec.

| Item | Met on F4? | Evidence | Left |
|---|---|---|---|
| R1 MODEL-OAK.md written before any value moved | Yes | `ed46d06f7`; `fn-236-*/MODEL-OAK.md` | none |
| R2 a broad dense dome within the sources' ranges at 40 and 80, seeds 1/7/42 | Partly, on round 3 | At 80 the crowns are closed and broad, width/height 1.09 to 1.12. Seed 7's crown is 23.0 m against a 10 to 20 m target, and the DBH at 80 is 1.5 to 2.7 times the derived range (SHEET-3.md). Not re-viewed since fn-255, fn-256 and fn-264 R1a (`d46044868`). | Host: accept the overruns or run another round; re-view on the current engine |
| R3 at the site's scale (n36/48, age 40, grove seeds) it reads as a round oak | No | SHEET-3.md: seeds 7432, 1234 and 5678 are irregular | More rounds, or the owner accepts |
| R4 cost each round; the site's coarse timing | Yes, on round 3 | SHEET-3.md R4; fn-214 COARSE.md Task 2 | Re-measure if the values change |
| R5 the owner's verdict on the final stills | No (owner) | No owner verdict on the oak after 2026-10-09 is recorded | Owner |
| R6 gate and `npm test` green; other species unchanged | Yes, on round 3 | SHEET-3.md "Gate": render 205 passed, `npm test` 143/143, the beech, spruce and palm pins held | none |

**Left to close:** the owner's verdict (R5), with R3's irregular grove seeds and R2's overruns put beside it, preferably on a re-view on the current engine.

## Cross-cutting

None of the four tasks has a done summary. Each spec has a FRICTION.md the host has not yet read:

- fn-213: a gate that outran the 10-minute foreground limit, and a still runner that cannot read a value file.
- fn-219: age ceilings found only by renders that failed, and the Wasm bound found only by `npm test`.
- fn-236: a leaf probe that needs a rebuild.

AGENTS.md asks the host to raise each of these before the spec closes.
