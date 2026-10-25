# A palm's size is its growth rate and mature range; the literature rerun's last defects

## Goal

The requirements table asked every growth form for height and trunk diameter at 20, 50 and 80 years, but palm literature gives growth rates, a few age classes and mature ranges, and a palm grows near-linearly. On the palm's rerun after fn-129-fn-131 (2026-09-23), sufficient leaflet sizes still failed on `no_mature_size`, height and DBH had one age point each, and the conductor searched for sources before rerunning stages whose code had changed. Owner (2026-09-23), asked what a palm row should require: "Growth rate + mature range (Recommended)".

## Decisions

- Host design: a new `rate` field kind (e.g. `height_growth_m_per_year`) with its own question set; the palm row asks the rate plus mature `height_m` and `dbh_m`; other growth forms keep 20/50/80.
- The gap follows the level: a `partial`/`sufficient` mature field never carries `no_mature_size`.
- Stale stages first: the conductor reruns stages whose artifact records another build before searching or pausing (the fingerprint lacked the build id).
- Located (2026-09-24): `stages/quality.rs` `run`; `conductor/plan.rs` `next`.

## Acceptance Criteria

- **R1:** The palm's leaflet rows (`sufficient`, 7 points) pass with no `no_mature_size` — met.
- **R2:** Palm row requires a height growth rate and mature height/DBH; A1/P5 rates, P8 range and A1's trunk fill them — met (trunk filled from "18 inches", 0.4572 m; the grammar reads no "½").
- **R3:** Rate question set with labelled cases and no-match — met.
- **R4:** With stale stages the conductor reruns the stages first — met.
- **R5:** Gate green — met (1023 passed).

## Result

Landed in `068ba162`; carried by the fn-149 rewrite (#121). The palm's `dbh_m` bar set to `proxy_only` (A1's "up to ½ m" labels as a bound).

## Open

- Vulgar fractions in `quantity::unit_re`.
