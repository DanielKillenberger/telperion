# The browser render test waits for the tree it checks

## Goal & Context

`npm run test:render` (`tests/browser/render.mjs`, which is not in CI) fails on the release branch with "Ordinary generated no wood", and failed the same way on the base. The test came up during fn-311, 2026-10-24.

The owner opened the harness on the release branch at 655171fd7 and clicked Ordinary: "ordinary looks fine to me" (2026-10-24, with a screenshot of the full tree with trunk, branches, leaves and the person). So the product draws wood, and the test is wrong.

The test is from PR #7, before the device engine. For each preset it clicks the preset, waits for the stats line to change, waits one more second, then reads `N wood tris` and requires N > 0. Most likely it reads an intermediate stats line before the device build has finished, but that is not traced. The harness writes the line in `harness/GrowerDev.tsx:388-390`, from `submitted.wood_triangles` in `crates/telperion-render/src/web.rs:441`. It also shows a "let go to make room under the device budget" note while a tree builds.

## Unknown, check before the plan

What the stats line says, in order, from the click to the finished build, read in code: the build states, the device-budget path and the async compile. Then why the fixed one-second wait reads a zero.

## Requirements

- **R1:** the test waits for the build it checks to finish, using a signal the harness already gives (or a new, minimal one), with a stated timeout. It never waits a fixed time.
- **R2:** the test passes on the release branch. Agents do not drive WebGPU in a browser (owner rule), so the owner runs `npm run test:render` and confirms it.
- **R3:** state whether the test should join CI, given it needs a real GPU.

## Boundaries

- No product change. If R1's trace finds a real defect, that becomes its own spec.
- After the release.
