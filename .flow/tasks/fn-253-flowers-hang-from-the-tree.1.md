---
satisfies: [R1, R2, R3, R4]
---
# fn-253-flowers-hang-from-the-tree.1 Measure the seats, seat at tips, draw anchored stalks

## Description
As the title says; the spec states the requirements.

## Acceptance
- The spec's R-IDs named in `satisfies`.

## Done summary
Each inflorescence is seated at its shoot's tip on both executors (share 1 on the Two Trees; Telperion later 0.3), leaves along the shoot and bends by its droop; its rachis and pedicels are drawn in the organ's one draw between ends no floret transform moves; inflorescences are thinned and enlarged as units. Record: SEATS.md.
- R1: nine in ten flowers sat along a short shoot's four years of wood, every floret 5-39 cm from its wood with nothing drawn between.
- R2: tip seating with no new row; `Bloom.side` deleted.
- R3: pedicel part of the floret at the stamen's width; one rachis instance per inflorescence, tapering from the twig radius, in the shoot's colour; thin rachises as ribbons by chord error; one packing box on both executors (`foliage::bloom_box`).
- R4: tests for seating, stalks, every floret hung through selection with thinning, units whole, ribbons, forced levels; frame cost +2.9 % Laurelin, +4.1 % Telperion at the whole view (host accepted); Astra round 2 AGREE; codex SHIP. fn-246's floret card stays, its fixture moved to the beech at 40.
Open: Laurelin's gold reads denser than the 4x reference at the whole view; Astra's polish on Laurelin's long rachises.

## Evidence
- Commits: 79365f26e441856f4ad1b53faf762ae47d8f788e, 80f3ac29a9492487449f93631ca0402bcbf757a4, 09835ae0f6196574f5d758eb3286a4049895ce2f, 04d2600b1234e7ffb7dc690197c8124451181e66, 369146a511f09721c3bca1c740ece04da57b735c, 2118ef4bf9faa0ae77cfaf607446845018ccb799, 49950270baf34987dcdab0035da1243917a28c1e, bd5b9023affb8ea3d2c186d02dd1726baa76cc61, 2b28657e8d617e9cadf1d862bd9d1164cdcb89bc, 63ea58fb059eaedfc720b61695d960b08301487f, 202f6ca8a417b41631a5dbd08d0575ccc1857196, a1341e7ea96450fa0b93ab9e49364c69c6db1ae6, d4388608b3f3225a7fc48343a23d8ba834d893d8, c06a8d37dc8046cc73b70f766076e651d5a8d12f, 513c6175218098cc50b54337503f262f0e23e762, 04e3d01db35355efac4b30b6c3144bec4f2aadde, 7d65edc1d9cbbe53b6c623c884c56c772989c93c, 8dedf7c01524daa376c1d2060d77fe3e1e620c40, 9061f8b8b7b9ac46b4c50534a3a0a72471bea6f5, 4b63df0dd3b4a95c2e7b14ac567942b2f727371e, 1d373383e1d3a21f645d71c8da35c730b8e57162, a5db2d092f700d63d78da9298054c9c065f3cda8
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock), cargo test --profile ci -p telperion-core/-space/-field/-wasm/-jev --no-fail-fast, npm test
- PRs:
