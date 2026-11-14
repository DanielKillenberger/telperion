# fn-364 R9: which turns of the lay read the length grown, and which the length laid (builder, Opus, 14 Nov 2026)

Line numbers are `crates/telperion-space/src/geometry/lay.rs` with R9 applied (the probe checkout `.worktrees/fn-364-r9`). The step now takes the phytomer's stretch beside it, `step((phytomer, stretch), …)`.
- **The allotted length** is `grown = internode × scale × varied` (l. 242). It is the internode at stretch 1, the phytomer's `scale` without its stretch.
- **The laid length** is `length = internode × scale × stretch × varied` (l. 241).
- **Callers:**
  - the settle's lay (`geometry::lay`) passes the settled phytomer's stretch;
  - growth's lay (`draw_bent`) passes its unit's stretch and no longer folds it into the growing phytomer's scale;
  - `lay_tests` passes 1.

At stretch 1 the two lengths are the same `f32`, so every lay at stretch 1 is the same to the bit.

## Moved to the allotted length (the shoot's own steering)

| term | line | what it was |
|---|---|---|
| tropism towards the elevation, `tropic = tropism × turning × grown` | 243 | `× length` |
| the bend memory's relaxation, `falloff_keep(grown / bend_length)` | 269 | `length / bend_length` |
| the bend memory's turn, `turns(|curve| × grown)` | 275 | `× length` |
| the wander's turn where there is no memory, `turns(wander × grown × signed)` | 284 | `× length` |

The wander's kick into the bend memory (`curve + pivot × draw × rest(keep)`, l. 271 to 273) is a draw per node scaled by `rest(keep)`. It follows the relaxation above, so it now reads the allotted length too, through `keep`.

## Kept on the laid length (mechanical)

| term | line | why |
|---|---|---|
| the tip's travel, `next = tip + direction × length` | 358 | the stretch is travel |
| the run the reach and the pulls' fades are measured over, `run += scale` and `(offset + run + scale × 0.5) / total` | 316, 328 | `total` is the axis's laid reach (`geometry::reaches`, laid lengths), so run and reach stay in one measure |
| the sag's moment and lever, `sag_angle(load, (phytomer, length), …)` | 297 | a beam's lever is the wood as laid |
| the step returned, `Ok(length)` (growth's path, the pull's `total` in `draw_bent`) | 387 | laid |

## Neither clearly: left on the laid length, listed for the host

| term | line | why it is neither |
|---|---|---|
| the landing ease, `support(tip_z − radius, radius, length)`, its zone `CONTACT × radius + LANDING × length` | 351, 447 | a turn spread over `LANDING` internodes near the ground: geometric in its zone, steering in its intent |
| the resting clamp, `rested(tip, …, (radius, length))` | 372 | keeps the step's length on the ground; travel, but it turns the step |
| the take-over's fade, `over = take × fade`, `fade = 1 − (run + scale/2) / total` | 316 to 333 | the turn RELAYS.md measured inside each relay (about 22°). It is spent over the relay's own laid reach, so it does not shrink with the stretch. The rule as written leaves it, since its measure is the reach. |
| the pulls handed on from the axis a relay carries on (`handed`, `inherited` lifts) | 151 to 190, 312 to 325 | read with `run` and `total`, laid; the same question as the take-over |

Outside the lay, nothing changed: `stand.rs`, `reaches`, `unfold.rs` and the pipeline still read `Phytomer::length` (laid). Growth's stand reads the growing scale, which is now `bare × size` during the step and `bare` outside it, as before fn-364.
