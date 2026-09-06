# FN-9.7 — species and anatomy bindings

The Rust binding now owns the five-entry identity catalogue. Numeric ABI IDs 0/1/2 retain Ordinary/Telperion/Laurelin; oak and spruce are 3/4, deliberately listed before the Two Trees to exercise non-positional selection. `presetById` returns an independent parameter copy and rejects unknown identities. `TreeEngine.build` accepts a family or native identity string. `ORDINARY` remains a plain Family baseline; `PRESETS` contains all five entries and `TWO_TREES` preserves the existing comparison scope.

The existing `fields!` table carries a tagged colonizing/spreading/tiered habit, every active habit control, nested `bias.supernatural`, element anatomy/connector length, and canopy attachment. Binding-local serde adapters keep the runtime core serialization-free. `wasm:build` derives the TypeScript union types and catalogue from Rust metadata; no browser schema is maintained by hand. The harness carries native controls without sliders through preset conversion, including element geometry, growth overrides, surface settings, attachment and shell depth. Task 8 still owns the new botanical/supernatural controls and framing modes.

Foliage retains the single prototype/matrix layout. Owned metadata exposes biological unit counts and connector-excluded, half-open vertex/index/section ranges; scalar index offsets are documented. Generic geometry has no asserted biological count. Field-only builds report biological counts but transfer no render geometry or anatomy subsets. Existing optional-stage, revision, disposal and copied-buffer semantics remain intact.

## Validation and intentional fixture changes

- Native Wasm crate: two tests pass, including all five identity/metadata roundtrips, modified habit controls, stored disabled supernatural settings, unknown IDs/keys, invalid variants and ranges.
- Harness: 104 tests pass. Catalogue roundtrips cover all five; the existing full-size geometry/forest tests stay bounded to the Two Trees.
- Headless binding fixtures use height 4 m, 40 attractors, 1200 nodes and 12000 instances maximum. Both species cover anatomy ranges, all transformed vertices within bounds, per-matrix biological counts, determinism, disabled nonzero effects, malformed anatomy, surface-only/foliage-only/field-only behavior, zero-size foliage, release and stale field handles. Existing malformed raw ABI, ownership and viewer failure/retry tests are retained.
- Ordinary historical 13616 nodes / 59810 instances are replaced with explicit local-branch/retained-foliage and packed cardinality invariants plus exact repeat structure equality. Natural defaults and retained clipped laterals intentionally changed those counts.
- Two Trees exact handoffs remain 1075/1649; exact twigs change from 54890/32153 to 55315/32313 because clipped lateral stations now survive. Forest aggregation and deterministic geometry tests remain.
- Surface dial tests explicitly activate lobing because Ordinary now has zero lobe depth. Supernatural geometry tests explicitly enable effects. Bow measured on the changing subset below a height cutoff is not monotonic under natural growth/clipping; the test now requires straight-at-zero, appreciably bent and distinct enabled settings, and valid topology. Separate conversion tests retain exact torsion scaling.
- Typecheck, formatting, diff whitespace, binding-only strict Clippy (`--no-deps`) pass. An additional dependency-inclusive Clippy attempt reports the pre-existing `chunks_exact_to_as_chunks` warning in core `foliage/element.rs:100`; no unrelated native change was made.
- Successive `wasm:build` runs produce byte-identical generated TypeScript and Wasm.

## Environment and captures

The supplied `/tmp/fn20-browser/.../playwright/index.mjs` and `/usr/bin/chromium` were absent. Used the repository's installed Playwright and downloaded its bundled Chromium via `npx playwright install chromium`; all browser runs remain headless. Rust homes match the job instructions.

No new oak/spruce whole PNGs were produced. Existing `.flow/tmp/oak-{1,2,3}-whole.png` are foliage-on captures: the capture runner screenshots before hiding the instanced foliage, and its saved inputs contain 55980/58711/59814 leaf matrices respectively. Seed 1 was also visually inspected; the crown is sparse but foliage is visible. These pre-existing native-export captures are not a new Wasm visual acceptance claim. No spruce whole capture is claimed. Current browser gate artifacts are under `.flow/tmp/fn97-browser/`.

Next job: fn-9.8. No reviews, PR, merge, or task 9 work.
