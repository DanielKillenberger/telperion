import { execFileSync } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { beforeAll, describe, expect, it } from "vitest";

import { presetById, type Family } from "../src/browser/core";

import { presetToParams, toFamily } from "./family";
import { fetchLook, open, showing } from "./look";
import { writeRow } from "./rows";

/* A look on the dials (fn-166). The runner's look is written by the
   runner itself, so the family the harness draws is compared with the
   family core builds for `headless --family` over the same overlay. */

const FIXTURE = resolve("crates/telperion-jev/tests/fixtures/look/result.json");
const SPECIES = resolve("target/release/species");

/** `species european-beech --look` over the fixture run: its output and look. */
function runnerLook(): { printed: string; body: Record<string, unknown> } {
  const dir = mkdtempSync(join(tmpdir(), "harness-look-"));
  const tuning = join(dir, "run/runner/tuning");
  mkdirSync(join(tuning, "1"), { recursive: true });
  copyFileSync(FIXTURE, join(tuning, "result.json"));
  copyFileSync(FIXTURE, join(tuning, "1/result.json"));
  const printed = execFileSync(SPECIES, ["european-beech", "--look", "--run-dir", join(dir, "run")],
    { cwd: dir, encoding: "utf8" });
  const body = JSON.parse(readFileSync(join(dir, "harness/looks/european-beech.json"), "utf8"));
  return { printed, body };
}

beforeAll(() => {
  execFileSync("cargo", ["build", "--release", "-p", "telperion-jev", "--bin", "species"], { timeout: 600_000 });
}, 600_000);

describe("a runner's look", () => {
  it("draws the family core builds for headless --family, and says where it comes from", () => {
    const { printed, body } = runnerLook();
    expect(printed).toContain("http://localhost:5173/?look=european-beech&seed=1");
    const opened = open("european-beech", body, null);
    // headless sets the seed it is given over the family, as the seed box does.
    const atSeed = (family: unknown) => writeRow(family as Family, "/skeleton/seed", 1);
    expect(toFamily(opened.look)).toEqual(atSeed(body.family));
    expect(toFamily(opened.shipped)).toEqual(atSeed(body.preset_family));
    expect(opened.source).toMatchObject({ revision: 1, round: 2, label: "bundle@0.5", kept: 1 });
    expect(opened.source?.run).toMatch(/run$/);
  }, 600_000);

  it("takes the run's fixed seed, and says which tree the dials hold", () => {
    const { body } = runnerLook();
    const opened = open("beech", { ...body, seed: 17 }, null);
    expect([opened.look.seed, opened.shipped.seed]).toEqual([17, 17]);
    expect(showing(opened, { ...opened.look, seed: 3 })).toBe("look");
    expect(showing(opened, { ...opened.shipped, seed: 3 })).toBe("shipped");
    const moved = writeRow(opened.look.family, "/shellDepth", 0.2);
    expect(showing(opened, { ...opened.look, family: moved })).toBeNull();
    expect(showing(opened, presetToParams(presetById("norway-spruce")))).toBeNull();
  }, 600_000);

  it("is refused, by the path, where the harness's family would differ from core's", () => {
    const { body } = runnerLook();
    const family = structuredClone(body.family) as { shellDepth: number };
    family.shellDepth += 0.01;
    expect(() => open("beech", { ...body, family }, null)).toThrow("look beech: the harness's family differs from core's at /shellDepth");
    expect(() => open("beech", body, "ordinary")).toThrow("look beech: is laid over european-beech, not ordinary");
  }, 600_000);
});

describe("a plain overlay", () => {
  it("is laid over the shipped preset ?species names", () => {
    const opened = open("candidate", { shellDepth: 0.5, skeleton: { habit: { apicalDominance: 0.3 } } }, "oregon-white-oak");
    const oak = presetById("oregon-white-oak");
    expect(opened.source).toBeNull();
    expect(opened.shipped).toEqual(presetToParams(oak));
    expect(toFamily(opened.look)).toEqual({ ...toFamily(opened.shipped), shellDepth: 0.5,
      skeleton: { ...oak.skeleton, habit: { ...oak.skeleton.habit, apicalDominance: 0.3 } } });
  });

  it.each([
    [{ skeleton: { habit: { stemDivergence: 1 } } }, "oregon-white-oak", "the overlay path /skeleton/habit/stemDivergence is not a row"],
    [{ skeleton: 3 }, "oregon-white-oak", "the overlay path /skeleton is not a row"],
    [{ shellDepth: { x: 1 } }, "oregon-white-oak", "the overlay path /shellDepth is not a row"],
    [{ shellDepth: 0.5 }, null, "a plain overlay needs ?species=<preset> as its base"],
    [{ shellDepth: 0.5 }, "no-such-tree", "Unknown tree preset: no-such-tree"],
  ])("%j over %s is refused, naming %s", (body, species, says) => {
    expect(() => open("candidate", body, species)).toThrow(`look candidate: ${says}`);
  });
});

describe("fetchLook", () => {
  it("opens a served look, and names the file the dev server does not have or a name that is no look's", async () => {
    const answers = [new Response("", { status: 404 }), new Response("<!doctype html>", { headers: { "content-type": "text/html" } })];
    for (const answer of answers) {
      const missing = (async () => answer) as typeof fetch;
      await expect(fetchLook("gone", null, missing)).rejects.toThrow("look gone: the dev server has no /harness/looks/gone.json");
    }
    await expect(fetchLook("../secret", null, fetch)).rejects.toThrow("look ../secret: a look is named");
    const served = (async () => Response.json({ shellDepth: 0.5 })) as typeof fetch;
    await expect(fetchLook("candidate", "ordinary", served)).resolves.toMatchObject({ name: "candidate", preset: "ordinary" });
  });
});
