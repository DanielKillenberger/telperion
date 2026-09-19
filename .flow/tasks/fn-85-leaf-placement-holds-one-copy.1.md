---
satisfies: [R1, R2, R3]
---
# fn-85-leaf-placement-holds-one-copy.1 Implement Leaf placement holds one copy

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
The crown is one copy while it is filtered, whatever the cull drops.

`foliage::cull` takes its `Instances` by value and keeps the survivors with `Vec::retain` in the buffer it was handed. The `try_reserve` at the input's own length is gone, so the second allocation no longer exists at any point in the call. Capacity is not shrunk, so the vector keeps the block the caller owned.

`retain`'s closure cannot refuse, so an overflowing or non-finite transform sets a flag, the pass runs to its end, and the error is raised after it. The vector is discarded unread on that path, which is why the leaves already removed behind the failure point do not matter. Every other behaviour is unchanged: a leaf still survives when any of its vertices lies inside the shell or the crown's underside, in the order it was placed.

`cull_retains_in_the_buffer_it_was_given` pins the pointer and the capacity across a cull that drops one of two leaves and a cull that drops every leaf, and checks that the right leaf survived rather than only that one did. It was confirmed red against a restored second-buffer body and green with the fix.

Measured on the `ci` profile, peak RSS polled from `/proc/<pid>/status` VmHWM with one test per process, the same method both ends:

```
fixed_spruces   4,994 MB -> 3,244 MB   -35%
fixed_beeches   4,786 MB -> 3,945 MB   -18%
fixed_oaks      1,672 MB -> 1,469 MB   -12%
fixed_birches     820 MB ->   778 MB    -5%
```

R3 asked for `fixed_spruces` under 3,500 MB. The spruce saving lands where the arithmetic predicted for removing one copy of 470 MB across four in-flight seeds.

Two findings were escalated to the host rather than resolved here, and both went into fn-86: `shadow.wgsl:21` binds the same placement buffer as `foliage.wgsl` and `select.wgsl`, so the encoding spec names three shaders and not two; and `timeline::Placement` caches leaves per shoot while the tree's AABB grows, so fn-86 quantizes against a parameter-derived box that cannot drift instead of a tree-derived one.

The implementer was the session model in-host. The dispatch named `gpt-6-astra at high`; the owner has no Astra quota, the conductor changed the routing mid-task, and no `codex` bridge was started at any point.
## Evidence
- Commits: c9aed1c0, 66e5229d
- Tests: cargo test --profile ci -p telperion-core --test foliage, cargo test --release --workspace --no-fail-fast -- --test-threads=1, cargo fmt --all --check, cargo clippy --profile ci --workspace --all-targets
- PRs: