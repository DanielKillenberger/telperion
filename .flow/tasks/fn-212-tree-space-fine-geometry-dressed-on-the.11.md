---
satisfies: [R5]
---
# fn-212-tree-space-fine-geometry-dressed-on-the.11 Astra's implementation review: nine findings

## Description
The spec's R5 review. Astra's findings: deferred destroys drained by an unrelated submit; Oversize unwrapped into panics; tied widest nodes publishing mixed coordinates; fine foliage bypassing max_instances; octahedral sign(0) reversing frames; divergence narrowed to f32 unreduced; zero leaf size still placing instances; lobed surfaces and surface_contact=0 refused by the fine path; test-only run_words kept alive in production. Each finding is verified by a test that fails before the fix (a finding that does not reproduce is recorded with the attempt) and then fixed.

## Acceptance
- Each of the nine findings reproduced red then fixed green, or shown not to reproduce.
- Gate (two parts, render first) and npm test green.

## Done summary
Astra's nine findings, each tested before its fix:
1. Deferred destroys belong to the encoders open when a buffer is let go (Gpu::encoder, Recorded).
2. Every fine-path io::buffer returns Oversize by name; the device executor keeps its RenderError.
3. Not reproduced on the RTX 3080; fixed as directed: least-id winner, one publish pass.
4. max_instances holds both blocks, refused in the CPU reference's words.
5. Octahedral packing uses a sign-bit signum, as f64::signum.
6. Divergence reduced to a turn in f64 before it narrows.
7. Zero leaf size places no device stations.
8. Unseated leaves lay no ring positions; lobed ring vertices on the device (host option (a)), vertex parity 1.9 um at spruce 80, leaf parity unchanged.
9. run_words no longer live in production.
Parity at 80 unchanged; gate 176 + 988 passed; npm test green.

## Evidence
- Commits: 34e1722a, 528c06f1, 302d81a2, f8143b1c, 7169eea1, 4abd0ac2, 88687a81, ad01db84, 6f4e46a4, f488fdde, ea61df67, feaaf184
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: