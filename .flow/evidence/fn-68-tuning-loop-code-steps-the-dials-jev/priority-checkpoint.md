# Human gap-priority checkpoint

The owner reviews the proposed three gaps and linked reference/render images, then says **confirm**, gives a new order, or adds a missed gap. The host records that conversational decision. The owner does not need to author JSON, choose dial values, or manage implementation.

After initial assessment, `tuning-loop run` writes `priority-review.json` beside `run.json` and pauses—even if the model passed every cell. The packet retains the full visual review, all findings and image hashes/paths. The proposed three are the first eligible findings in reviewer source order, explicitly **not** a newly calibrated ranking. Fewer than three are shown when fewer supported entries exist. Unranked findings remain visible.

## Host transport example

Read the actual pause and packet. Copy selected gap objects unchanged into `ordered`, in the owner's order. Add a missed gap with a new `owner-` ID, the owner's observation, existing evidence IDs and relevant configured view names. Fixed/fresh seeds for those views remain mandatory; a leafy gap is not also required on a bark-only view.

```json
{
  "pause_id": "<run.pause.id>",
  "identity": "<run.identity>",
  "action": "approve gap priorities",
  "by": "<actual owner, relayed by host if applicable>",
  "rationale": "<actual confirmation/reordering/addition>",
  "preserve_evidence": true,
  "priority_approval": {
    "checkpoint_sha256": "<priority-review.checkpoint_sha256>",
    "scope_sha256": "<priority-review.checkpoint.scope_sha256>",
    "ordered": [
      {
        "id": "owner-missed-gap",
        "observation": "<owner's added objective, not a proposed mechanism>",
        "evidence_ids": ["render-0", "reference-0"],
        "views": ["<relevant configured view>"]
      }
    ]
  }
}
```

The host retains any still-required experimental-pilot authority from the existing explicitly authorized scope in the same decision; this checkpoint supplies no new authority, cap increase or efficacy qualification. Submit the recorded decision using the existing command:

```sh
cargo run -p telperion-jev --bin tuning-loop -- run \
  --config CONFIG.json --out RUN_DIRECTORY --resume OWNER_DECISION.json
```

Missing, stale or invalid approval is rejected before paid work. Approval binds species, owner objectives, checklist, required views/seeds, references and finish-anchor identity. Routine candidate changes/rebuilds do not change these goals. Objective/reference/finish changes require another checkpoint; all past packets and owner decisions remain in history. Existing artifact freshness and budget checks still apply.

Approved goals become explicit additional visual cells, not just prompt prose. Added goals require reassessment using valid existing images; missing views are captured only through the ordinary budgeted path. The approved full-view set and extra serialized requirements are included in image/token reservation. Unknown or failed coverage cannot become readiness. Original model findings are never rewritten and approval does not mark a gap resolved. Final species acceptance remains separate.

## This checkpoint's evidence

Frozen offline packet from the last valid reframed result: `priority-review.json`. Browse `visual-review.html#priority-review`. No owner ranking has been applied. Confirming this packet does not resume `.flow/tmp/fn68-pilot-run`.

| Identity | SHA-256 |
| --- | --- |
| checkpoint | `dcc6a63fbfa782fb4970fb5a9fbf0a1228c39a65b6ec3c1a9736e7c6e02a8010` |
| scope | `903276d3348f3e5f40aa2c8ec25659c7d16f3880e61c559d2fabef33c985bed3` |
| candidate | `c1781092148dc4c69cbf4b60c37be26a59dcd59f6e43d014f409f0c29c269d57` |
| bound result | `bdb13daa0bb9fb9e0bc8cc6d0d73c1fd35e2188a39de7943c85c47608971998f` |
| raw receipt | `97ae88cb7c716ed2c99df28f5e651004679b3f14035b84342b6568e39547656f` |
| inventory | `f98f4325752a95103c510cc3f1871af2fcd12a2053a9359e6f770b128853c20f` |
| original runtime | `d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e` |

Proposed source order, not a new ranking: finding-0 foliage organization, finding-1 bark required-unknown, finding-2 hanging reach and regular base. Preserved: finding-4 joint coverage constraint and the supported finish finding. Owner crown/hanging/material notes remain sourced history, not approval of this packet. Blind calibration fixtures and receipts are unchanged. R7 magnitude efficacy and R8 empirical visual adequacy remain open. Cumulative paid usage remains 552,431 / 570,000. Visual 20 / 20 exhausted. See `r7-r8-proof-proposal.md`.
