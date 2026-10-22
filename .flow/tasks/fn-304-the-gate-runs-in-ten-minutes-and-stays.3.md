---
satisfies: [R6]
---
# fn-304-the-gate-runs-in-ten-minutes-and-stays.3 Walks merged, wasm flower check folded, device field at one grid

## Description
Rulings 6, 3 and 4: the walks binary (369 test-seconds, its two big tests each build the walks under nextest), the wasm flower binding (128 s, three builds per preset) and the device field (92 s).

**Size:** M
**Files:** `crates/telperion-space/tests/walks.rs`, `crates/telperion-wasm/src/generate/tests.rs`, `crates/telperion-render/src/generation/resident/field_tests.rs`, RESULT.md
**Touches:** [crates/telperion-space/tests/walks.rs, crates/telperion-wasm/src/generate/tests.rs, crates/telperion-render/src/generation/resident/field_tests.rs, .flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/**]

### Approach
- walks (`walks.rs:19-33`, `:143-181`): one test calls `every_walk()` once and runs `judge` then the vanishing-size body; every assertion kept; no walk narrowed. Check `docs/tree-space.md:52` still reads right. Prove each half red on fn-271's patches (insertion switch, `lineage.rs`; presence window at full size, `contract/presence.rs`).
- wasm (`generate/tests.rs:125-147`, `:168-209`): the main-entry test, which already generates every preset with surface and foliage, asserts slots 27-29 empty and `organReference` null for non-flowering presets; the flower test builds and checks only the flowering presets (telperion, laurelin) and keeps its `flowering == [...]` claim from the main-entry pass or its own. Prove the folded assertion red (a temporary organ slot written for a non-flowering preset).
- Device field (`field_tests.rs:191-207`): the gate runs one grid size (36, unless measurement favours another) at seeds 1 and 5678; an ignored test runs `[24, 36, 48]`. Its reason must not start with `device limits:` or `release check:` (`docs/gpu-tests.md:13-14`). Red proof: a temporary per-cell offset in the device read-out at all grids. fn-302 and fn-303 edit `generation/` in parallel: touch only this test.

### Investigation targets
**Required:**
- `crates/telperion-space/tests/walks.rs`
- `crates/telperion-space/tests/walk/mod.rs:22-59`
- `crates/telperion-wasm/src/generate/tests.rs:120-210`
- `crates/telperion-render/src/generation/resident/field_tests.rs:130-210`
- `docs/gpu-tests.md:1-40`
## Acceptance
- [ ] walks: one merged test growing the walks once, both halves red on their patches.
- [ ] wasm: no non-flowering preset is built twice across the two tests; the flowering presets keep their own extra builds (pipeline reference, foliage and surface-only requests); folded assertion red on its patch.
- [ ] Device field: one grid at both seeds in the gate, all grids ignored, red proof recorded.
- [ ] Gate run once at the end; codex impl review.
## Done summary
Rulings 6, 3 and 4. The two walks tests over every walk are one test that makes the walks once (306 s at rayon 4), both assertions red on their own patches (insertion switch; presence window x 1e-6 with the by-degree assertion skipped). The wasm main-entry test holds non-flowering presets to empty organ slots and the flowering list; the flower test builds Telperion and Laurelin only; the one case no longer caught is named in RESULT.md. The device field runs grid 36 at both seeds; every grid is ignored. F4 merged at 26a3b19ab (fn-303). Gate green: 1,473 passed, 0 failed, 123 ignored, 1,971 s.
stage: impl-review - ran (codex, three lenses, SHIP; one evidence claim corrected) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 110c29289d7ffa3b614b2b6507d835fbf87ec879, 652297d193676875ddeaddce171bc1a492229490, 68c947f4177944f693de970ea297d429785f943d, b20deba80baa087e375fad16944da2dae8d2a492, e98843a5c9e7fd28f6a8a5b06f179f8e49a19d04, 75b52fe6d10913a6b6df97e49635b2b5b2f66eaf, e8d33b3ce037ba8a131dec7b6ad97d116c5b3b9e, 642d03478ed5de50df21606c2f1aec871513cbb4, 20fcedce1f9a3c3abdb470e7d7f04e3bc8aeb61f, 1833ccf15c8f37b4ac2ee25cfb5e8dc0dd1292e8, 6198fbc9a189494b0f40e7f88e956efe88aec814, 89ceb5ebc436c396b9d23d30f46be659fb50553c, b2c433cafa12b61de748b3342a15dfa053351bfc, f85378805bb1736a710df698b4c29184c8960ba0, 3debe60d3495c4282a4f57aec57e23d5e038964d, 96a67939f37e35177b9ac20bafffab62f901a079, 2416619ebf500c9256298ba3252966ea021cdee6, a497031e18a444b60812d8e706c35eee0f4215f6, 0d085779c15a866cc36f922f982cc85d2aec8e76, 26a3b19ab4fac4099946fe8b80afd05f28c98c65, f3f109409387160dcd21d3214be0fdf8d08b9b8d, 16e592331339f7993376146bce4c8775292c21af, 7f3d88403372c3d56c04dd728760d652ffa4dd69, 3121a4aa9e544616781f9c54931085be0a352c3d
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1473 passed, 0 failed, 123 ignored), proofs.sh t3-* (nextest, telperion-heavy 8 --threads 4)
- PRs: