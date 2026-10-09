# A prepared tree meets master's memory and frame rate

## Goal & Context

The owner's release bar (fn-237 R0, 2026-10-09): "I would really like to ship something that is at least as good as the previous version", and "if we get way better fidelity for slightly less performance that's a trade we can live with. But as of now performance is way worse." The scorecard (`EQZB24YMij8rnJgEXh36rz`, scratchpad `parity/SCORECARD.md`) shows memory behind master on every species (palm 1→106 MB, spruce about 400→1,450 MB) and frame rate behind for the ordinary tree and both Two Trees. Astra (fn-245 `ASTRA-ARCH.md` §6): faster generation cannot close this alone. `roots.rs:119` keeps several per-run buffers and `pool.rs:43` keeps another build's scratch on purpose; whether those bytes are waste is unknown. Independent of the growth stages; release item.

## Requirements

- **R1 (where the bytes are, checked against code):** per preset, resident GPU and CPU memory split into the drawn tree, growth scratch, pools and duplicates (laid versus packed), each with its consumer and lifetime; frame time per pass at whole/mid/close.
- **R2 (cut):** growth scratch retired after preparation, retained pools bounded, duplicate representations removed, and emitted detail chosen by a measured geometric error; each removal names the consumer it checked.
- **R3 (proved):** every shipped preset at or under master's resident memory and frame time at whole, mid and close, at the same age as master and at the product default age, both reported (scorecard rows that compared different ages are redone); any remaining loss is named with its measured size and the fidelity it buys, for the owner to accept or refuse; stills and motion viewed; Astra's agreement; gate and npm test green.

## Boundaries

Memory and frame time of a prepared tree. Release item for fn-237.
