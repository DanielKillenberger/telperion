---
satisfies: [R15]
---
# fn-14-bark-and-foliage-appearance.5 4x multisampling with a resolve, on the headless target and the page

## Description
Render colour and depth with 4x multisampling on the headless target and the page.

## Acceptance
- 4x MSAA with resolve; 1-sample fallback recorded; cost inside the R12 bound.

## Done summary
Colour and depth render at four samples and resolve into the single-sample target headless and on the page; shadow map and selection stay at one sample; the adapter's support decides the fallback.

## Evidence
- Commits: 4fc5027666fc153f96f34dc4c104623129bc2dbe
- Tests: cargo test --release --workspace; npm run render:build && npm run test:render
