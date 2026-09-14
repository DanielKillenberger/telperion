// Render templates/species-spec.md for one species and create the flow spec.
//
//   node scripts/new-species-spec.mjs --id european-ash \
//     --scientific "Fraxinus excelsior" --common "European ash" \
//     --model "Rauh" --organs "pinnate compound leaf, samara" \
//     [--base-for "Yggdrasil (fn-10)"] [--context "mature open-grown"] [--dry-run]
//
// Every placeholder must be filled; a leftover {{...}} fails the run naming it.
import { execFileSync } from 'node:child_process';
import { readFile, writeFile } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const args = process.argv.slice(2);
const option = (name) => { const i = args.indexOf(name); return i >= 0 ? args[i + 1] : undefined; };
const flag = (name) => args.includes(name);

const id = option('--id');
const scientific = option('--scientific');
const common = option('--common');
const model = option('--model');
const organs = option('--organs');
if (!id || !scientific || !common || !model || !organs) {
  console.error('required: --id --scientific --common --model --organs');
  process.exit(2);
}
if (!/^[a-z][a-z0-9-]*$/.test(id)) {
  console.error(`catalogue id must be lowercase kebab-case: ${id}`);
  process.exit(2);
}
const baseFor = option('--base-for');
const values = {
  id,
  scientific_name: scientific,
  common_name: common,
  model,
  organs,
  context: option('--context') ?? 'mature open-grown',
  base_for: baseFor ? `Natural base for ${baseFor}. ` : '',
};

const template = await readFile(new URL('../templates/species-spec.md', import.meta.url), 'utf8');
const body = template.replace(/\{\{(\w+)\}\}/g, (_, key) => {
  if (!(key in values)) { console.error(`unfilled placeholder: {{${key}}}`); process.exit(2); }
  return values[key];
});
const leftover = body.match(/\{\{[^}]*\}\}/);
if (leftover) { console.error(`unfilled placeholder: ${leftover[0]}`); process.exit(2); }

const plan = join(tmpdir(), `species-spec-${id}-${process.pid}.md`);
await writeFile(plan, body);
const title = `${common} as a real species`;
if (flag('--dry-run')) { console.log(`title: ${title}\nplan: ${plan}`); process.exit(0); }

const flowctl = process.env.FLOWCTL ?? 'flowctl';
const out = execFileSync(flowctl, ['spec', 'create', '--title', title, '--plan-file', plan, '--json'], { encoding: 'utf8' });
const created = JSON.parse(out);
console.log(`${created.id}\t${plan}`);
