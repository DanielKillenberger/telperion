function F(o, t) {
  if (!Number.isInteger(t) || t < 3) throw RangeError("a grid needs at least three cells a side");
  const { min: s, max: e } = o, n = Math.max(e[0] - s[0], e[1] - s[1], e[2] - s[2]) / (t - 2), f = [(s[0] + e[0]) / 2 - t * n / 2, s[1] - n, (s[2] + e[2]) / 2 - t * n / 2], u = new Float64Array(t * t * t * 4);
  for (let r = 0; r < t; r++) for (let c = 0; c < t; c++) for (let i = 0; i < t; i++)
    u.set([f[0] + (i + 0.5) * n, f[1] + (c + 0.5) * n, f[2] + (r + 0.5) * n, n / 2], ((r * t + c) * t + i) * 4);
  return { n: t, cell: n, origin: f, cells: u };
}
function S({ n: o, cell: t, origin: s }, e) {
  return [s[0] + (e % o + 0.5) * t, s[1] + (Math.floor(e / o) % o + 0.5) * t, s[2] + (Math.floor(e / (o * o)) + 0.5) * t];
}
const M = 1;
function C(o, t, s) {
  const { n: e } = o, n = e * e * e;
  if (t.flags.length !== n) throw RangeError(`the answer has ${t.flags.length} cells, the grid ${n}`);
  if (!(s.woodCutoff >= 0)) throw RangeError("woodCutoff is a radius in metres");
  const { flags: f, limbs: u, leaves: r, woodRadius: c } = t, i = new Uint8Array(n), h = [], l = (a) => (f[a] & 1) !== 0 && c[a] >= s.woodCutoff;
  for (let a = 0; a < n; a++) l(a) && (i[a] = M, h.push(a));
  if (s.thin === void 0) return { wood: Uint32Array.from(h), foliage: new Uint32Array(0), clumps: 0 };
  const { rule: y, keep: g } = s.thin;
  if (!(g >= 0 && g <= 1)) throw RangeError("keep is a fraction");
  const m = /* @__PURE__ */ new Set(), d = [];
  for (let a = 0; a < n; a++) !l(a) && f[a] & 2 && (d.push(a), m.add(u[a]));
  if (y === "coin") {
    const a = d.filter((w) => R(u[w], g));
    return { wood: Uint32Array.from(h), foliage: Uint32Array.from(a), clumps: m.size };
  }
  const A = v[y](o, d, i, u, r), U = d.map((a, w) => ({ i: a, at: w })).sort((a, w) => A[w.at] - A[a.at]), E = Math.round(d.length * g);
  return { wood: Uint32Array.from(h), foliage: Uint32Array.from(U.slice(0, E), (a) => a.i), clumps: m.size };
}
function R(o, t) {
  let s = Math.imul(o ^ 2654435769, 2246822507);
  return s = Math.imul(s ^ s >>> 13, 3266489909), ((s ^ s >>> 16) >>> 0) / 2 ** 32 < t;
}
const p = (o, t) => [t % o, Math.floor(t / o) % o, Math.floor(t / (o * o))], _ = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]];
function j(o, t) {
  const [s, e, n] = p(o, t), f = [];
  for (const [u, r, c] of _) {
    const i = s + u, h = e + r, l = n + c;
    i >= 0 && h >= 0 && l >= 0 && i < o && h < o && l < o && f.push((l * o + h) * o + i);
  }
  return f;
}
function k(o, t, s) {
  const e = new Int32Array(o * o * o).fill(-1);
  let n = t;
  for (const f of t) e[f] = 0;
  for (; n.length; ) {
    const f = [];
    for (const u of n) for (const r of j(o, u)) e[r] === -1 && s(u, r) && (e[r] = e[u] + 1, f.push(r));
    n = f;
  }
  return e;
}
const v = {
  density: (o, t, s, e, n) => Float64Array.from(t, (f) => n[f]),
  // Nearer drawn wood scores higher; ties broken by density so masses stay full.
  mass: ({ n: o }, t, s, e, n) => {
    const f = [];
    for (let r = 0; r < s.length; r++) s[r] === M && f.push(r);
    const u = k(o, f, () => !0);
    return Float64Array.from(t, (r) => -u[r] + n[r] * 1e-6);
  },
  // A cell scores by how deep it sits in its own system, so the fringe
  // between neighbouring systems goes first and the branch to each mass shows.
  tuft: ({ n: o }, t, s, e) => {
    const n = /* @__PURE__ */ new Map();
    for (const r of t) {
      const [c, i, h] = p(o, r), l = n.get(e[r]) ?? { i: 0, j: 0, k: 0, c: 0 };
      l.i += c, l.j += i, l.k += h, l.c++, n.set(e[r], l);
    }
    const f = (r) => {
      const [c, i, h] = p(o, r), l = n.get(e[r]);
      return Math.hypot(c - l.i / l.c, i - l.j / l.c, h - l.k / l.c);
    }, u = /* @__PURE__ */ new Map();
    for (const r of t) {
      const c = u.get(e[r]) ?? { s: 0, c: 0 };
      c.s += f(r), c.c++, u.set(e[r], c);
    }
    return Float64Array.from(t, (r) => {
      const c = u.get(e[r]);
      return -f(r) / (c.s / c.c + 1e-9);
    });
  },
  // The grid distance to the nearest cell of a different system; a system
  // with no neighbour never erodes and scores as its deepest cell would.
  gap: ({ n: o }, t, s, e, n) => {
    const f = new Uint8Array(o * o * o);
    for (const c of t) f[c] = 1;
    const u = t.filter((c) => j(o, c).some((i) => f[i] && e[i] !== e[c])), r = k(o, u, (c, i) => f[i] === 1 && e[i] === e[c]);
    return Float64Array.from(t, (c) => (r[c] === -1 ? o : r[c]) + n[c] * 1e-6);
  }
};
export {
  S as centre,
  R as coin,
  F as grid,
  C as voxelize
};
