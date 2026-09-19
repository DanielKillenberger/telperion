#!/usr/bin/env node
// The one-off backfill this spec ran: the five species in the catalogue
// predate the pipeline's `document` stage, so their source copies were written
// from a hand-warmed fetch cache rather than from a run. It reads the passage
// selections beside it, copies the quotes out of the cache by line range, and
// calls the same writer the stage calls. Kept as evidence of how the five
// folders were filled; a new species never needs it.
//
//   node .flow/evidence/fn-81-the-species-article-the-sources/backfill.mjs <species>
import { execFileSync } from 'node:child_process';
import { existsSync, readFileSync, writeFileSync, mkdtempSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const HERE = new URL('.', import.meta.url).pathname;
const ROOT = join(HERE, '../../..');
const species = process.argv[2];
if (!species) throw new Error('usage: node backfill.mjs <species>');

const cache = join(ROOT, '.flow/evidence', species, 'pipeline/cache');
const selections = existsSync(join(HERE, 'passages', `${species}.json`))
  ? JSON.parse(readFileSync(join(HERE, 'passages', `${species}.json`), 'utf8'))
  : {};
const sources = JSON.parse(readFileSync(join(ROOT, 'catalogue', species, 'sources.json'), 'utf8')).sources;
const scratch = mkdtempSync(join(tmpdir(), 'backfill-'));

for (const source of sources) {
  const fetched = join(cache, `${source.id}.md`);
  const args = ['scripts/catalogue-sources.mjs', '--species', species, '--source', source.id];
  if (!existsSync(fetched)) {
    args.push('--unavailable');
  } else {
    args.push('--from', fetched, '--fetched', '2026-09-19');
    const passages = selections[source.id];
    if (passages) {
      const path = join(scratch, `${source.id}.json`);
      writeFileSync(path, JSON.stringify(passages));
      args.push('--passages', path);
    }
  }
  process.stdout.write(execFileSync('node', args, { cwd: ROOT, encoding: 'utf8' }));
}
// A scaffolded article exits non-zero until a writer has filled it; that is
// the command reporting work, not the backfill failing.
try {
  process.stdout.write(execFileSync('node', ['scripts/catalogue-article.mjs', '--species', species],
    { cwd: ROOT, encoding: 'utf8', stdio: ['ignore', 'pipe', 'inherit'] }));
} catch (error) {
  process.stdout.write(error.stdout ?? '');
}
