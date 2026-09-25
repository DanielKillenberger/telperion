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
