# fn-63 friction

## 2026-09-18 — an interrupted worker left a crate that does not compile

Resuming fn-63 after the first worker was killed by a model rate limit. It
left about 1,650 lines of uncommitted, untracked gap-loop code whose `mod.rs`
declared two modules (`metrics`, `rounds`) that did not exist, so the crate
would not build and nothing in the inherited work had ever been run. Cost:
about 25 minutes of reading before a single `cargo build` could tell me
whether any of it was right, and no way to separate "this is broken" from
"this is unfinished" except by reading all six files.

What would have removed it: a worker that commits a compiling checkpoint
before it starts the next module, even a WIP one on its own branch. A
`mod.rs` declaring a module that is not there is the cheapest possible
tripwire and it fired only after a human resumed the task. The lifecycle
already commits a receipt at the end; a mid-task checkpoint commit on the
task branch would make an interrupted run resumable by `cargo test` instead
of by re-reading.

## 2026-09-18 — R7 needs a real species run and cannot be done inside this task

R7 asks for one real species end to end with at least one real gap: the gap
routed by the table, its fix landed as a reviewed spec, the run resumed, the
owner's checklist reached. That is a species onboarding (its own spec under
the one-species-per-spec rule), a generator spec worked and reviewed, and an
owner verdict on stills — none of which fits inside the spec that builds the
machinery, and the last of which is the owner's by rule. Cost: none spent, it
was recognised before any capture. Reported rather than attempted.

What would have removed it: a spec whose acceptance separates the loop's
machinery from its first live exercise, so the live run is a dependent spec
with its own species and its own budget rather than a criterion the
machinery's own task cannot close.

## 2026-09-18 — the workspace suite outruns the tool's timeout

`cargo test --profile ci --workspace` is the full gate, and its geometry tests
(`fixed_oaks_…`, `fixed_spruces_…`) run past 60 seconds each with the whole run
well past the 600 s an agent's shell call may block for. It has to be started
in the background and polled, which means a worker cannot simply gate on it:
it either waits without a deadline or reasons about which crates its diff
could reach. Cost here: about 25 minutes of wall clock still running at the
end of the task, and a gate argued rather than observed for the crates the
diff does not touch.

What would have removed it: a per-crate gate the worker can run in the
foreground (`-p <crate>` already works and the jev crate's whole suite is
under 10 seconds), named in the spec's Quick commands beside the full one, so
a diff confined to one crate gates on that crate in the foreground and the
workspace run is CI's job. The repository already has the shape for it; the
convention is what is missing.
