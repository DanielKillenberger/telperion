# Friction - fn-85 leaf placement holds one copy in twelve bytes

## 2026-09-19 - the spec is three crates wide and the dispatch carried a 90 minute timebox

Implementing fn-85. The spec's eight criteria reach `foliage::cull`, the
`Instances` representation, six reading sites in the core, three WGSL shaders
(`foliage`, `select` and `shadow` - the spec names two and `shadow.wgsl` binds
the same buffer), the Wasm slot-5 boundary, the browser and harness TypeScript
that reads sixteen floats a leaf, the committed species digests, and evidence
that needs `npm run species:qa` across five species, a peak-RSS poll of four
test binaries and a headless still per species.

Cost: the whole 90 minute timebox bought R1 alone - the one-copy cull, which is
a factor of two on the resident crown. R2 to R8, the twelve-byte encoding, were
not started. Roughly 35 minutes of the box went to reading the surface before
the first edit, because the call-site count is only visible from a repo-wide
grep; the remaining time went to R1 and its gates, and one full workspace test
run is about four minutes each time.

What would have removed it: this spec wanted splitting before dispatch. R1 (one
copy) and R2-R8 (twelve bytes) are independent - R1 needs no representation
change and lands green on its own, as it now has. The owner's "one task per
spec" rule is about not splitting a spec into tasks; it does not say a spec may
not be sized to a session. A second, cheaper fix: a timeboxed dispatch should
carry the spec's own estimate, and a spec whose acceptance names `species:qa`
plus four RSS measurements plus five headless stills cannot close inside 90
minutes whatever the implementation costs, because the evidence alone does not
fit.

Also: `/usr/bin/time` is not installed on this machine, and the sandbox denies
`git checkout <ref> -- <path>`, so the before number for the RSS comparison
could not be re-taken under the same method inside the box. Only the after
numbers below are this session's own. A local setup note, not a repo change.
