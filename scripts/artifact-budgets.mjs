import { readFile, stat } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

/* Every shipped Wasm module and package script against its budget in
 * artifact-budgets.json, measured on the package CI built (npm run build).
 * An artifact over its budget fails the job: a budget rises only when the
 * PR edits the budget file with the new measurement beside it. Run after
 * the build; it builds nothing itself.
 *
 *   node scripts/artifact-budgets.mjs
 */
const root = new URL('../', import.meta.url);

/** Every artifact over its budget, or missing, as one line each. */
export function check(budgets, sizes) {
  const fails = [];
  for (const { path, maxBytes, measuredBytes, measuredOn } of budgets.artifacts) {
    const size = sizes[path];
    if (size === undefined) fails.push(`${path}: not built by the recipe (${budgets.recipe})`);
    else if (size > maxBytes) {
      const growth = (100 * (size / measuredBytes - 1)).toFixed(1);
      fails.push(`${path}: ${size} bytes, budget ${maxBytes} (+${growth}% over ${measuredBytes} measured on ${measuredOn}); raise it in scripts/artifact-budgets.json with the measurement, or shrink the artifact`);
    }
  }
  return fails;
}

export async function budgets() {
  return JSON.parse(await readFile(new URL('scripts/artifact-budgets.json', root), 'utf8'));
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const b = await budgets();
  const sizes = {};
  for (const { path } of b.artifacts) {
    try {
      sizes[path] = (await stat(new URL(path, root))).size;
      console.log(`${path}: ${sizes[path]} bytes`);
    } catch {}
  }
  const fails = check(b, sizes);
  for (const f of fails) console.error(`FAIL ${f}`);
  process.exit(fails.length ? 1 : 0);
}
