import { defineConfig } from "vitest/config";

/* Pure units only. No GPU and no browser on CI, and nothing under
   src/ needs either: the generator is geometry, and geometry is
   checkable without a screen. The look is judged by a human in the
   harness, never asserted here. */
export default defineConfig({
  test: {
    include: ["src/**/*.test.ts", "harness/**/*.test.ts"],
    environment: "node",
    /* A preset built in full is seconds of work now that the tree
       branches to the twig; the default 5 s was a timeout on the tree's
       size, not on a regression. */
    testTimeout: 60_000,
  },
});
