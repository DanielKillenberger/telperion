# The palm's trunk organs: persistent leaf bases and acanthophylls

## Conversation Evidence

> gap loop, round two on `date-palm/gate/onboarding-gate/capability`, 2026-09-22: option `trunk-organs-leaf-bases-and-acanthophylls` in the stronger set (Jev 0.14 behind the rosette's 0.42), change kind generator, generalizes: no.
> user (2026-09-22): "we can do both no? tune and build the missing capabilities?" The owner waived the capability gate for the first pass and the two organ specs are minted as fn-82's dependencies to be built in parallel.

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 30% [user], 40% [paraphrase], 30% [inferred] -->

After fn-108 and fn-109 the date palm's gate names three capabilities the generator does not express. Two of them are on the trunk: `persistent-leaf-base`, the retained petiole bases that clothe the trunk as a 3D organ in the diamond pattern every date palm shows, and `acanthophyll`, the basal leaflets of a Phoenix frond modified as spines. The host's capability assessment of 2026-09-19 reads both as unsupported anatomy: no trait row, no organ slot, no material term draws them (`packet/capability.json`, confidence medium for the leaf bases, high for the spines). [paraphrase]

This spec gives the generator both as one capability set on the trunk, off by default, minted by the loop as fn-82's dependency and built in parallel with the palm's first tuning. When it lands the conductor reruns the stages and a tuning revision. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **The design is the strong tier's.** The conductor routes the design to a high-reasoning model at medium effort; the handoff resolves interfaces, invariants, difficult cases and verification and names what stays unknown. This section records what is checked. [host design]
- **What exists, checked 2026-09-22.** The rosette (fn-109) places fronds at every stem apex on a phyllotactic spiral in `foliage/rosette.rs`; a frond is one placement expanded by `fan` into leaflets. The trunk's surface is drawn by `surface.rs` (flare, lobes, twist) and the bark by the material layer; neither carries a repeating 3D element on the bole. `fn-33` (open) specifies one reproductive organ slot on the foliage layer, which is not this. [checked]
- **The shape the design must land on.** The leaf bases are the rosette's own history: each frond that has fallen leaves its base on the trunk, so the organ follows the same phyllotactic spiral downward from the crown with the frond count per turn, a retained length, a wedge cross-section and a weathering that fades toward the ground; a design that derives them from the rosette's rows rather than authoring a second spiral is preferred. The acanthophylls sit on each living frond's rachis at its base, a spine count and length per frond. Both are off by default; every parameter is a validated row with a doc comment and a dial-table entry; `persistent-leaf-base` and `acanthophyll` move into the vocabulary's expressed list with derivable clauses. [inferred]
- **Mesh budget.** A hundred retained bases of a dozen faces each is cheap; the design states the count and the far draw follows the renderer's rule, the far draw is the near draw minus what the eye cannot resolve. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The palm run's gate, rerun at the landed commit, names `persistent-leaf-base` and `acanthophyll` as expressed and halts on `infructescence` alone, or passes if fn-33's slot has landed. Errors: a gate still naming either name fails.
- **R2:** Every shipped preset is byte-identical in mesh and metrics with both organs absent (species digests and catalogue pins). Errors: a changed pin is a defect.
- **R3:** The date palm at seed 1 builds with leaf bases spiralling down the trunk from the crown and spines at each frond's base, and the trunk view still, rendered once through the headless renderer, is judged by the owner's eye and the reviewer. Errors: bases that do not follow the crown's spiral, or spines on a frond that has none, fail.
- **R4:** The rows are in the dial table with meaning, range and steps; the workspace gate is green.

### Host decisions on the design handoff (2026-09-22)

The strong-tier design (`.flow/evidence/fn80/design-fn-110.md`) escalated two questions; both are settled here.

- **The lattice's axial resolution comes from splitting the stem polyline for the bases, not from a finer `stepDistance`.** A finer step changes the skeleton every preset shares and is not byte-identical; a split of the stem's run for the leaf-base placement is local to the organ and leaves generation untouched. [host design]
- **`persistent-leaf-base` requires the rosette.** The bases are the rosette's own history down the trunk and take their spiral from `rosette::frame`; a base without a rosette would need a second spiral authored for nothing, and no palm carries bases without a crown. The derived clause is guarded on the rosette being on. [host design]
- **The acanthophyll's cross-section stays the leaflet's**, as the design states; the owner's verdict on the trunk still decides whether that reads. [inferred]

## Boundaries
<!-- scope: business -->

- Not the infructescence, which is its own spec on fn-33's slot. No species branch. Not the palm's acceptance, which fn-82 owns.

## Strategy Alignment

- Serves "The catalogue": two more forms the generator could not draw become shared capabilities. [strategy:The catalogue]
