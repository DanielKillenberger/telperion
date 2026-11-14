# Review brief: Telperion goal spec fn-321

You are an independent reviewer. Read only; change no file, commit nothing, push nothing, and start no test suite or GPU run. Cheap read-only inspection of code and evidence is what is wanted.

Repository checkout: `/home/daniel/Projects/telperion/.worktrees/fn-319` (branch `fn-319-the-release-audits-slice-lands-before`, with uncommitted notes).

Read first:
- `.flow/specs/fn-321-020-one-grown-tree-at-any-scale-in.md`: the spec under review.
- `/home/daniel/.claude/plugins/cache/flow-next/flow-next/9.4.0/skills/flow-next-flow/references/goal-mode.md`: the contract the autonomous run follows.
- `AGENTS.md`, `STRATEGY.md`, `docs/principles.md`.
- `.flow/evidence/fn-237-release-the-tree-space-engine-replaces/FIRST-PRINCIPLES-2026-10-31.md` and `AUDIT-2026-10-26.md`.
- `.flow/evidence/fn-245-a-breakthrough-in-generation-time/COUNCIL.md`: earlier attempts at coarse growth.
- `.flow/evidence/fn-321-020-one-grown-tree-at-any-scale-in/JEV-VISUAL-2026-11-01.md`.
- The code the spec's claims rest on, chiefly under `crates/telperion-render/src/generation/`, `crates/telperion-space/`, `crates/telperion-core/` and `src/field/`.

The purpose: the owner intends to start this as an unattended `flow --goal` run, stopping at 19:00 Zurich time on 1 Nov 2026, while away. It must be able to run without asking anyone. It starts from the release branch plus fn-319, on an integration branch `goal/<branch_name>`.

Judge the spec on these questions, and report what you find, including anything else that matters:

1. **Runnable unattended.** Can a run follow it from the first action to a defined stop without a human? Where would it stall, loop, or have to guess? Does anything conflict with the goal-mode contract, such as how the integration branch is created or where the final pull request goes?
2. **Checkable criteria.** Can each R-ID be met or failed by a named observation? Name any threshold that is unmeasurable, contradictory, unreachable on what the evidence shows, or that passes without delivering what the owner asked for.
3. **Claims against code.** Check the factual claims in Goal & Context, Decision Context and the criteria against the code and evidence. Say which hold, which do not, and which you could not check. In particular, can the benchmark in R2, the phytomer test in R1, the "full cycle" comparison in R4 and the native entry in R8 be built from what exists?
4. **The route.** Is the proposed route (coupled scaffold, trained closure, detail on demand) sound, given earlier results in COUNCIL.md? What is the strongest reason it fails, and does the spec's order of work and its route stop catch that early and cheaply?
5. **Scope and conflicts.** Do the criteria fit one goal, or does some part need to leave it? Do they conflict with AGENTS.md or docs/principles.md? Is anything missing that the owner's quoted words in Conversation Evidence ask for?
6. **Resources.** Are the limits safe on this machine: one RTX 3080 with 10 GB, 32 threads, 31 GB RAM, 44 GB free on `/home`, each worktree with its own `target/`, a full gate of about 20 minutes, and about 18 hours of run time?

Output, in this form and nothing else:

- A verdict on the first line: `READY`, `READY WITH CHANGES` or `NOT READY`.
- Findings ordered most severe first, each with: severity (P0 blocks the run, P1 likely wastes the run or misleads the owner, P2 worth fixing), the R-ID or section, what is wrong with evidence (file and line, or the measurement), and the concrete wording or change you would make to the spec.
- A short list of what you checked and found sound.
- What you could not check.

Mark each claim as measured, read from code, inferred or a guess. Write plainly. Keep it under 1,200 words. Be direct: a finding that is wrong costs little, and a real defect you soften costs the owner a day of unattended compute.
