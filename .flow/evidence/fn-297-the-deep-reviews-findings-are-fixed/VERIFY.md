# Host verification of the deep review (2026-10-19)

Passes A to C2e: the owner's codex session, each finding checked by the host against the code. C1e, D2, D3, E1, E3 to E7: read-only `codex exec` from the host (`raw/`), then a fresh verifier tried to refute each.

| ref | verdict | sev | reachable by a shipped preset or the package | where |
|---|---|---|---|---|
| B.1 | real | P2 | no (7.47M sleepers) | fn-297 R4 |
| B.2 | real | P2 | no | fn-297 R4 |
| B.3 | real | P2 | no (2^32 leaves) | fn-297 R4 |
| C.1, C.2 | same-value races | hardening | n/a | fn-296 R1 |
| C.5 | code-confirmed | P1 | palm with cauline 1 and cluster leaves; to be reproduced | fn-297 R2 |
| C.6 | real | P2 | no (bound ±1e9°) | fn-297 R6 |
| C.7 | code-confirmed | P1 | Laurelin's flowers; extent to be measured | fn-297 R3 |
| C.8 | plausible | P2 | edge pixels only | fn-296 R2 |
| C1e:C.9 | real, unreachable | P2 | no (only the palm establishes; no lateral orders) | fn-296 R3 |
| D2:D.1 | real | P2 | yes (Laurelin then oak), waste does not grow | fn-297 R5 |
| D3:D.1 | real, by design | P2 | hidden growth path; freed on dispose | fn-296 R4 |
| D3:D.2 | real | P2 | 32 bytes | fn-297 R5 |
| E1:E.1 | real, metric | P2 | diagnostic | fn-296 R5 |
| E3:E.1 | refuted for the package; latent `OnceLock` | P2 | no | fn-296 R6 |
| E3:E.2 | real | P2 | ~2M florets | fn-296 R7 |
| E3:E.3 | real | P2 | example only | fn-296 R8 |
| E4:E.1 | real | P2 | Rust walks only; field inflated, tree not | fn-296 R9 |
| E5:E.1 | real | P2 | Rust `blend::centroid` only | fn-296 R10 |
| E6:E.1 | likely | P2 | contrived | fn-296 R11 |
| E6:E.2 | real, test only | P2 | no | fn-296 R11 |
| E7:E.1 | real | P1 | yes: `createRenderer` and `prepareField` on one page | fn-297 R1 |
| E7:E.2 | real | P2 | benchmark | fn-293.8 |
