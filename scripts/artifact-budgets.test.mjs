import { describe, expect, it } from 'vitest';
import { budgets, check } from './artifact-budgets.mjs';

/* The size budget (docs/principles.md): the shipped 0.1.4 package passes,
 * and fn-150's rejected first slim build (+48%) fails. */
describe('artifact budgets', () => {
  it('pass on 0.1.4 and fail on the rejected slim build', async () => {
    const b = await budgets();
    const shipped = Object.fromEntries(b.artifacts.map(a => [a.path, a.measuredBytes]));
    expect(check(b, shipped)).toEqual([]);
    const fails = check(b, { ...shipped, 'dist/telperion-field.wasm': 526_965 });
    expect(fails).toHaveLength(1);
    expect(fails[0]).toMatch(/^dist\/telperion-field\.wasm: 526965 bytes, budget 362000 \(\+52\.8%/);
    const { 'dist/field.js': _, ...missing } = shipped;
    expect(check(b, missing)[0]).toMatch(/^dist\/field\.js: not built/);
  });
});
