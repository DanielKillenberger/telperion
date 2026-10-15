# fn-275-a-young-palms-fronds-grow-to-their-size.3 A frond draws its rachis

## Description
R2c.

## Acceptance
R2c.

## Done summary
R2c: the drawn rachis is deferred to fn-283 (host ruling iii: as wood it needed the device's frond code moved into the boots stage, about 1,200 lines). Instead a young frond's segments crowd toward its base: segment k stands at f times its station (`Split::segment`, `pack.wgsl` `leaflet`), so its rachis is short and lengthens as it matures. A leaflet with no spine drawn stands within 0.75 widths of another segment; reach is 0.79 to 1.21 blades, with no jump from one segment to two. The gap beside a basal spine and between spines (up to 3.2 widths; 1.39 on the mature palm) was accepted by the host, to be closed by fn-283. CPU and device agree at 3 and 4. The palm at 40 is byte-identical. Stills at 2, 3, 4 and 40 were viewed. The gate passed 1,407, failed 0, ignored 101.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 49f0a225b, 4a4b69544
- Tests: cargo test --profile ci --workspace --no-fail-fast
- PRs: