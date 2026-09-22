# date-palm pipeline report

Status: halted.

## Sources

| Source | Final URL | Raw | Markdown |
| --- | --- | --- | --- ||
| A1 | https://apps.cals.arizona.edu/arboretum/taxon.aspx?id=207 | 5e57ffb659123b552f916f41e64dee6229524d5a39c65ff2b0759dd421246618 | 6ff9467522b16d062f323684f8b63be43806d42c66f04618f4b7f2f12f8ee854 |
| F1 | https://ask.ifas.ufl.edu/publication/FR314 | 3d87cbe22d734b96219348c8114ca3ad71f9e5a732d2211dc8c7d1af8e088698 | a78f7b1699627498b9c03448b0fcd8b490a0406c484dbf22222a1fcc9377ff50 |
| M1 | https://link.springer.com/article/10.1186/s12870-022-03841-0 | 364571b4cbb92b16aa546ab5ea882435afaf359e2e671d407d868242166b9709 | 2f79c74190191dca91f07b704c6fd71bb678bb4ed3adf70f1aecda68d26445e6 |

## Fields

| Field | Level | Bar | Value |
| --- | --- | --- | --- ||
| dbh_m | proxy_only | partial | below the data-quality bar |
| height_m | proxy_only | proxy_only | [15.24,30.48] m |

## Curves

Rate , shape , mature at  years.

None.

## Decisions

| Decision | Kind | Status |
| --- | --- | --- ||
| date-palm/discover/manifest-proposed | manifest-proposed | resolved |
| date-palm/document/article-unfilled | article-unfilled | resolved |
| date-palm/gap/gap-fix/gate-onboarding-gate-capability | gap-fix | resolved |
| date-palm/gate/onboarding-gate/capability | onboarding-gate | resolved |
| date-palm/gate/onboarding-gate/registry | onboarding-gate | resolved |
| date-palm/gate/onboarding-gate/seeds | onboarding-gate | resolved |
| date-palm/generate/visual-unassessed | visual-unassessed | open |
| date-palm/quality/data-insufficient/dbh_m | data-insufficient | resolved |
| date-palm/verify/claim-unsupported/A1 | claim-unsupported | resolved |

## Stills

| Still | Path or error |
| --- | --- ||
| preset | .flow/evidence/date-palm/pipeline/stills/preset.png |

## The run's numbers

Missing: metrics.json; run `species-pipeline gap metrics`.

## Cost

| Stage | Runs | Firecrawl credits | Counted | Jev calls |
| --- | --- | --- | --- | --- ||
| discover | 2 | 12 | estimated: one credit per call the adapter did not price | 4 |
| document | 3 | 0 |  | 0 |
| extract | 2 | 0 |  | 0 |
| fetch | 2 | 6 | estimated: one credit per call the adapter did not price | 0 |
| fit | 1 | 0 |  | 0 |
| gate | 2 | 0 |  | 0 |
| generate | 3 | 0 |  | 0 |
| quality | 2 | 0 |  | 4 |
| screen | 2 | 0 |  | 66 |
| select | 2 | 0 |  | 2 |
| verify | 2 | 0 |  | 4 |
| total | 23 | 18 | estimated: one credit per call the adapter did not price | 80 |
