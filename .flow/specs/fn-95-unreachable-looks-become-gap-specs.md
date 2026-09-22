# Unreachable looks become gap specs people contribute

## Conversation Evidence

> user (turn 1, part 1): "check the fn-68 and corresponding add-species runner spec. I want to rework the UI to basically take many of these building blocks and expose them natively => connect your claude code / codex and express what your tree should look like."
> user (turn 1, part 2): "Jev will propose tuning sets and the llm will set the parameters. You can then iterate this way and if you're unable to get something done create flow-next spec that would close the gap."
> user (turn 1, part 3): "People could then make pull requests with those specs or done implementations."
> user (turn 2, selected): "i think we can combine a + b?"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 50% [user], 30% [paraphrase], 20% [inferred] -->

In the tree studio a person describes a tree and iterates with their own agent and Jev's proposals. Some looks will not be reachable with the dials the generator has. The owner wants that moment to produce something useful: the agent writes a flow-next spec that would close the gap, and people send pull requests with those specs or with finished implementations.

The owner-run species pipeline already has this shape. A gap is opened, candidate fixes are written, the fix is its own spec, and the run resumes when it lands (fn-63, the add-species runner). This spec opens the same path to anyone using the studio, so every look the generator cannot draw names the next generator spec.

## Architecture & Data Models
<!-- scope: technical -->

- **Input is the studio's stuck report:** the described trait that was not reached and the rounds that tried. [paraphrase]
- **A gap check comes first.** The trait is read against what the generator declares it can express (fn-84) and against open specs, so a look the dials can reach, or a gap already specced, mints nothing new. [inferred]
- **The gap spec is a flow-next spec in the repository's template,** written by the person's agent: the trait in plain words, the session evidence, what the generator lacks, and acceptance stated on the tree. [paraphrase]
- **A contribution is a pull request** carrying the spec alone or the spec with its implementation, in the repository's PR format. [paraphrase]
- **The owner's review is the authority.** A contributed spec is a proposal; design judgment about the generator stays with the host and the owner, as the repository's escalation rule says. [inferred]

## API Contracts
<!-- scope: technical -->

- **Gap check:** stuck report in; one of reachable (with the dials to try), covered (with the existing spec) or new gap out. [inferred]
- **Gap spec:** a spec file that passes flowctl's validation, carrying the description, the rounds' overlays and seed, and the stills' identities so the owner can reproduce the session. [inferred]
- **Contribution:** a branch and a pull request whose body follows the repository's PR format and links the spec. [paraphrase]

## Edge Cases & Constraints
<!-- scope: technical -->

- **One gap per spec,** as one species per spec: a session stuck on two traits produces two specs. [inferred]
- **A contributor's agent has no authority over the system.** Its spec may be wrong about what the generator lacks; the evidence must let the owner check that in one reproduction. [inferred]
- **Session evidence stays small:** parameters, seed, generator revision and at most four stills, never receipts or frame sequences. [paraphrase]
- **Implementations meet the code rules and the workspace tests** like any change; presets stay value tables with no species branch. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** From a stuck report, the gap check answers reachable, covered or new gap, and only a new gap leads to a spec. Errors: a reachable trait returns the dials to try and mints nothing; a covered trait returns the existing spec id. [inferred]
- **R2:** The person's agent creates a flow-next spec that would close the gap, in the repository's template, with the described trait, the reproducing parameters, seed and generator revision, and acceptance stated on the tree. Errors: a spec that fails validation is reported and not saved; two traits make two specs. [user]
- **R3:** A person can open a pull request with the gap spec alone. Errors: a pull request whose spec fails validation or lacks reproducing evidence is refused by the checks with the reason. [user]
- **R4:** A person can open a pull request with a finished implementation of a gap spec, judged by the same tests, code rules and review as any change. Errors: an implementation with no spec it satisfies is refused. [user]
- **R5:** A contributor guide states the whole path in one place: studio session, stuck report, gap spec, pull request, what the owner reviews. Errors: no error surface beyond the checks in R3 and R4. [inferred]
- **R6:** The owner reproduces a contributed gap from the spec's evidence in one build and sees the described shortfall. Errors: evidence from a different generator revision is flagged as stale, not trusted. [inferred]

## Boundaries
<!-- scope: business -->

- The studio, the agent connection and Jev's proposals belong to the tree studio spec this one depends on. [paraphrase]
- No automatic merge and no automated acceptance of contributed specs or implementations; the owner decides. [inferred]
- No change to the owner-run species gap loop (fn-63) or its routes. [inferred]
- No hosted service for submissions; contributions are pull requests. [paraphrase]

## Decision Context

Split from the tree studio at the owner's choice so the contribution path is buildable and reviewable alone. It mirrors the owner-run gap loop's rule, a gap is its own spec, and widens who may write one; the authority to accept a design stays where it is. [paraphrase]

### One gap-list shape for studio and pipeline (owner, 2026-09-22)

- The gap check here and the gap list a tuning run ends with (fn-89, "A tuning run ends with a gap list") share one entry shape: the trait in the person's words, the rounds tried with overlays and seed, the reviewer's or agent's words per attempt, the stills' identities, and the check's verdict, reachable, covered or new. A studio gap and a pipeline gap then read the same to the owner and to the host that writes the candidate spec. Owner, to the contract: "ok that works". [user and host design]

## Strategy Alignment

- Serves "The catalogue": every gap found names the next generator spec, now from anyone's session. [strategy:The catalogue]

## Parked unknowns

- Whether outside contributors' specs need a lighter template than the repository's own. The first contributed spec answers it.
- Which checks on a contributed pull request can run without the owner's keys or GPU. Reading the CI jobs against a fork answers it.
