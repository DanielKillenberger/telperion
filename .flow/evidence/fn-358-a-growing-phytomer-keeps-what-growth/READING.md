# fn-358 R1: what growth reads of a phytomer, and at what width

Read from the code at `8d15b662c`, before any change. Paths are under `crates/telperion-space/src/`. "Growth" means everything a grower runs between its first cycle and its settle: the units (`grow/unit.rs`), the rough layout (`grow/draw.rs`, `geometry.rs`, `geometry/stand.rs`, `geometry/lay.rs`), the leaves (`grow/deposit.rs`), the light read at the tips (`grow/sketch.rs`), the year's wood (`girth::lay`), relays (`presence::relay_node`) and retirement (`grow/cycle/retire.rs`).

## The answer in one line

The rough layout reads the geometric fields: a lateral's frame reads its node's tip, heading, side and rank, a woken bud's node's too, and that node can be old. So tip, heading and side cannot leave the grower. But growth writes every geometric and scale field as an exact `f32` image, so each can be held at `f32` and widened back to the same `f64`, bit for bit. `radius` is never written in growth, so the grower can drop it. A grower's phytomer goes from 136 to 72 bytes.

## By field

`size_of::<Phytomer>()` is 136. Writer and reader line numbers are at the base.

| Field | Written in growth | Read in growth | Value stored | Verdict |
|---|---|---|---|---|
| `cycle` u32 | `grow/unit.rs:71` | unit ranges: `grow/draw.rs:348`, `geometry/stand.rs:239`, `presence.rs:199-204` (`relay_node`), `presence.rs:226` (retirement's `assign_one` on a copy); leaf age: `girth.rs:49`, `girth.rs:57` | the cycle | stays, u32: a cycle is not bounded below 2^16 |
| `key` u64 | `grow/unit.rs:77` | `geometry/lay.rs:235` (internode spread), `lay.rs:250` (wander draw) | the node's key | stays |
| `tip` Vec3 | `geometry/lay.rs:377`, `v3(next)`; default 0 | `grow/sketch.rs:233` (light at the tip); `grow/deposit.rs:24`, `:38` (leaf positions, older ones included); `geometry.rs:158` (`end`); `geometry.rs:336-338` (`lateral`: a node's tip and, for a woken bud, the tip below it); `geometry/stand.rs:211-213`, `:278`, `:288` (a relay's stand and base) | `v3` of a `[f32; 3]`, exact (`geometry/q.rs:1-5, 21-23`) | read; narrows to `[f32; 3]` |
| `heading` Vec3 | `lay.rs:378`, `v3(direction)` | `geometry.rs:158`, `:328` (`q3`); `stand.rs:285` (`q3`) | exact `f32` image | read; narrows to `[f32; 3]` |
| `side` Vec3 | `lay.rs:380`, `v3(..)` | `geometry.rs:158`, `:328`; `stand.rs:290` | exact `f32` image | read; narrows to `[f32; 3]` |
| `radius` f64 | never: written only by `girth::thicken` at the settle (`girth.rs:148`) and by `unfold.rs:499` on settled trees | `lay.rs:288` (sag, only with a load, which the rough layout never passes: `grow/draw.rs:381`), `lay.rs:343` (landing): reads 0.0 throughout growth | 0.0 | leaves the grower; the narrow record reads as 0.0 |
| `scale` f64 | `unit.rs:76` (1.0); `grow/draw.rs:363`, `:379` (`f64::from(f32)`), `:384` (restored from `:363`'s value) | `draw.rs:275`, `:372` (`as f32`); `lay.rs:231` (`as f32`); `stand.rs:155`, `:161`, `:171`, `:258`, `:302` (`as f32`); `girth.rs:50` (leaf, in f64), `girth.rs:112` (`established`, `as f32`); `deposit.rs` via `girth::leaf` | exact `f32` image | read; narrows to `f32` |
| `grown` f64 | `unit.rs:80` (0.0); `draw.rs:364` (`f64::from(f32)`) | `stand.rs:150` (`as f32`) | exact `f32` image | read; narrows to `f32` |
| `size` f64 | `unit.rs:79` (1.0); `draw.rs:365`, `draws.shade(k)`: each entry a pencil's size (`grow/advance.rs:184`, `grow/stop.rs:116`), set from `allocation::sized` (`allocation.rs:196`, `f64::from(f32)`) or 1.0 | `draw.rs:372`, `:379` (`as f32`); `girth.rs:50` (leaf, in f64) | exact `f32` image | read; narrows to `f32` |
| `rank` f64 | `unit.rs:78` (0.0); `draw.rs:366`, the pencil's rank: 0.0 at birth (`draw.rs:294`), then `f64::from(f32)` (`draw.rs:367`) | `geometry.rs:323` (`as f32`); `stand.rs:286`, `:291` (`as f32`) | exact `f32` image | read; narrows to `f32` |
| `section`, `sapwood` f32 | `girth.rs:113`, `:117` | `girth.rs:111`, `:115` | f32 | stays, f32 |

**Widening is exact.** Every narrowed value is an `f32` image when stored, so `f64::from` of the `f32` gives back the same `f64`. A reader that computes in `f64` (`girth::leaf`, the light reading the tips and leaves) therefore reads the same value as before, and so does one that computes in `f32` (`q3`, `as f32`). No reader needs more than the `f32` that was stored.

**Retirement.** `retire.rs:327-331` copies a closed subtree's phytomers into full records for the settle's `assign_living`. That rewrites scale, grown, size and rank from the draws and reads `cycle` (`presence.rs:226-231`). `subtree_living` then reads the scale just assigned (`shed.rs:237`, `:248`). The copy can be widened from the narrow record.

**The settle.** `assign` rewrites scale, grown, size and rank of every phytomer (`presence.rs:226-231`), `place` rewrites tip, heading and side (`geometry/lay.rs:430`), and `thicken` writes radius (`girth.rs:148`). So a laid tree reads only cycle, key, section and sapwood from the grower. The test-only unlaid sketch (`grow::sketch`, `lay` false) also keeps the rough layout's tips and frames. The settle widens every field, so both paths stay the same to the bit.

## What the grower holds of a phytomer after this

`cycle` u32, `key` u64, `tip`, `heading` and `side` as `[f32; 3]`, `scale`, `grown`, `size` and `rank` as `f32`, `section` and `sapwood` as `f32`: 72 bytes against 136. The settle adds `radius` and the `f64` widths.

## A pencil, and a closed axis's records

This task changes neither. The reading is below.

**Pencils of a closed axis are read by nothing.** Every pass reads only open axes' pencils:
- The balance and the allotment read living apexes or the working list, which holds only open axes (`grow/cycle/passes.rs:46-62`, `:85`, `:99-113`, `:127-141`).
- The light read and the shoot's reads (`light`, `rests`, `size`, `kept`) are for living apexes (`grow/sketch.rs:113-115`, `grow/shoot.rs:243-265`).
- The layout reads the axes that grew this cycle and a new axis's bearer, which is open (`grow/draw.rs:98-103`, `:179-186`, `:235-311`).
- A closed axis never reopens (`girth.rs:76-78`).

Once an axis closes, its pencil could leave: 272 bytes per axis at 120 by fn-357's footprint, with 928,424 closed axes held.

Pencil scalars whose writer stores an `f32` image are `base_scale` (`draw.rs:292`), `running` (`draw.rs:387`), `rank`, `size`, `memory` (`passes.rs:101`) and `kept` (`passes.rs:112`). `light` (`sketch.rs:114`) and `Layer` were not read for width.

**A closed axis's phytomers are read only for their cycle.** An ancestor's evaluation copies them, and `assign_one` reads `cycle` and rewrites the rest. The other reads skip a closed axis: `girth::lay` (`girth.rs:92-94`), the leaves (`deposit.rs:29-31`), and frames, which are taken only from a new axis's open bearer. The settle reads key, cycle, section and sapwood. So once an axis closes, its phytomers could keep only those, 24 bytes.

## No stop

No field growth reads has to stay at full width to give the same result. `tip`, `heading` and `side` stay in the store because the rough layout reads them, but at `f32`. That is the design's stop clause applied: they do not leave, and they narrow because each stored value is already an `f32` image.
