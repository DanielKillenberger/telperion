# TypeSafe usage guidelines

## Conversation Evidence

> user (turn 1): "we just installed typesafe-ai Using the TypeSafe skill, explore the project and find opportunities for using intelligent judgement to stand in for complex parsing or other fragile code."
> user (turn 1): "From that we want to capture a spec to prep repo for typesafe usage guidelines."
> user (turn 2): "you can also do this: Using the TypeSafe skill, run some experiments using the TypeSafe API key that I've exported to `TYPESAFE_API_KEY`. Propose changes based on the most promising results."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 55% [paraphrase], 25% [inferred] -->

The owner installed TypeSafe on 2026-09-16 and asked for two things: a survey of the places where a typed judgment from Jev could stand in for fragile parsing or hand-written heuristics, and a spec that prepares the repo for usage guidelines so the next agent applies Jev where it earns its place and nowhere else. [paraphrase] The owner's standing rule, recorded in the project instructions the same day, fixes the two poles. Jev never touches generation, because pin tests, byte-identical presets and seeded streams depend on determinism. Jev earns its place screening source literature, where code finds candidate numbers and owns every calculation, and the model judges only which candidates are a measured value at a stated age and under what growing condition. [paraphrase]

A host sweep on 2026-09-16 ranked sixteen sites. The ones that matter fall into four kinds. [paraphrase]
- Literature to preset. Species profiles cite sources by ID with hand-authored confidence, and the growth report composes curves from yield tables transcribed by hand. Nothing checks a cited sentence against the claim it backs. The first misuse found this way is the O1 citation, a site-classification criterion read as a measurement. [paraphrase]
- Owner feedback to work. QA notes of the form "viewer sees ..." are turned into titled findings with severity, an introduced-or-pre-existing classification and a constant confidence, by an agent's narration. No code judges them, and duplicates of a defect the owner already rejected are caught by memory alone. [paraphrase]
- Report assembly. The growth report script lifts a table out of an older report by slicing between two literal substrings, so a sentence edit in the old report breaks the new one. [paraphrase]
- Plain parsing bugs. A correctness test scans JSON by hand, and species QA reads the renderer's last stdout line with a regex. These are code fixes, not judgments, and this spec only records them. [inferred]

Five experiments ran against the live API on real source text and real QA notes, with the key read from the owner's interactive shell and never printed. Every request answered in under one second. The screen classified 12 of 12 sentences by kind, the O1 sentence among them at 0.99 for site criterion. The citation check answered 7 of 9 claims as labelled; the O1 misuse landed at confidence 0.26, which any threshold routes to a person, and one claim that restated the site criterion as a typical rate passed at 1.00, which is why the screen and the check compose rather than either standing alone. Number selection scored 5 of 5 once the candidate regex covered the word "foot", and answered "none" at 0.49 when it did not. Routing of eight owner observations to open specs scored 8 of 8, duplicate detection separated the two true pairs from five negatives, and the severity score matched all four labels. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

- **One caller, one ledger.** Every Jev call goes through one shared caller that owns the endpoint, the bearer key from the interactive shell, exponential backoff on rate-limit and overload responses, and the record of the call. The record, called the ledger entry, holds the state's checksum, the questions, the answers with their probabilities and confidence, the model name and the token usage. A pipeline that consumed a judgment cites its ledger entry, so a reader can see the exact state and the exact answer without re-calling the model. [inferred]
- **Code finds, the model chooses, code copies.** Every numeric value reaches a preset by selection over candidates that code extracted from the fetched source bytes. The model's answer is an option key, and code copies the span verbatim. No model output is ever parsed for a number. A `none` option is always present. [paraphrase]
- **Four question sets, kept as data.** The screen (kind of statement, growing condition, usable as a height-at-age anchor), the citation check (supports, contradicts, says nothing), the pre-parsed selection (candidates plus none) and the triage set (owning spec, same defect, severity) are versioned question definitions the tools load, with their criteria written out in full so the ledger reproduces them. [inferred]
- **Compose, then threshold.** A candidate number passes to a profile author only when the screen calls it a measured size at a stated age and the citation check calls the claim supported, each above the threshold chosen on the labelled set. Anything else is listed for the owner with its probabilities. Confidence summarizes how concentrated a distribution is and never stands in for correctness. [paraphrase]
- **Labelled cases live beside the tools.** The pilot's sentences, claims, selections and triage pairs, with their expected answers, are the regression set. A change to a question set reruns them and reports accuracy and confidence spread before the change lands. [inferred]
- **Judgments never write state.** A triage run produces a proposal the host session reads. It does not create a memory entry, a receipt, a finding or an owner verdict. [inferred]

## API Contracts
<!-- scope: technical -->

The ledger entry is the one shape every tool shares. The fields shown are the contract.

```json
{
  "tool": "screen",
  "state_sha256": "…",
  "source": {"id": "OWIC-OAK", "url": "…", "sha256": "…", "bytes": 42038},
  "model": "jev-latest",
  "questions": {"kind": {"type": "choice", "instructions": "…", "criteria": {"…": "…"}}},
  "answers": {"kind": {"choice": "site_quality_criterion", "probabilities": {"…": 0.99}, "confidence": 0.98}},
  "usage": {"input_tokens": 1206, "output_tokens": 162},
  "elapsed_ms": 560,
  "recorded_at": "2026-09-16T00:00:00Z"
}
```

The screen tool takes a fetched source and a species and returns one row per candidate sentence with the sentence, its kind, its growing condition, the anchor probability and the ledger reference. The citation tool takes a spec's research section and returns one row per claim with its relation, confidence and the source section it was judged against. The triage tool takes one owner observation and the open spec list and returns the owning spec with its distribution, the most similar prior finding with its same-defect probability, and the severity level. Each tool's rows carry the ledger reference of the call that produced them. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **No key, no silent skip.** A tool that cannot read the key stops with a message naming the interactive-shell path. It never returns a fabricated or default judgment, and it never prints, echoes or copies the key. [paraphrase]
- **The build path never reaches the network.** Generation, rendering, the presets, the Wasm bindings and every test under the workspace test commands run with the key unset. A guard fails when code under the core, render or Wasm crates, or under the browser source, imports the caller or names the endpoint. [paraphrase]
- **Candidate coverage is a code bug, not a model miss.** When the model answers `none` with a spread distribution, the tool prints the candidate list beside the source sentence so the reader can see whether the recall regex dropped the span. The pilot's miss on "6-7 foot" is the regression case. [paraphrase]
- **Rate limits and overload back off.** The caller retries 429 and 529 with exponential delay and gives up after a bounded number of attempts, recording the failure in the ledger. [inferred]
- **Source text is untrusted.** The state sent to the model is the fetched bytes with their checksum, never an agent's summary of them, and the model returns typed answers only, so nothing in a source can instruct the tool. [inferred]
- **Cost is measured and reported.** Each run reports its total input and output tokens and wall time. The pilot spent under 60,000 input tokens across 44 requests. No cap on calls is hardcoded. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The repo carries a usage guideline that a new agent can follow without this conversation. It states where Jev may run (evidence tooling, research checks, QA triage, report assembly), where it never runs (generation, rendering, presets, any test on the workspace test commands), that code owns every number and calculation, that every question offers a no-match answer, that candidate coverage is tested before a selection is trusted, that thresholds come from the labelled set, that every call leaves a ledger entry, and that a judgment proposes and never writes project state. [paraphrase] Errors: no error surface beyond the guard in R6.
- **R2:** One shared caller reaches the endpoint with the key read from the interactive shell, retries rate-limit and overload responses with backoff, and writes a ledger entry for every call in the shape shown under API Contracts. [paraphrase] Errors: missing key stops with a message naming the shell path and never prints the key; a failed call after retries records the failure in the ledger and exits non-zero.
- **R3:** A literature screen takes a fetched source with its checksum and a species, extracts every sentence carrying a number with a length, age or rate unit, asks the screen questions per sentence, and prints one row per sentence with kind, condition, anchor probability and ledger reference. A selection tool takes a document and a field question, presents every candidate span code extracted plus none, and prints the chosen span with its distribution and ledger reference; it scores 5 of 5 on the pilot's selections. The screen scores 12 of 12 on kind over the pilot's labelled sentences, and the O1 sentence lands as a site criterion. No number in its output is written by the model. [paraphrase] Errors: a source that yields no candidate sentence prints an empty table with the source's checksum; a selection answering none with a spread distribution prints the candidate list beside the sentence; a sentence longer than the state limit is split at sentence boundaries and each part judged with the whole as context.
- **R4:** A citation check takes a spec's research section, fetches or reads each bullet's source, finds the section that carries the claim's key terms, and judges supports, contradicts or says nothing with confidence. Claims that are contradicted, say nothing, or fall below the threshold chosen on the labelled set are listed for the owner. A numeric claim passes only when R3's screen also calls its sentence a measured size at a stated age. On the pilot's nine claims the O1 misuse is listed for the owner and the six true claims pass. [paraphrase] Errors: an unreachable source lists the claim as unchecked with the fetch error; a claim whose key terms match no section is listed as says nothing at confidence zero.
- **R5:** A triage tool takes one owner observation and returns the owning open spec with its distribution, the most similar prior QA finding with its same-defect probability, and a severity level against the owner's recorded standard. It writes nothing to memory, receipts or verdict slots. On the pilot's cases it routes 8 of 8 observations, ranks the two true duplicate pairs above every negative pair, and matches the four severity labels. [inferred] Errors: an observation that matches no open spec above the threshold returns new spec as the top answer with the distribution shown.
- **R6:** A guard keeps Jev out of generation. A test fails when the core, render or Wasm crates or the browser source import the caller or name the endpoint, and the workspace test commands pass with the key unset. [paraphrase] Errors: the failure names the offending file and the rule.
- **R7:** The pilot's labelled cases, the sweep's sixteen-site inventory with a disposition per site, and the results table are recorded under the spec's evidence directory, and the pilot scripts are not shipped as tools without a rewrite and a test. [paraphrase] Errors: no error surface beyond R3 to R5's case reruns.

## Boundaries
<!-- scope: business -->

- Jev never runs in the generator, the renderer, the presets, the bindings or any test that gates a build. Determinism of pins, presets and seeded streams is not negotiable. [paraphrase]
- Jev judges text and structured state, never a still. The owner's eye stays the visual verdict, and the species QA runner keeps exiting non-zero while a visual inspection is unassessed. [inferred]
- No judgment writes a memory entry, a QA receipt, a finding, an outcome or a verdict slot. A triage output is a proposal for the host session. [inferred]
- The two plain parsing bugs the sweep found, the hand-rolled JSON scan in a foliage test and the stdout regex over the headless renderer, are code fixes for a separate cleanup, not Jev work. [inferred]
- No SDK or language for the tooling is chosen here. The plan decides, within the project's typed-code rule. [inferred]
- The model is not asked to generate text, explain itself, or write a number. [paraphrase]

## Decision Context
<!-- scope: both — conditionally substructured -->

### Motivation
<!-- scope: business -->

The owner asked for judgment to stand in for "complex parsing or other fragile code", and for proposals "based on the most promising results" of live experiments. [user] The most promising results were the literature screen and the citation check, because they close the gap that let a site-classification criterion stand behind the oak's ten-year height, and the triage set, because it turns the owner's own words into a routed, de-duplicated, scored proposal in under a second per observation. [paraphrase] The guideline exists so the next agent inherits the boundary the owner set, and so the ledger makes every judgment as attributable as a preset dial. [paraphrase]

## Strategy Alignment

- Serves "Growth and botanical fidelity": reference-based QA exposes structural mistakes that counts alone cannot catch, and the literature screen makes the references themselves checkable before they shape a preset. [strategy:Growth and botanical fidelity]
- Serves the Attributability metric under the same track: every change to a preset traces to a named dial, and with the ledger every sourced value traces to the sentence and the judgment that admitted it. [strategy:Growth and botanical fidelity]

## Parked unknowns

- Whether TypeSafe exposes a pinned model version, so a ledger entry can name the exact model rather than the alias the response reports. The models endpoint answers this once someone reads it.
