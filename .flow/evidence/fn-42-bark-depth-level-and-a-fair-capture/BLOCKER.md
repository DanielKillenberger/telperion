# fn-42 checkpoint and blocker

NEEDS_HUMAN: Restore the original owner-white-oak-bark.png and owner-norway-spruce-bark.png photographs (or identify their actual local paths). The fn-32 catalogue has no source URL for them, and repository plus sibling-worktree caches contain only historic measurements and rendered stills. Reference comparison and colour calibration cannot be accepted against absent source pixels. Reference crop scale must be documented as measured or explicitly estimated when the sources are restored.

Prepared a test-only production-renderer fixture that captures a flat 0.4 m square of oak and spruce bark with an explicit material radius. The host inspected two initial backlit images and the two corrected images, the full four-image budget. Final lighting is azimuth 45 degrees, elevation 55 degrees. Both final images are usable baseline captures, not a fidelity improvement or owner acceptance.

Verified one physical-coordinate test and one explicit hardware capture, including deterministic redraw. Formatting and diff checks passed after module ordering was formatted. No production shader, material, species-pipeline or leaf-layout change. Timing is not claimed under GPU contention. R1 is partial; R2-R5 remain incomplete. Task must not be marked done.

Friction reviewed by host: stale fn-42 design reconciled with accepted fn-71 filtering; missing local source cache needs restoration; flat-patch radius requirement resolved by the internal fixture; cold compilation cost at least 30 seconds and can be reduced by retaining the warmed build; wrong initial light orientation resolved and recorded in the capture recipe. No new friction specs created. The source cache and build-cache issues are local setup, not repository work.

Route taken: fn-42 -> reconcile newer owner constraints -> direct work -> missing reference photographs.
stage: work - ran (partial; calibrated capture checkpoint)
stage: impl-review - skipped(config: review.backend=none; host inspected the fixture diff)
stage: completion-review - skipped(policy: task incomplete)
stage: qa - skipped(policy: reference comparison and implementation incomplete)
stage: make-pr - skipped(policy: task blocked before implementation acceptance)
Tracker sync: n/a (bridge inactive)
Next: restore original reference photographs, confirm their crop scale basis, and resume this task.
