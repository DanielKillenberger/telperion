import * as THREE from "three";
import { growReport } from "./frozen/src/skeleton/grow";
import { solveRadii, DEFAULT_RADII } from "./frozen/src/radius";
import { DEFAULT_ENVELOPE } from "./frozen/src/envelope";
import { DEFAULT_SURFACE } from "./frozen/src/mesh/surface";
import { TELPERION, LAURELIN } from "./frozen/src/presets/two-trees";
import { pack } from "./shared";
import { mkdirSync, writeFileSync } from "node:fs";
import { createHash } from "node:crypto";
const dir = new URL("./fixtures/", import.meta.url);
mkdirSync(dir, { recursive: true });
const cases: any[] = [];
for (const preset of [TELPERION, LAURELIN]) {
  const report = growReport(preset.skeleton, preset.radii);
  cases.push({
    name: preset.id,
    skeleton: report.skeleton,
    field: solveRadii(report.skeleton, preset.skeleton.envelope, preset.radii),
    envelope: preset.skeleton.envelope,
    params: preset.surface,
    provenance: {
      seed: preset.skeleton.seed,
      capped: report.capped,
      shed: report.shed,
      preset,
    },
  });
}
const shapes: Record<string, number[][]> = {
  empty: [],
  root: [[-1, 0, 0, 0]],
  straight: [
    [-1, 0, 0, 0],
    [0, 0, 4, 0],
    [1, 0, 8, 0],
  ],
  fork: [
    [-1, 0, 0, 0],
    [0, 0, 4, 0],
    [1, 2, 6, 0],
    [1, -2, 6, 0],
  ],
  duplicates: [
    [-1, 0, 0, 0],
    [0, 0, 0, 0],
    [1, 0, 4, 0],
    [2, 2, 6, 0],
    [2, -2, 6, 0],
  ],
  reversal: [
    [-1, 0, 0, 0],
    [0, 0, 4, 0],
    [1, 0, 0, 0],
  ],
  all_duplicates: [
    [-1, 0, 0, 0],
    [0, 0, 0, 0],
    [1, 0, 0, 0],
  ],
  root_fork: [
    [-1, 0, 0, 0],
    [0, 1, 2, 0],
    [0, -1, 2, 0],
  ],
};
for (const [name, rows] of Object.entries(shapes)) {
  const skeleton = {
    nodes: rows.map(([parent, x, y, z]) => ({
      parent,
      position: new THREE.Vector3(x, y, z),
    })),
  };
  cases.push({
    name,
    skeleton,
    field: solveRadii(skeleton, DEFAULT_ENVELOPE, DEFAULT_RADII),
    envelope: DEFAULT_ENVELOPE,
    params: DEFAULT_SURFACE,
  });
}
for (const [name, params] of Object.entries({
  no_burial: { flareDepth: 0, lobes: 0 },
  rails: {
    radialSegments: 1,
    lobes: -4,
    lobeDepth: 8,
    flareRadius: 0,
    flareFalloff: 0,
    forkSocket: -1,
    forkSwell: -3,
    twistRate: NaN,
  },
  max_rails: {
    radialSegments: Number.MAX_VALUE,
    lobes: Number.MAX_VALUE,
    flareRadius: Number.MAX_VALUE,
    flareDepth: Number.MAX_VALUE,
    flareFalloff: Number.MAX_VALUE,
    twistRate: Number.MAX_VALUE,
    forkSocket: Number.MAX_VALUE,
    forkSwell: Number.MAX_VALUE,
  },
})) {
  const base = cases.find((c) => c.name === "fork");
  cases.push({ ...base, name, params: { ...DEFAULT_SURFACE, ...params } });
}
const manifest = cases.map((c) => {
  const binary = pack(c);
  const bytes = Buffer.from(binary.buffer);
  writeFileSync(new URL(c.name + ".bin", dir), bytes);
  return {
    name: c.name,
    nodes: c.skeleton.nodes.length,
    inputBytes: bytes.length,
    inputSha256: createHash("sha256").update(bytes).digest("hex"),
    provenance: c.provenance ?? "synthetic hand-authored topology",
  };
});
writeFileSync(
  new URL("manifest.json", dir),
  JSON.stringify(manifest, null, 2) + "\n",
);
console.log(
  JSON.stringify(
    manifest.map(({ name, nodes, inputBytes }) => ({
      name,
      nodes,
      inputBytes,
    })),
  ),
);
