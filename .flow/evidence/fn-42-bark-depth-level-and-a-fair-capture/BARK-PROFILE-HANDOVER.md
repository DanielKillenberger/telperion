Implemented the dispatched rough profile candidate in plates.wgsl and presets/materials.rs. Oak: furrow strength .55, cell scale .055, dome .18, edge lift .35, floor row 0; elongation 1.8 unchanged. Spruce: cell scale .021, dome .08, edge lift .30, floor row 0 unchanged, directional occlusion .45; elongation .2 unchanged. All other rows, including colour and grain, are unchanged.

Rough constants are wall .06, depth .03 and floor .006; smooth selects retain .14, .045 and .012. Rough-only boundary noise uses the requested two projections, frequency 8, matching footprint, phase (17.3,29.1), and host-confirmed signed-mean offset .04*(a+b-1). No new fade or footprint switch. Smooth arithmetic source-normalization check passes.

PROVISIONAL: original plate mean and furrow mean constants remain unchanged as directed. Rough production near/far mean measurement and any correction are host-owned and pending. This is not a passing gate or acceptance claim.

baseline: none (no Quick commands defined). GPU/Cargo checks deferred to host to avoid contention. git diff --check and static source/preset preservation checks pass. Changes remain uncommitted per shared-checkout dispatch; host owns test edits, gates, commit, review and completion.

Tier: session via flowctl judge.
stage: impl-review - skipped(config: REVIEW_MODE=none; host owns completion)
