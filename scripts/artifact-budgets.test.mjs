import { describe, expect, it } from 'vitest';
import { budgets, check, declaredGrowth, findBase } from './artifact-budgets.mjs';

/* The two size limits (docs/principles.md), on recorded sizes: the owner's
 * ceilings, and the per-PR growth share over the base, read through an
 * injected reader so nothing here touches the network. */
const b = await budgets();
const recorded = Object.fromEntries(b.artifacts.map(a => [a.path, a.baselineBytes]));
const FIELD = 'dist/telperion-field.wasm';
const MAIN = 'dist/telperion.wasm';
// fn-150's first slim build: 355,100 to 526,965 bytes (its FRICTION.md).
const fn150 = { base: { ...recorded, [FIELD]: 355_100 }, head: { ...recorded, [FIELD]: 526_965 } };
// fn-170's fork rule: master 1,395,510 and 358,698 bytes to its CI build.
const fn170 = { base: { ...recorded, [MAIN]: 1_395_510, [FIELD]: 358_698 }, head: recorded };
const on = (/** @type {Record<string, number>} */ sizes) => ({ label: 'abcd1234', sizes });

describe('artifact ceilings', () => {
  it('fail an artifact over its ceiling, naming the ceiling and its reason', () => {
    const field = b.artifacts.find(a => a.path === FIELD);
    expect(check(b, recorded)).toEqual([]);
    expect(check(b, { ...recorded, [FIELD]: field.ceilingBytes + 1 })).toEqual([
      `${FIELD}: ${field.ceilingBytes + 1} bytes, over its ceiling ${field.ceilingBytes} (${field.reason}); crossing a ceiling is an owner decision`,
    ]);
  });

  it('fail an artifact the recipe did not build', () => {
    const { 'dist/field.js': _, ...missing } = recorded;
    expect(check(b, missing)).toEqual([expect.stringMatching(/^dist\/field\.js: not built/)]);
  });
});

describe('growth per PR', () => {
  it('fails fn-150s rejected slim build, naming the base and the growth', () => {
    const fails = check(b, fn150.head, { base: on(fn150.base) });
    expect(fails).toContain(
      `${FIELD}: 526965 bytes, +48.4% over 355100 on the base abcd1234, above the 5.0% growth share per PR; name telperion-field.wasm in the PR's Decisions section, or shrink the artifact`,
    );
  });

  it('passes fn-170s fork rule under the budget file as it stands', () => {
    expect(check(b, fn170.head, { base: on(fn170.base) })).toEqual([]);
  });

  it('passes a growth the Decisions section names, and only that one', () => {
    const grown = { ...recorded, [FIELD]: 380_000, [MAIN]: 1_500_000 };
    const body = [
      '## Change', 'telperion.wasm is not declared here.',
      '## Decisions', '- D1: the leaf plan grows `dist/telperion-field.wasm` by 5 percent.',
      '## Open', '- telperion.wasm grows too.',
    ].join('\r\n');
    const declared = declaredGrowth(b, body);
    expect([...declared]).toEqual([FIELD]);
    const base = on({ ...recorded, [FIELD]: 355_100, [MAIN]: 1_400_000 });
    expect(check(b, grown, { base, declared })).toEqual([expect.stringMatching(/^dist\/telperion\.wasm: 1500000 bytes, \+7\.1%/)]);
    expect(declaredGrowth(b, '## Decisions\n- telperion-render.wasm and telperion.json')).toEqual(new Set(['dist/telperion-render.wasm']));
  });
});

describe('the base', () => {
  const runs = [
    { id: 3, sha: 'c'.repeat(40), url: 'run/3' },
    { id: 2, sha: 'b'.repeat(40), url: 'run/2' },
    { id: 1, sha: 'a'.repeat(40), url: 'run/1' },
  ];
  /** A reader over recorded runs: which are ancestors, which still hold a package. */
  const reader = (/** @type {Set<number>} */ ancestors, /** @type {Record<number, Record<string, number>>} */ packages) => ({
    runs: async () => runs,
    isAncestor: async (/** @type {string} */ sha) => ancestors.has(runs.find(r => r.sha === sha).id),
    packageSizes: async (/** @type {number} */ id) => packages[id] ?? null,
  });

  it('is the newest green master ancestor whose package still exists', async () => {
    const base = await findBase(reader(new Set([2, 1]), { 1: fn170.base, 3: fn150.base }), 'head', []);
    expect(base).toEqual({ label: 'aaaaaaaa (run/1)', sizes: fn170.base });
  });

  it('is absent when no ancestor holds a package, so growth goes unchecked', async () => {
    expect(await findBase(reader(new Set([2]), { 3: fn150.base }), 'head', [])).toBeNull();
  });
});
