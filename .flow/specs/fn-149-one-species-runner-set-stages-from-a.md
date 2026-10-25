# One species runner: set stages from a name to an accepted tree

## Goal

The date palm run (fn-80, fn-82) proved the onboarding method — literature sets the starting tree, a reviewer compares renders with photographs, tuning moves dials, a stall names a real generator gap — but the machinery cost more than the method: on the run's last day about 30 stops came up and four were real work, the host wrote 49 decision files by hand, and the conductor, pipeline and tuning held about 33,000 lines (plus the 2,636-line gap loop). This spec does to the species runner what fn-102 did to the build: one path with set stages, each writing one artifact the next reads; the only stops are an identity gap waiting on its spec and the owner's eye.

Owner quotes (2026-09-24):
> "We also want the simplest lightweight species adding runner that can take a species or a picture of a tree gather documentation, compile it into a catalog entry and start and finish the tuning and gap analysis quickly and produce specs to close those gaps. Differentiate between identity defining specs that are part of developing the species or global specs that apply to many/most species. Bloat has to be gone. Design has to be lean. How we now refactored the generator pipeline having simplified all to one path."
> "i agree with cleaning up first before merging to master. Photo input is a follow up spec"
> "don't finish until we agree on complexity is adequate and it's lean and elegant"

## Decisions

- One binary `species <id>`, eight stages behind one `Stage` trait (`STAGES: [&dyn Stage; 8]`), rerun only when an input's content hash changes (never the build id); flags `--until`, `--stage`, `--status`, `--record`, `--replay`, `--tools`. Stages: Sources (discover, fetch, self-admit; unavailable sources skipped and logged), Profile (extract..fit -> `packet/profile.json`, `references.json`, the reference inventory), Capability (`gate.json` from the host's `packet/capability.json`, before Catalogue), Catalogue (the `catalogue/<id>/` folder passing `scripts/catalogue-check.mjs`), Start (profile -> starting overlay), Tune (rounds from the last kept tree over live dials until nothing is kept), Gaps (`gaps.md`: reachable / identity / global; the runner never mints a spec), Accept (owner looks in the harness; writes the preset and catalogue pins).
- Owner decisions (2026-09-25): unavailable sources are skipped, never waited on; a claim the search cannot settle is kept as its sources' range (contradicted measurement) or left `unsourced` for Tune, a person's `resolutions.json` still wins, `--settle-claims` restores the stop, Jev supplies no number; the inventory belongs to Profile; Capability runs before Catalogue; the add-species agent writes the article prose; Accept writes a preset function in today's style; render tools build only when needed; `veto/` and the contact sheet stay; the proof is a recorded run replayed offline.
- Kept: the literature stages and Jev selections, the reviewer receipt, the no-progress stop, stage idempotence, the catalogue scripts; labelled sets stay as threshold tests, never runtime gates.
- Removed: the conductor (plan/step/state/dependency/policy/dispatch/...), the pipeline gap loop and route table, pilot authority, priority re-approval, continuation, caps, preflight pauses, the second reviewer path and Codex twin scripts, three of four result writers.
- Not here: photo input (fn-147), variation ranges (fn-146), continuity gates beyond R3 (fn-148), any generator change.

## Acceptance Criteria

- **R1:** `species <id>` runs the eight stages in order, each writing its artifact, and a second run with no changed input reruns nothing.
- **R2:** A run stops only for an identity gap and the owner's look; every other condition reruns or is logged, and a claim the search could not settle is kept as its sources' range or left unsourced (owner, 2026-09-25; `--settle-claims` restores the stop for claims).
- **R3:** A revision starts from the last kept tree, reads the dial table live and offers only live dials; the render tools it uses are rebuilt from the same commit.
- **R4:** `gaps.md` classes every failing trait as reachable, identity or global, each with its evidence, and the runner writes no spec.
- **R5:** Accepting writes the preset and the catalogue pins with no hand-editing, and the catalogue folder the run writes passes `scripts/catalogue-check.mjs` as written (the date palm's first folder failed it on four counts: a `catalogue-sources` schema name, references citing unrecorded sources R1 and R2, an unversioned `packet/specimens.json` and an article decision left open by a script crash, so the palm shipped in npm 0.1.3 without its catalogue entry).
- **R6:** The proof is a recorded run replayed offline (owner, 2026-09-25). The European beech was recorded live from a bare seed with `species european-beech --record`, keeping every external answer it received: Firecrawl searches and scrapes, Jev calls, the Wikimedia Commons API and image bytes, and every vision adapter reply. `tests/replay.rs`, inside the workspace gate, replays it with `--replay` from the same bare seed through Start with no network and no key, asserting each stage's word, and a second run reruns nothing; a request the recording lacks fails the run, naming it. Every defect a later live run finds becomes a recorded case beside it. The beech's values are not the proof: the literature stages are replaced by fn-157, and tuning's first live revision and the beech's values move to fn-157's proof. The date palm is not re-tuned: its folder passes `scripts/catalogue-check.mjs` and its Start overlay is compared against the shipped palm preset per dial (`palm-offline-start.md`).
- **R7:** The removed modules are gone, not disabled; the workspace gate is green; the lines before and after are reported.
- **R8:** At close, the host accounts for each of the 20 fn-80-stack specs whose PRs the 2026-09-24 stack cleanup closed unmerged (fn-113, fn-114, fn-116 to fn-119, fn-121, fn-122, fn-127 to fn-133, fn-135 to fn-137, fn-139 to fn-142): landed in this diff (closed), removed by design (closed as superseded, reason named), or still needed (left open, unready). The implementer's done summary names which of them its rewrite carried; the host settles each spec against the merged diff (owner, 2026-09-25).

## Result

All R1-R8 met. 46 commits from `fa0f1c98` to `03a1584c`; codex review SHIP at `03a1584c`. The beech was recorded live from a bare seed on 2026-09-25 and `tests/replay.rs` replays it offline through Start inside the gate (fixture `crates/telperion-jev/tests/fixtures/replay/european-beech`, 7.6 MB in 247 files; non-open pages keep only the passages the run quoted, checked by `tape_trim`; unpushed history rewritten so full pages never entered it). The date palm folder passes `catalogue-check.mjs`; its Start overlay derived 19 values compared per dial against the shipped preset (`palm-offline-start.md`). Crate `telperion-jev` src 36,881 -> 28,584 lines, tests 22,936 -> 17,743, plus the 4,964-line conductor removed. Gate 1,019/1,019 at `03a1584c` (first run lost four core memory-ceiling tests to host load); `npm test` 129; reviewer scripts 12.

R8 settlement: carried and closed — fn-113, fn-114, fn-116, fn-118, fn-119, fn-122, fn-127 to fn-133, fn-135, fn-136, fn-137, fn-139 to fn-142; fn-117 partly (no-progress stop kept, cap removed by design); fn-121 removed with the conductor.

## Open

- The beech's values and Tune's first live revision moved to fn-157.

## Friction

Fifteen entries, kept in `FRICTION.md` because open fn-165 cites it. Dispositions: one task for a 57,000-line rewrite -> ordered checkpoints; dcg refusals of `git checkout <ref> -- <path>`, heredocs and `git rm -r` -> local setup; tests built on removed pauses (~60 min) -> assert through `command::run_with`; carried article script turned `npm test` red -> refresh articles with the validating script; serde_json vs Node key order -> canonical JSON (fn-165); review cap spent before an owner re-plan -> reset rounds on re-plan; no documented tuning config or photograph source -> `--init-tuning` (fn-165); Firecrawl rate limit dropped two sources -> retry on rate limit; ledger lacks hashed state -> fn-165; reviewer script tests in no gate -> fn-165; run-specific paths in the recording -> fixed in the build; memory ceiling under load -> fn-165.

## Evidence

- `.flow/evidence/fn-149-one-species-runner-set-stages-from-a/palm-offline-start.md` (R6 palm Start overlay vs shipped preset; cited by fn-155)
- `.flow/evidence/fn-149-one-species-runner-set-stages-from-a/FRICTION.md` (cited by fn-165)
