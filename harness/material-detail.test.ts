import { expect, it } from "vitest";
import { ORDINARY } from "../src/browser/core";
import { presetToParams, toFamily } from "./family";

it("carries every surface-detail control from the generated row through the panel", () => {
  const params = presetToParams(ORDINARY);
  const names = ["ridgeScale", "plateScale", "roughnessDetail", "veinScale", "veinContrast",
    "transmissionStrength", "transmissionRed", "transmissionGreen", "transmissionBlue", "thickness"];
  for (const name of names) {
    expect(Object.keys(params.family.material)).toContain(name);
    Object.assign(params.family.material, { [name]: 0.25 });
  }
  expect(toFamily(params).material).toEqual(params.family.material);
});
