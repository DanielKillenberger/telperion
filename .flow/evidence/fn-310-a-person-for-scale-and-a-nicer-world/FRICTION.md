# Friction: fn-310

## 2026-10-23, conductor, run start

- Doing: starting `/flow-next:flow --auto fn-310-a-person-for-scale-and-a-nicer-world` in worktree f4-310.
- Hindered by: the spec has no ready flag, so `pilot snapshot` reports `eligible: false` (route `host`, rule "spec not ready"). `flowctl spec ready` is documented as a person's consent gate, so the conductor cannot set it on an agent's instruction.
- Cost: one round trip to the host before any planning; about 5 minutes and a few thousand tokens.
- Would have removed it: marking the spec ready (`flowctl spec ready fn-310-...`) in the same commit that recorded the owner's pick (201288747), before dispatching the run.
