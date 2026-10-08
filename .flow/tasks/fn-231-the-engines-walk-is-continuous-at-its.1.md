---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-231-the-engines-walk-is-continuous-at-its.1 Measure, fade in and prove the walk's ends

## Description
TBD

## Acceptance
- [ ] TBD

## Done summary
The engine's walk is continuous at both ends and inside. Every fractional element is now a making draw that grows in from its lead and that the predictor counts at its share: the last bud place of a fractional count, the share of present nodes a lifespan-cut unit lays (the age's last unit, cut from its room, and the rest of the cycle a continuation or an ended age's relay grows), and a fading subtree in the kept prediction. Both executors do this in integers, and their parity tests include fractional buds, fractional lifespans and sub-tick lifespans. R3's convergence test replaces the assertion that the raw jump remained. fn-230's walk frames at 40 and 80 show no pop. The beech, spruce, oak and palm hash identically; the ordinary tree keeps its rows (host). R5's cost was measured and reported, and its fix is deferred to its own spec (host).

stage: impl-review - ran (codex: fan-out NEEDS_WORK, five findings, three fixed and two settled by the host; after the reset, a fresh fan-out with one finding declined as out of the valid input domain, then SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 911bf5076ca971d418e49ef9362b68f6785ad549, 327f6d6cc9c5e585a0df8924d8cd9085978c12a8, 069378a4738f66600e13b155fe11dc0baae6993d, 8b116ed199c7758f78e02bc3bf04f8136565425a, cee0782cf60dc17fbe67105271d7a22061363c62, 5482d7cadd98706c8f86c803918a0c9708f1dcf2, 0d76dd919acfbafe9f551a5b2254469aaa588295, 1ea8212f5ca7c9cba8cbefbea0d6cdc8ede746b5, 50830b18eb1687785db99f2171d495c446de657b, f93615a8721824bb657a93ecaa6d5a0aef757581, 4d0a5d7e7a6e228d6451cfa5bd1909e26d2ab0ae, b562df46705583c102335226ed956c0fe335b229, ca424b7f1b7e20f9eb3fe2f21ff7f35e39e446d1, f88009a5e1a39d5ad53428dff28c3e8110f284cc, 5820dc15fe861324ed184299279bab95d44e49ec, 9a8128a6cfcfa52c5f41bca40b7ccfbccfa40f1e, f2379818f60966e05e72c1ea802a9811f965276a, 7df6fd3f36b600e1160eec948a63a439d23b0432, be18299eb2d74f16ece47bb352522566d1f739bd, d02046ea02cf08ac84e19233593a81d101d5e080, e4e3d319dc08bd55048cd2cd45c2bf0599fbecde, 875b372680d99f293adf8f85a45f76436cd45ce4, 9af1a4e2886d5a039dce34f5c9ecc2e16d416588, 43fd035d348c53f9cbbe98719943757552037339, 6a60b106ed01d81f47972b5eee4d27f21112eaf4, 5f2264b2bbac955b06607eb25965dece22cca856, 74e51a461d6b8e7a7fa8f16c7ff3aa1724072ed5, 787658d5c54a12787258a2fb27efbc80321bab98, a66fc2e282674f539c01e4c6d625a6133993c26e, 49ef05fea8774376a9579106d6706c5281bc1fc4
- Tests: flock gpu.lock cargo test --profile ci -p telperion-render --no-fail-fast, cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render, npm test
- PRs: