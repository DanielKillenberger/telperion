# The Jev caller carries a still, so a question can be asked of a picture

## Conversation Evidence

> owner, 11 Nov 2026: "use jev for specific questions about faults/features of images"
> docs/typesafe.md, owner, 1 Nov 2026: "Jev may read a still"
> AGENTS.md, owner, 16 Sep 2026: "Every call goes through the shared caller and leaves a ledger entry."

## Goal & Context

Child of goal fn-347, serving its R4 and its rigor line on Jev. The goal's host must put narrow questions about a still to Jev. The shared caller (`crates/telperion-jev/src/caller.rs`) sends `model`, `state` and `questions` only, and the TypeSafe API takes images as a separate `assets` map (`type: image`, `format`, `data_base64`), so today no question can be asked of a picture through the caller. [paraphrase]

## Acceptance Criteria

- **R1:** `jev ask` takes an optional `--assets <file>`: a JSON map from an asset's name to a local image path. The caller sends each as the API's image asset, by the contract in the live docs (`https://docs.typesafe.ai/api/system-one.md`, read before writing; how state names an asset, the formats and the size limits are taken from that page and quoted in the result). [inferred]
- **R2:** the ledger entry for such a call records each asset's name, path and SHA-256, never its bytes. The key is never read, echoed or copied. [paraphrase]
- **R3:** tests on the mock transport hold that the request carries the assets, that the ledger holds the digests, that an image over the documented limit is refused by name before any call, and that a call without `--assets` sends the bytes it sends today. [inferred]
- **R4:** one live call through `bash -ic`, on one still from `.flow/evidence/fn-346-the-carbon-balance-charges-living-wood/`, with one Choice question that offers a no-match answer; its answer, probabilities and ledger reference are recorded in this spec's evidence. [inferred]
- **R5:** `docs/typesafe.md` gains a short section: how to ask a question of a still, with the command, and what such an answer may and may not be used for. [inferred]
- **R6:** the gate is green and the `codex` review of the diff ends SHIP. [paraphrase]

## Boundaries

- Jev stays out of generation, rendering, presets, the Wasm bindings, the browser source and every workspace test; the isolation guard still passes. [paraphrase]
- Jev selects and never supplies a number; every question offers a no-match answer; an answer is evidence and never a verdict. [paraphrase]
- Nothing but the caller, the `ask` command, their tests and the doc is changed. Nothing merges to master and nothing is pushed. [paraphrase]

## Decision Context

Settled by the owner: Jev is used for specific questions about faults and features of images. The host's choice: the smallest change is an optional asset map on the existing `ask` command, so every image call still passes the one caller and its ledger. [inferred]

## Open Questions

None.
