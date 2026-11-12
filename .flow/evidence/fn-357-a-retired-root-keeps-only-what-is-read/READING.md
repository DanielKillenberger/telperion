# fn-357 R1: what is read of a retired root, and until when

Read from the code at `cb634ffee`, before any change. Paths under `crates/telperion-space/src/`. A retired root is an axis whose fate is `Fate::Root(living)` (`grow/cycle/retire.rs:39-40`): a lateral (`retire.rs:252-255`), its subtree closed and evaluated, its fade at the age grown none, its own phytomers and draws' per-unit and per-node vectors already let go (`retire.rs:208-221`). Its descendants are `Fate::Inside` and leave every store at the next compaction (`grow/cycle/compact.rs:134-156`); the root itself never does, today.

## The two readers

Everything still read of a root is read by two passes, both through its bearer.

**A. The settle's fades** (`grow/settle.rs:50-69`, `shed.rs:146-241`), at every settle from then on:
- `mark` multiplies a woken root's `draws.birth[1]` by its kept share (`settle.rs:26-34`, `dormant.rs:345-389`).
- `assign` sets its `vigour` from `draws.birth[0] × draws.birth[1]` (`presence.rs:183-192`); `assign_one` sets `units` (empty), `sleep`, `rank`, `alive` (`presence.rs:215-242`).
- `fades` takes its `lived` and `alive` from its living (`given`, `shed.rs:194-198`), overwriting the `sleep` and `alive` just set. In the backward loop (`shed.rs:199-224`) it carries into its bearer: `weight = vigour × bearer.phytomers[node].scale`, `units`, `gap` and `before` from its `birth` against the bearer's units and sleep, `lived[bearer] = max(lived[bearer], carry(..))`, `alive[bearer] = max(alive[bearer], alive × weight)`.
- Its own fade (`shed.rs:232-239`) reads its `pa` (the delay) and `birth`. It is none at retirement and stays none: `fade` is non-increasing in the age (`contract/presence.rs:102-105`), and with its fade at none its `alive` is 0. So `shed_past` drops it at every settle (`shed.rs:114-121`); nothing else of it reaches the tree.

**B. An ancestor's evaluation** (`retire.rs:226-249`), each time a closed subtree holding the root is evaluated:
- `subtree` includes the root as a stub and does not descend below it (`retire.rs:227-228`, `333-345`).
- `copied` copies its whole `Axis` record (phytomers empty) and its `Draws`, a woken root's `birth[1]` times its kept share (`retire.rs:260-330`).
- `assign_living` and `subtree_living` then read exactly what A reads (`retire.rs:238-242`): `given` is its living, its carry into its bearer as above.

**The kept share is final.** A root holding a bud that woke is evaluated only from three cycles after the waking (`retire.rs:229-236`). `Woken::kept` then reads the bearer's unit after the waking, its stop, its successors' birth draws (`dormant.rs:363-388`), all final by then, and never the last-cycle branch (`dormant.rs:360-362`, the cycle being under the age). So the root's vigour, kept share included, is the same at every later settle and evaluation.

## By store

| Store (per axis) | Read of a retired root | By | Until |
|---|---|---|---|
| `axes[i].origin` (bearer, node) | yes | A, B (`shed.rs:200-211`) | its bearer is let go |
| `axes[i].birth` | yes | A, B (`shed.rs:208-211`; its own fade, `shed.rs:236`, none forever) | its bearer is let go |
| `axes[i].vigour` | yes, as set from `draws.birth` | A, B (`presence.rs:183-192`, `shed.rs:203`) | its bearer is let go |
| `axes[i].pa` | only for its own fade, none forever | A, B (`shed.rs:153-158`) | at once |
| `axes[i]`: lineage, blend, apex_end, base, heading, side, units, alive, rank, sleep, phase, joined | no: copied by B, never read; `alive`, `sleep` overwritten by `given`; `units` read only of a bearer, and a root's children are left out (`shed.rs:200`); `base`/`heading`/`side` read only to frame a child (`geometry.rs:176-232`), and nothing is born on a closed subtree | | at once |
| `axes[i].phytomers` | empty since retirement; `census.rs:86` sums its length, 0 | | at once |
| `draws[i].birth` | yes, into the vigour | A (`settle.rs:32-33`, `presence.rs:185`), B (`retire.rs:317-327`) | its bearer is let go |
| `draws[i].sleep` | copied into `axes[i].sleep`, overwritten | | at once |
| `draws[i]`: every other field | read only of a bearer (`dormant.rs:345-389`), a relay's parent (`presence.rs:157-163`), an axis that grew (`grow/draw.rs:98-137`) or a sleeper's bearer (`grow/wake.rs:44-50`); a root is none of these | | at once |
| `pencils[i]` | no: the balance, allotment and shade read living apexes (`grow/cycle/passes.rs:46-62`, `127-132`; `grow/sketch.rs:113-115`), the layout the axes that grew and a new axis's bearer (`grow/draw.rs:98-103`, `179-186`, `235-311`) | | at once |
| `horizon[i]`, `roots[i]` | no: read of a new axis's bearer (`grow/shoot.rs:176-191`, `282-320`, `grow/wake.rs:29-33`) | | at once |
| `units[i]` | no: read of a relay's bearer (`grow/shoot.rs:345-347`) | | at once |
| `successor[i]` | no: read of a woken bud's bearer (`dormant.rs:350-383`) and a sleeper's bearer chain; a root is a lateral, in no axis's successor list | | at once |
| the woken bud it is (`woken[k]`, `retire.woken[i]`) | yes, its kept share | A (`settle.rs:26-34`), B (`retire.rs:317-322`) | final from retirement (above) |
| `working.links[i]`, `working.pa[i]`, `working.by_node` | the root as a child of its bearer: the balance's sums, the split and the order skip a closed child (`working.rs:209-213`, `234-238`, `263`, `335`); B's `subtree` and an ancestor's `let_go` walk to it (`retire.rs:211-219`, `333-345`); compaction walks its children (`compact.rs:138-146`) | B, retirement, compaction | its bearer is let go |
| `working.open[i]` | read as closed: the sums above, `Working::closed` for the wood (`girth.rs:92-94`, which returns 0.0 and adds it to the bearer) and the deposit (`grow/deposit.rs:29-31`) | | at once |
| `working.at`, `light`, `cost`, `living`, `sub`, `incoming`, `bud` | no: read for open axes only (`working.rs:205-295`, `passes.rs`) | | at once |
| `retire.fate[i]` = `Root(living)` | yes: B's stub and `given` (`retire.rs:227`, `238-241`), A's `given` (`settle.rs:64-66`), `Retirement::roots` (the census's `retired`, `paused.rs:266`), `tally` (`footprint.rs:117-127`), compaction (`compact.rs:138-142`) | A, B, census | its bearer is let go |
| census `depth`, `order`, `born` (only where a census is taken) | totals only: `ended` and `compact` count every axis, held or let go, alike (`census.rs:101-135`) | | at once |
| `live`, `asleep` | never hold a root: its subtree is closed, nothing lives or sleeps in it | | |

## Until when

Each read above is of the root through its bearer, and lasts while that bearer is held and not itself retired. Once an ancestor retires, `let_go` marks the root `Inside` (`retire.rs:211-214`): no pass reads it again and compaction lets it go. The root's own fade runs out at retirement, so "at once" and "when its fade has run out" are the same moment. Its carry into its bearer never runs out: the bearer's `lived` and `alive` are a maximum over its children's carries, read at every later settle and evaluation.

## What a retired root must keep

Its bearer, its node, its birth cycle, the vigour its birth draws give (its kept share in it where it woke), and its living (`lived`, `alive`): 24 bytes. Every other per-axis store, about 960 bytes per axis on the oak at 120 by `PHASE5-CENSUS.md`'s footprint (axes 303, pencils 272, draws 188, successor 31, links 34, by_node 34, fate 17, horizon, units and roots 31, the working lists 50), is read of it by nothing.

**The order of a carry.** A child's carry enters its bearer by a maximum (`shed.rs:222-223`), of values that are never NaN or -0 (`carry` is `before + weight × (gap + lived)`, each term at least 0, `contract/presence.rs:94-96`). So a kept root's carry taken before the backward loop is the same to the bit as at its index, the bearer's own carry upward coming after every child's.

**No stop.** No read of a retired root needs a field outside the record above.
