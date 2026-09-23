# fn-119 friction

## 2026-09-23: the design's objective-to-track routing does not exist in code

Doing: investigating before any edit, to route each objective to the track whose dial groups answer it (design: "Each objective is routed to the dials that answer it (the existing routing)").

What hindered: no such routing exists. `Run::route_remaining` (`crates/telperion-jev/src/tuning/routing.rs`) sends each approved priority to one of `tuning`, `new_capability`, `appearance`, `insufficient_evidence` or `existing:<spec>` (`judgments::route_question`); the route note is `<gap>=tuning`, with no dial or group. Proposals (`live.rs` `propose`) are asked per dial against the whole state and carry no priority (`engine::Proposal` has `dial`, `action`, `ledger`, `direction_mass`, `rule`). `bundle::track::assign` splits moves by dial group; nothing ties a priority to a track. The palm's run-3 `result.json` confirms it: the latest routes read `tuning` or `insufficient_evidence`, never a track. So "a track's objectives are those routed to its dial groups" has nothing to read, and R2 ("the materials track, given `trunk-colour-and-weathering`") and R3 cannot be built without choosing how an objective reaches a track: a new route criterion per track (a changed Jev question), a per-priority tag on the proposal question, the track's view against the objective's views, or a track config listing. That choice is system design, which escalates to the host.

Smaller points the host should settle with it, both found in the same reading:
- Caps of 8 priorities: `priority::Approval::verify` refuses `ordered.len() > 8`, and `sheet::Request` and `progress::Request` refuse more than 8. The palm's objective list under the design is 3 owner priorities plus 11 expressible `core`/`secondary` traits (6 core less the unexpressed `fruit-clusters-pendent`, 6 secondary) = 14.
- An inventory trait is not yet a `priority::Gap`: a gap needs `views` inside the required cells and `evidence_ids` citing both a render and a reference from the checkpoint's evidence. A trait carries `reference_ids` only; deriving its views (the cited references' views, say) and render citations is a choice.

Cost: about 20 minutes of reading, no code written, no runs, no paid calls.

What would have removed it: the architecture section marking "the existing routing" as checked against `routing.rs` and `bundle/track.rs` before the spec was marked ready (CLAUDE.md, "Gates and checked claims").
