import { defineConfig } from "vitest/config";

// Adapter and harness units run in Node; native invariants run with cargo test.
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
