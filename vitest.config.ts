import { defineConfig } from "vitest/config";

/* Pure units only. No GPU and no browser on CI, and nothing under
   src/ needs either: the generator is geometry, and geometry is
   checkable without a screen. The look is judged by a human in the
   harness, never asserted here. */
export default defineConfig({
  test: {
    include: ["src/**/*.test.ts", "harness/**/*.test.ts"],
    environment: "node",
  },
});
