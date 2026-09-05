import * as THREE from "three";
const keys = [
  "radialSegments",
  "lobes",
  "lobeDepth",
  "twistRate",
  "flareRadius",
  "flareFalloff",
  "flareDepth",
  "forkSocket",
  "forkSwell",
];
export function pack(c: any): Float64Array {
  const a = new Float64Array(11 + c.skeleton.nodes.length * 6);
  a[0] = c.skeleton.nodes.length;
  a[1] = c.envelope.height;
  keys.forEach((k, i) => (a[2 + i] = c.params[k]));
  c.skeleton.nodes.forEach((n: any, i: number) => {
    const j = 11 + i * 6;
    a[j] = n.parent;
    a[j + 1] = n.position.x;
    a[j + 2] = n.position.y;
    a[j + 3] = n.position.z;
    a[j + 4] = c.field.radius[i];
    a[j + 5] = c.field.startRadius[i];
  });
  return a;
}
export function unpack(a: Float64Array): any {
  const nodes = [];
  const radius = new Float64Array(a[0]);
  const startRadius = new Float64Array(a[0]);
  for (let i = 0; i < a[0]; i++) {
    let j = 11 + i * 6;
    nodes.push({
      parent: a[j],
      position: new THREE.Vector3(a[j + 1], a[j + 2], a[j + 3]),
    });
    radius[i] = a[j + 4];
    startRadius[i] = a[j + 5];
  }
  return {
    skeleton: { nodes },
    field: { radius, startRadius },
    envelope: { height: a[1] },
    params: Object.fromEntries(keys.map((k, i) => [k, a[2 + i]])),
  };
}
export function summary(samples: number[]) {
  const s = [...samples].sort((a, b) => a - b);
  return {
    samples,
    median:
      s.length % 2
        ? s[Math.floor(s.length / 2)]
        : (s[s.length / 2 - 1] + s[s.length / 2]) / 2,
    p95: s[Math.ceil(0.95 * s.length) - 1],
  };
}
