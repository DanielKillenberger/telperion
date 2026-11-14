# R1 reading: who reads a fade, and whether any reader needs the product form, 12 Nov 2026

Read on `fn-359-a-subtree-fades-once` at `0289bdd02`, before any product change. Code read, no run.

## The two places a fade enters the drawn tree

`grow/settle.rs:50-90` (`settle_shed`) runs `assign`, then `shed_past`, then hands two things on:

1. **The faded vigour.** `shed.rs:139`, `axis.vigour *= fade[i]`, read by `geometry::scale` (`geometry.rs:73-82`): `base_scale[i] = inherited x vigour x share`, the inherited part being the parent's base scale (a relay or continuation) or the node's scale times the parent's base scale (a lateral). The fade is therefore compounded once per bearer.
2. **The raw fade, `Shed::fade`.** `settle.rs:76` passes it to `girth::shares` (`girth.rs:126-141`), whose doc reads: "Every kept axis's share as drawn: its own fade times the share of the axis it stands on, as its scale takes its bearer's (`geometry::scale`)". `share[k] = own[k] x share[parent]`. `girth::thicken` (`girth.rs:147-150`) then sets every phytomer's radius to `sqrt(section x share / pi)`. `docs/pipeline.md:67` and `girth.rs:16-17` state the same ("`share` the fade its axis is drawn at").

## Every reader, against the product form

| reader | what it reads | assumes the product form? |
|---|---|---|
| `geometry::scale` (`geometry.rs:73-82`) | faded `vigour` | yes, and the design changes what it is fed: with factor `A / A(parent)` the product down a line is `A`. Consistent with the design. |
| **`girth::shares`** (`girth.rs:126-141`, called `grow/settle.rs:76`) | **raw `Shed::fade`, multiplied down every bearer** | **yes. Stop.** It composes the raw fade as a product, stated as mirroring `scale`. If the vigour takes `A / A(parent)` and `Shed::fade` stays raw, scale and girth part. |
| `docs/pipeline.md:67`, `girth.rs:16-17` | the share's definition | states the share as "the fade its axis is drawn at", which today is the product |
| `shed::fades`, weights (`shed.rs:242-253`) | `link.vigour` before the multiply at `:139` (fades runs on the assigned, unfaded axes) | no |
| `shed::Stand::vigour` (`shed.rs:79-90`, `retire.rs:274`, `compact.rs:146-175`) | the vigour `presence::assign` gives, unfaded | no |
| `shed::subtree_living` (`shed.rs:282-289`, `retire.rs:280`) | the `Living` carried up, no fade applied | no |
| retirement, `faded_out` and the verdict (`retire.rs:198-210`, `:283`) | one axis's own raw fade, `<= 0` | no: it decides what is lost from the axis's own fade, which the design keeps |
| `shed_past`'s loss test (`shed.rs:129-135`) | own raw fade `<= 0` | no, kept as the design says |
| `grow/draw.rs:272-290` (the grower's rough layout) | birth draws and running presence, no fade | no |
| `unfold.rs:345` | the settled vigour of two trees, mixed | no composition |
| `fine/canonical.rs:127` | settled vigour into the digest | no; pins move where a fade is below one (R5) |
| closed form, `Kept::shed` (`closed_form/kept.rs:244-293`) | its own fade `clamp(delay + 1 - (m - s), 0, 1)` per child table, nested tables faded again at their powers | the nesting multiplies fades. With whole presences and a whole delay the fade is 0 or 1, so product and minimum agree. Whether a fractional delay makes the bound assume the product: not read (unknown). |

## The stop, with numbers

The spec's stop: "Any reader that needs the product form". `girth::shares` is one.

On the diagnosis's limb line at 300 (seed 1001; every axis of the line at fade 0.132, its bearers at 1), taking four axes in a row (lateral 14, relays 15, 26, 27), on paper from the diagnosis's fades:

| axis | share today (product of raw fades) | radius factor today, sqrt(share) | base scale today (diagnosis, settled) | base scale under the design | share if `shares` keeps reading raw fades | share if it reads the applied `A` |
|---|---|---|---|---|---|---|
| lateral 14 | 0.132 | 0.363 | 0.1066 | 0.104 | 0.132 | 0.132 |
| relay 15 | 0.0174 | 0.132 | 0.0134 | 0.104 | 0.0174 | 0.132 |
| relay 26 | 0.0023 | 0.048 | 0.0017 | 0.104 | 0.0023 | 0.132 |
| relay 27 | 0.0003 | 0.017 | 0.0002 | 0.104 | 0.0003 | 0.132 |

(The diagnosis's scales include the phytomer's own factors; the shares are the fades alone. Inferred, not run.)

So building the design to the letter, vigour only, leaves the line's living wood drawn at 0.104 of its length on a radius that still tapers to 1.7% by the fourth axis. Making the share read the applied fade `A` (the product of the factors, as the scale gets it) keeps scale and girth as one, but that is a choice about how girth should fade, which the design does not state. Returned to the host.
