# Friction, fn-207

## 2026-10-05: the gate ran before the review

**What:** I ran the workspace gate (509 s) and then Codex review 1 found two P1s. Their fix changes `lay.rs` after the gate, so the gate has to run again.

**Cost:** about 9 minutes of a repeated gate.

**What would remove it:** run the Codex review before the gate on a task whose closing needs both. The rule "gate once, at the end" reads as before review.
