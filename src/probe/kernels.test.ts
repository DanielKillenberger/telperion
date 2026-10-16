import { describe, expect, it } from "vitest";
import { CHAIN, CYCLE } from "./kernels";

/* Every helper a composed module calls is defined in it: the contract's
 * marked arithmetic (`exact_*`, cycle/arith.wgsl) came without its
 * definitions once, and only Chrome's compile caught it. */
const called = (source: string): string[] => [...new Set([...source.matchAll(/\b(exact_\w+)\s*\(/g)].map((m) => m[1]))];

describe("the probe's cycle modules", () => {
  for (const kit of [...CYCLE, CHAIN]) {
    it(`${kit.name} defines the arithmetic it calls`, () => {
      for (const name of called(kit.source)) {
        expect(kit.source, name).toMatch(new RegExp(`\\bfn ${name}\\s*\\(`));
      }
    });
  }
});
