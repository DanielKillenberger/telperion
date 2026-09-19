# Friction

## 2026-09-19 — Admission policy needs a host decision

Status: NEEDS_HUMAN. Stopped before implementation under the dispatch's
system-design escalation rule.

While tracing the specimen buffers and the species runner for R4, the admission
contract left two decisions unresolved:

- R4 requires a minimum of one admitted seed and a memory bound on any host.
  If even one specimen exceeds the available budget, those requirements cannot
  both hold. The host must decide the behavior in that case.
- The architecture requires a reserve but does not define its size or which
  bound the VmHWM assertion checks. The prediction describes a finished specimen,
  whereas the process also holds temporary allocations: surface construction
  allocates distance, path and frame buffers (`src/surface.rs`), and the species
  digest serializes the skeleton (`tests/species.rs`). Both paths are under
  `crates/telperion-core`. The host must define how the reserve covers this memory
  and what total-process limit R4 requires.

Cost: one preimplementation inspection pass; zero pilot ticks, build/test runs,
or implementation attempts. No repeated measurements were needed to establish
this contract conflict.

What would remove the blocker: a host clarification in the existing fn-87 spec
of the reserve/bound and the behavior when one seed cannot fit. No new spec or
admission policy was authored by this worker.

## 2026-09-19 — Prediction inputs need a host decision

Status: NEEDS_HUMAN. R4 remains deferred as instructed; this blocker is in
R1–R3, before any prediction implementation.

The architecture says that the grown tree carries the station and vertex
counts. Inspection found that `Tree` carries nodes, crossover and diagnostics,
but no station counts, child counts or surface-run counts. The existing
builders derive the missing topology using allocated scratch:

- `src/surface/paths.rs` builds collapsed-node adjacency and run paths. Surface
  vertices depend on the resulting path lengths, run caps and buried trunk
  samples, not just the number of nodes.
- `src/foliage/placement.rs::bearing_runs` builds child lists and joins a run
  only where the parent has exactly one bearing child with the same branch ID.
  `src/foliage/station.rs` rounds the length of each resulting run to stations.
  Summing rounded edge lengths is not the same count.
- `src/foliage/short_shoots.rs::each` adds the beech's short shoots using a seeded
  phase and tests each candidate's radial position against the crown floor.
- `src/foliage/station.rs::reserve` grows the stored-leaf vector per run or
  short-shoot batch with `try_reserve`. Both clumping and culling retain its
  capacity. Station length times `size_of::<Leaf>()` alone therefore does not
  describe the allocation a finished specimen keeps.

All paths above are under `crates/telperion-core`. The current stored leaf
type itself is available as `foliage::Leaf`; coupling byte arithmetic to that
type is straightforward. The unresolved part is obtaining the counts and
retained capacity within the allocation-free O(nodes) contract. Reusing the
current counting paths would allocate; persisting new counts in the tree or
changing builder reservation would change storage; substituting edge counts
or an average capacity factor would choose an approximation policy. None of
those changes was selected by this worker under the system-design escalation
rule.

Cost: one source-inspection pass and one read-only foliage scout; zero pilot
ticks, prediction implementations or repeated measurement runs. The scout
finished and was reconciled before this report was staged.

What would remove the blocker: host direction on the count source and capacity
accounting consistent with the no-allocation, O(nodes), and unchanged-storage
requirements. No admission decision, specification change or new task was made.

## 2026-09-19 - a spec architecture claim nobody had checked cost two bridge dispatches

- **Doing:** dispatching the routed implementer (codex `gpt-6-astra` at high) at
  fn-87 under the no-plan route, then re-dispatching it with R4 deferred.
- **Hindered by:** the spec's Architecture section states that "the tree carries
  the station and vertex counts, so the prediction is an O(nodes) pass that
  allocates nothing". `crates/telperion-core/src/tree.rs:76` carries `nodes`,
  `crossover` and `diagnostics`, and nothing else. The premise R1 rests on is not
  true of the code, and the one route that would make it true - persisting the
  counts on `Tree` - is closed by the spec's own Boundaries, because `Tree` is
  `bincode::serialize`d into the committed digests
  (`crates/telperion-core/tests/species.rs:39`) and the spec changes no stored
  byte. Two dispatches were spent reaching that: the first stopped on R4's
  undefined reserve, the second on this.
- **Cost:** about twenty-five minutes of wall clock, two bridged dispatches
  (61k and 65k tokens), two wrapper agents, and no implementation. The baseline
  measurement run that preceded them - four thread counts, about four and a half
  minutes - was useful and is kept in BASELINE.md.
- **What would remove it:** an architecture bullet that asserts a property of the
  code cites the file it was read from. This claim was one `grep` from being
  checkable, and a citation would have surfaced the gap at capture rather than at
  the third agent down. The same citation habit would have caught R2's "all five
  catalogue species", where only four of the five catalogue folders have a preset.
- **Early return:** taken. This goes up as NEEDS_HUMAN rather than a third dispatch.

## 2026-09-20 - the workspace gate ran twice because no command says which profile is the gate

- **Doing:** verifying the fn-87 implementation before `flowctl done`.
- **Hindered by:** the repo names two full-suite commands and neither is marked
  as the gate. `package.json`'s `rust:test` is `cargo test --release
  --workspace`, `Cargo.toml`'s comment says "every package script and local
  command stays on release", and CI runs `cargo nextest run --cargo-profile ci`
  while this spec's own BASELINE.md was measured on the `ci` profile. I ran the
  `ci` workspace suite green, then ran the release one as well because the
  project instruction file names it, and the second run compiled the whole
  workspace again under fat LTO.
- **Cost:** 6 min 40 s for the `ci` workspace run and 10 min 22 s for the
  release one, so about ten minutes spent re-proving a green result. The
  release run also exceeded the harness's 600 s foreground tool timeout and had
  to be moved to the background and waited on, which cost two more turns.
- **What would remove it:** one line in CLAUDE.md, or a Quick commands block on
  the spec, naming the command that is the gate and the profile it runs under.
  The two profiles differ only in link-time optimisation, so one of the two
  runs proved nothing the other had not.
- **Early return:** not taken. The work was finished and green; the waste was
  in the gate, not in the implementation.
