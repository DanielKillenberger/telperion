# The literature step retires stale decisions and cites the sentence an appearance came from

## Conversation Evidence

> fn-80 live run, 2026-09-23, the palm's second pass on fn-127: quality passed crown width, frond length, leaflet length and width, yet select marked crown width and the leaflet sizes "below the data-quality bar"; seven old `requirements-unmet` decisions stayed open; `verify` filed four `obligation-unmet` and two `claim-unsupported` on appearance values whose cited sentence was page navigation.
> user (2026-09-23): "yes pls fix the frictions if they're obvious."

## Goal & Context
<!-- scope: business -->

Two defects keep the palm's literature step from finishing on data it now reads correctly, and one makes its appearance evidence unverifiable. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **What exists, checked 2026-09-23 on the fn-80 branch.**
  - `decision::retire_unfiled` (`pipeline/decision.rs:181`) supersedes a stage's open decision that a rerun with changed inputs did not file again; only `gate` calls it (`stages/gate.rs:144`). `quality` and `select`, which file `requirements-unmet` since fn-118, never retire theirs. [checked]
  - `select` skips a field that has an open decision (`blocked.contains`) or failed quality (`stages/select.rs`, "below the data-quality bar"); the stale decisions blocked crown width and both leaflet sizes although `quality.json` passed them. [checked]
  - `select.json` appearance: `bark_roughness` level `rough` from A1 with a cited sentence of navigation text ("e Landscapes Tour](https://arboretum…) **Bo"); `bark_colour` `unstated` while A1's extract holds "The trunk has a characteristic diamond pattern and is rough gray". `verify` applies `measurement_not_invention` to appearance pointers (`stages/verify.rs:117-126`). [checked]
- **Shape.** [host design]
  - `quality` and `select` call `retire_unfiled` for their stage after filing, as `gate` does.
  - An appearance trait is judged over the extracted candidate sentences of its admitted sources, one sentence per judgment as a field is, and the value records the sentence the chosen level came from, not a page chunk.
  - `verify` checks an appearance value with a support question (does the cited sentence describe this level) in place of `measurement_not_invention`, which stays for measured fields.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** A red test from the live run: a `requirements-unmet` decision whose field now passes is superseded by the quality rerun, and select fills the field. [inferred]
- **R2:** A red test on A1's sentences: `bark_colour` is judged from "rough gray" and the value cites that sentence. [inferred]
- **R3:** `verify` holds on an appearance value whose cited sentence supports it and files a claim decision when it does not; no `measurement_not_invention` is asked of an appearance value. The support question has labelled cases and a no-match answer. [inferred]
- **R4:** The gate is green: `cargo test --profile ci --workspace --no-fail-fast`. [paraphrase]
- **R5:** `verify` retires its own open decisions that a rerun with changed inputs did not file again (`retire_unfiled` for stage "verify"), as `quality` and `select` do; a red test from the live run: the four appearance `obligation-unmet` decisions and `claim-unsupported/A1`, `/F1` stop blocking `generate` after a rerun. [host design]

## Boundaries

- No live run here; the host reruns the palm's stages. [inferred]

## Decision Context

- Owner's standing rule of 2026-09-23: obvious friction fixes are the host's to spec. [user]

## Open Questions

- None.

## Settled

Closed as landed (2026-09-26, fn-149 R8): carried by the fn-149 runner rewrite, merged in #121 (c2ac430b).
