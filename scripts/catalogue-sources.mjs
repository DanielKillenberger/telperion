#!/usr/bin/env node
// The source copies a species folder keeps, and the rights rule that decides
// their shape. The repository is public, so a full markdown copy is written
// only under an explicit permitting rights statement - a named open licence or
// a public-domain statement. Silence, ambiguity and "no redistribution
// inferred" all mean an extract of the cited passages, which is quotation
// rather than republication.
//
//   node scripts/catalogue-sources.mjs --species <id>
//   node scripts/catalogue-sources.mjs --species <id> --source <sid> --from <fetched.md>
//   node scripts/catalogue-sources.mjs --species <id> --source <sid> --from <fetched.md> \
//       --passages <passages.json>
//
// Without `--source` the command reports, per source, the shape its rights
// require and whether a copy exists. With `--from` it writes the copy: the
// whole fetched markdown where the rights permit it, and otherwise only the
// passages named in `--passages` ([{location, quote}, ...] or, better,
// [{location, lines: [from, to]}, ...]), each verified to be a literal run of
// the fetched text so no quotation can be invented. A
// source whose text cannot be fetched is recorded with `--unavailable` and
// states the gap in place of the passages.
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync, mkdirSync, existsSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { ROOT, CATALOGUE } from './catalogue-pages.mjs';

/** A licence statement that explicitly permits redistribution of the text. */
const PERMITS = [
  /\bCC0\b/i,
  /\bCC[\s-]?BY(?:[\s-]?SA)?[\s-]?\d/i,
  /\bpublic domain\b/i,
];
/** A statement that withholds it, whatever else the sentence says. */
const WITHHOLDS = [/\bno redistribution\b/i, /\ball rights reserved\b/i];

/**
 * Whether a rights sentence permits a full copy. Conservative by construction:
 * anything it cannot classify withholds.
 */
export function rightsPermitFullCopy(rights) {
  const text = String(rights ?? '');
  if (WITHHOLDS.some((pattern) => pattern.test(text))) return false;
  return PERMITS.some((pattern) => pattern.test(text));
}

/** The form a source's copy must take under its rights. */
export const formFor = (rights) => (rightsPermitFullCopy(rights) ? 'full' : 'extract');

export const sha256 = (text) => createHash('sha256').update(text).digest('hex');

const squeeze = (text) => text.replace(/\s+/g, ' ').trim();

/**
 * Read `---` front matter as a flat record. A bare `key: value` is a string, a
 * `[a, b]` value is a list, and an indented block under `key:` is a nested
 * record. Enough for the two documents this project writes, and nothing more.
 */
export function readFrontMatter(text) {
  const match = /^---\n([\s\S]*?)\n---\n?/.exec(text);
  if (!match) return { front: null, body: text };
  const front = {};
  let nested = null;
  for (const line of match[1].split('\n')) {
    if (line.trim() === '') continue;
    const indented = /^\s/.test(line);
    const pair = /^\s*([^:]+):\s*(.*)$/.exec(line);
    if (!pair) continue;
    const [, key, raw] = pair;
    if (indented && nested) {
      nested[key.trim()] = raw.trim();
      continue;
    }
    nested = null;
    if (raw === '') {
      nested = {};
      front[key.trim()] = nested;
    } else if (raw.startsWith('[') && raw.endsWith(']')) {
      const inner = raw.slice(1, -1).trim();
      front[key.trim()] = inner === '' ? [] : inner.split(',').map((item) => item.trim());
    } else {
      front[key.trim()] = raw.trim();
    }
  }
  return { front, body: text.slice(match[0].length) };
}

/** Render a front-matter record back, in the same shape `readFrontMatter` reads. */
export function writeFrontMatter(record) {
  const lines = ['---'];
  for (const [key, value] of Object.entries(record)) {
    if (Array.isArray(value)) lines.push(`${key}: [${value.join(', ')}]`);
    else if (value && typeof value === 'object') {
      lines.push(`${key}:`);
      for (const [inner, own] of Object.entries(value)) lines.push(`  ${inner}: ${own}`);
    } else lines.push(`${key}: ${value}`);
  }
  lines.push('---', '');
  return lines.join('\n');
}

export const sourcesDir = (root, id) => join(root, CATALOGUE, id, 'sources');
export const sourceCopyPath = (root, id, sourceId) => join(sourcesDir(root, id), `${sourceId}.md`);

/** The preamble every copy opens with: the original link and its terms. */
function preamble(source, form) {
  const shape = form === 'full'
    ? 'full copy, permitted by the rights statement above'
    : 'cited passages only; the rights statement above does not permit a full copy';
  return [
    `# ${source.title}`,
    '',
    `- **Original:** <${source.url}>`,
    `- **Attribution:** ${source.attribution}`,
    `- **Rights:** ${source.rights}`,
    `- **Form:** ${shape}.`,
    '',
    '---',
    '',
    '',
  ].join('\n');
}

/**
 * The markdown of one source copy. `text` is the fetched source markdown, or
 * null when the source could not be reached; `passages` are the cited runs an
 * extract keeps, each verified against `text`.
 */
export function renderSourceCopy(source, { text = null, passages = [], fetched }) {
  const form = formFor(source.rights);
  const front = writeFrontMatter({
    source: source.id,
    url: source.url,
    title: source.title,
    attribution: source.attribution,
    rights: source.rights,
    fetched: fetched ?? 'unavailable',
    // Two checksums, because they pin two different things: `sha256` is the
    // text this copy was made from, and `source_sha256` is what the record
    // pins for the source's raw bytes - a PDF, where the copy is markdown.
    sha256: text === null ? 'null' : sha256(text),
    source_sha256: source.sha256 ?? 'null',
    form,
  });

  if (text === null) {
    return `${front}\n${preamble(source, form)}The source text could not be retrieved, so this copy carries no passage. What the packet takes from this source is recorded against each metric in \`packet/profile.json\`.\n`;
  }
  if (form === 'full') return `${front}\n${preamble(source, form)}${text.trimEnd()}\n`;

  const haystack = squeeze(text);
  const lines = text.split('\n');
  const out = [front, '\n', preamble(source, form), '## Cited passages\n'];
  for (const passage of passages) {
    // A passage is named either by its text or by the lines it runs over. The
    // line form is the safer one: the writer chooses, and code does the copying.
    const quote = passage.lines
      ? lines.slice(passage.lines[0] - 1, passage.lines[1]).join('\n')
      : passage.quote;
    if (!haystack.includes(squeeze(quote))) {
      throw new Error(`passage "${squeeze(quote).slice(0, 60)}..." is not a run of the fetched text`);
    }
    out.push(`\n### ${passage.location}\n`);
    for (const line of quote.trim().split('\n')) out.push(`\n> ${line.trim()}`);
    out.push('\n');
  }
  if (passages.length === 0) out.push('\n_No passage selected yet._\n');
  return out.join('');
}

/**
 * Every failure the source copies carry, in the check's `<path>: <reason>`
 * voice. The repository is public, so this is where the rights rule is
 * enforced: a full copy filed under rights that do not explicitly permit
 * redistribution fails, and so does one the rule cannot classify.
 */
export function validateSourceCopies(root, id, doc) {
  const failures = [];
  for (const source of doc.sources ?? []) {
    const where = `${CATALOGUE}/${id}/sources/${source.id}.md`;
    const path = sourceCopyPath(root, id, source.id);
    if (!existsSync(path)) {
      failures.push(`${CATALOGUE}/${id}: missing sources/${source.id}.md`);
      continue;
    }
    const { front } = readFrontMatter(readFileSync(path, 'utf8'));
    if (!front) {
      failures.push(`${where}: has no front matter`);
      continue;
    }
    for (const field of ['source', 'url', 'title', 'attribution', 'rights']) {
      const expected = field === 'source' ? source.id : source[field];
      if (front[field] !== expected) failures.push(`${where}: ${field} does not match sources.json`);
    }
    for (const field of ['fetched', 'sha256']) {
      if (!front[field]) failures.push(`${where}: ${field} is empty`);
    }
    if (front.source_sha256 !== String(source.sha256 ?? 'null')) {
      failures.push(`${where}: source_sha256 does not match the checksum recorded in sources.json`);
    }
    if (front.form !== formFor(source.rights)) {
      failures.push(front.form === 'full'
        ? `${where}: rights do not permit a full copy`
        : `${where}: is an extract of a source whose rights permit the full text`);
    }
  }
  return failures;
}

function readSources(root, id) {
  const doc = JSON.parse(readFileSync(join(root, CATALOGUE, id, 'sources.json'), 'utf8'));
  return doc.sources ?? [];
}

function arg(name) {
  const at = process.argv.indexOf(`--${name}`);
  return at === -1 ? null : process.argv[at + 1] ?? null;
}

function main() {
  const species = arg('species');
  if (!species) {
    console.error('usage: node scripts/catalogue-sources.mjs --species <id> [--source <sid> --from <file>]');
    process.exit(2);
  }
  const sources = readSources(ROOT, species);
  const wanted = arg('source');

  if (!wanted) {
    for (const source of sources) {
      const path = sourceCopyPath(ROOT, species, source.id);
      console.log(`${source.id}\t${formFor(source.rights)}\t${existsSync(path) ? 'written' : 'missing'}`);
    }
    return;
  }

  const source = sources.find((candidate) => candidate.id === wanted);
  if (!source) {
    console.error(`${CATALOGUE}/${species}/sources.json: no source ${wanted}`);
    process.exit(1);
  }
  const from = arg('from');
  const unavailable = process.argv.includes('--unavailable');
  if (!from && !unavailable) {
    console.error(`--from <fetched.md> or --unavailable is required to write ${wanted}`);
    process.exit(2);
  }
  const text = unavailable ? null : readFileSync(resolve(from), 'utf8');
  const passagesPath = arg('passages');
  const passages = passagesPath ? JSON.parse(readFileSync(resolve(passagesPath), 'utf8')) : [];
  const fetched = arg('fetched') ?? new Date().toISOString().slice(0, 10);

  mkdirSync(sourcesDir(ROOT, species), { recursive: true });
  const copy = renderSourceCopy(source, { text, passages, fetched: unavailable ? null : fetched });
  const path = sourceCopyPath(ROOT, species, source.id);
  writeFileSync(path, copy);
  console.log(`wrote ${CATALOGUE}/${species}/sources/${source.id}.md (${formFor(source.rights)}, ${copy.length} bytes)`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) main();
