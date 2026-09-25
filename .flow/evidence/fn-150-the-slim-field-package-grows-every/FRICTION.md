# fn-150 friction

## 2026-09-25: slim Wasm size stop

- Doing: measuring `telperion-field.wasm` after routing the slim crate through `pipeline::build`.
- Hindered: the brief says to stop if the fallback "pulls in much more code", but gives no threshold. The result was +48% (355,100 to 526,965 bytes), so I stopped for a host decision after R1 and the fix.
- Cost: about 5 minutes. The build itself is fast (about 10 s for the slim Wasm).
- Would remove it: a size budget in the spec, stated as bytes or percent, raw or gzip.

## 2026-09-25: the host's frond shape fails the visual check

- Doing: the visual check of the planned palm field against the placed one (host design, second round).
- Hindered: the design fixed each frond's reach at the leaflet extent (about 1.97 m for the palm), and I only saw that this makes the crown a round bush after building and measuring it. Agreement is 81%; the plan reports 2.8 times the placed foliage cells.
- Cost: about 40 minutes of build, plus a second NEEDS_HUMAN round trip.
- Would remove it: check a primitive's shape against the organ's in the design step, or have the host render one palm's field before specifying the shape. A two-line calculation of reach against frond thickness would have shown it.

## 2026-09-25: the ribbon took three fits to reach the target

- Doing: fitting frond ribbons to the host's accuracy target.
- Hindered: the first ribbon took worst-case slack for the leaflet draws (3.54 times the placed cells, worse than capsules). Replaying the draws gave 1.77, one rolled ribbon per row 1.64, and only the flat-box ribbon with a cell-to-box test reached 1.26. The unstable sort the host asked for was 16 KB against the stable sort's 8.5 KB, so an insertion sort replaced both.
- Cost: about 45 minutes, and five palm field runs of under a second each.
- Would remove it: a design step that names the primitive's cell test and whether draws are replayed or bounded, and a Wasm size measurement per candidate before a fix is prescribed. Twiggy's hashed crate names also need a normalising diff script, rewritten here each time.

## 2026-09-25: the agreement target was set against an inflated reference

- Doing: tapering the ribbons to the owner round's targets (1.10 times the placed cells, 97.5 % agreement).
- Hindered: agreement stayed near 95.4 % through every chord count and every fit. Only after five tries did a ceiling measurement show that the exact leaflets themselves agree with the placed field on just 95.1 to 95.3 %, because the placed field answers from world-aligned leaf boxes.
- Cost: about 30 minutes, and eight palm field runs.
- Would remove it: measure a target's ceiling (the exact geometry against the reference) before setting the target. Keep `examples/palm_field_ceiling.rs` for that.
