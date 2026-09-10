---
satisfies: [R15]
---
# fn-14-bark-and-foliage-appearance.5 4x multisampling with a resolve, on the headless target and the page

## Description
Render colour and depth at 4x with a resolve into the single-sample target on both the headless target and the page, with a capability probe and a recorded fallback, and record the sample count in the timing record (R15).

**Size:** S
**Files:** `crates/telperion-render/src/lib.rs`, `crates/telperion-render/src/headless.rs`, `crates/telperion-render/src/web.rs`, `crates/telperion-render/src/timing/report.rs`, `crates/telperion-render/tests/conformance.rs`
**Touches:** [crates/telperion-render/src/lib.rs, crates/telperion-render/src/headless.rs, crates/telperion-render/src/web.rs, crates/telperion-render/src/timing/**, crates/telperion-render/tests/**]

### Approach
- The pipeline helper at `lib.rs:320-366` takes a sample count; the room and vegetation passes render into a 4x colour and depth attachment and resolve into the existing single-sample colour target; the shadow pass and the selection compute are untouched at one sample.
- The headless surface (`headless.rs:54`) and the page surface (`web.rs:157`) create the multisampled attachments at the same count.
- Probe the adapter for 4x support on the colour and depth formats; fall back to one sample and write `samples` into the timing record and the still's log line.
- Cost: one oak timing run at 4x and one at 1x, both recorded, the difference is the resolve cost the report cites.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/lib.rs:222-366` — passes and the pipeline helper
- `crates/telperion-render/src/headless.rs:40-120` and `web.rs:140-180` — surface creation and readback

**Optional** (reference as needed):
- `crates/telperion-render/src/timing/report.rs:40-70` — the record fields

### Key context
- The readback in headless.rs copies the single-sample resolved target; the multisampled texture is never read back.
- One timing re-run per configuration, per the budget rules.

## Acceptance
- [ ] Colour and depth render at 4x and resolve on both the headless target and the page; the shadow pass and selection stay single-sample
- [ ] A device without 4x support falls back to one sample; the timing record and the still's log line carry the sample count
- [ ] Oak timing at 4x and 1x recorded; the difference stated
- [ ] `cargo test --release --workspace`, fmt, clippy and `npm run test:render` pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
