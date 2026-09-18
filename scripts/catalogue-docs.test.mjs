// The two rules the public repository rests on, and the gate that stands in
// for regenerating prose. Every case works on a copy of a real species folder
// in a temporary root, so the fixtures cannot drift from the records.
import { afterEach, describe, expect, it } from 'vitest';
import { cpSync, mkdtempSync, mkdirSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { ROOT, CATALOGUE, readSpecies } from './catalogue-pages.mjs';
import { rightsPermitFullCopy, renderSourceCopy, sourceCopyPath } from './catalogue-sources.mjs';
import { validateArticle, staleInputs } from './catalogue-article.mjs';
import { run } from './catalogue-check.mjs';

const SPECIES = 'oregon-white-oak';
const roots = [];

/** A temporary repository root holding one real species folder. */
function sandbox() {
  const root = mkdtempSync(join(tmpdir(), 'catalogue-'));
  roots.push(root);
  mkdirSync(join(root, CATALOGUE), { recursive: true });
  cpSync(join(ROOT, CATALOGUE, SPECIES), join(root, CATALOGUE, SPECIES), { recursive: true });
  cpSync(join(ROOT, '.gitattributes'), join(root, '.gitattributes'));
  return root;
}

const article = (root) => join(root, CATALOGUE, SPECIES, 'ARTICLE.md');
const edit = (path, change) => writeFileSync(path, change(readFileSync(path, 'utf8')));

afterEach(() => {
  for (const root of roots.splice(0)) rmSync(root, { recursive: true, force: true });
});

describe('the rights rule', () => {
  it('permits a full copy only on a named open licence or public domain', () => {
    expect(rightsPermitFullCopy('European Commission JRC, CC BY 4.0; cited with attribution')).toBe(true);
    expect(rightsPermitFullCopy('US Forest Service publication, public domain; cited with attribution')).toBe(true);
  });

  it('withholds it on silence, ambiguity and an inferred refusal', () => {
    expect(rightsPermitFullCopy('web page; cited with attribution')).toBe(false);
    expect(rightsPermitFullCopy('published tables; extract cited with attribution')).toBe(false);
    expect(rightsPermitFullCopy('published article; cited with attribution, no redistribution inferred')).toBe(false);
    expect(rightsPermitFullCopy('')).toBe(false);
    expect(rightsPermitFullCopy(undefined)).toBe(false);
  });

  it('writes an extract even when handed the whole source text', () => {
    const source = { id: 'X', url: 'https://example.invalid', title: 't', attribution: 'a', rights: 'web page; cited with attribution' };
    const copy = renderSourceCopy(source, { text: 'One sentence. Another sentence.', passages: [{ location: 'p1', quote: 'One sentence.' }], fetched: '2026-09-18' });
    expect(copy).toContain('form: extract');
    expect(copy).toContain('> One sentence.');
    expect(copy).not.toContain('Another sentence.');
  });

  it('refuses a quotation that is not a run of the fetched text', () => {
    const source = { id: 'X', url: 'https://example.invalid', title: 't', attribution: 'a', rights: 'web page' };
    expect(() => renderSourceCopy(source, {
      text: 'The crown is rounded.',
      passages: [{ location: 'p1', quote: 'The crown is 30 m wide.' }],
      fetched: '2026-09-18',
    })).toThrow(/not a run of the fetched text/);
  });
});

describe('the structure check', () => {
  it('fails a full copy filed under rights that do not permit one', () => {
    const root = sandbox();
    const path = sourceCopyPath(root, SPECIES, 'OSU-OAK');
    edit(path, (text) => text.replace('form: extract', 'form: full'));
    expect(run(root)).toContain(`${CATALOGUE}/${SPECIES}/sources/OSU-OAK.md: rights do not permit a full copy`);
  });

  it('fails a copy whose recorded source checksum has moved', () => {
    const root = sandbox();
    const sources = join(root, CATALOGUE, SPECIES, 'sources.json');
    const doc = JSON.parse(readFileSync(sources, 'utf8'));
    doc.sources.find((source) => source.id === 'OSU-OAK').sha256 = 'f'.repeat(64);
    writeFileSync(sources, `${JSON.stringify(doc, null, 1)}\n`);
    expect(run(root)).toContain(
      `${CATALOGUE}/${SPECIES}/sources/OSU-OAK.md: source_sha256 does not match the checksum recorded in sources.json`);
  });

  it('fails a species folder with no source copy', () => {
    const root = sandbox();
    rmSync(sourceCopyPath(root, SPECIES, 'USFS-OAK'));
    expect(run(root)).toContain(`${CATALOGUE}/${SPECIES}: missing sources/USFS-OAK.md`);
  });

  it('fails an article whose unsupported-claim decision is still open', () => {
    const root = sandbox();
    const path = join(root, CATALOGUE, SPECIES, 'decisions.json');
    writeFileSync(path, `${JSON.stringify({
      schema: 'decisions',
      schema_version: 1,
      decisions: [{
        id: `${SPECIES}/document/article-claim-unsupported/OSU-OAK`,
        species: SPECIES,
        stage: 'document',
        kind: 'article-claim-unsupported',
        status: 'open',
        blocks: ['report'],
      }],
    }, null, 1)}\n`);
    expect(run(root)).toContain(
      `${CATALOGUE}/${SPECIES}/ARTICLE.md: decision ${SPECIES}/document/article-claim-unsupported/OSU-OAK is open (article-claim-unsupported)`);
  });

  it('fails an article whose record has moved', () => {
    const root = sandbox();
    edit(join(root, CATALOGUE, SPECIES, 'sources.json'), (text) => `${text}\n`);
    expect(staleInputs(root, SPECIES)).toContain('sources.json');
    expect(run(root).some((failure) => failure.includes('sources.json changed since the article was written'))).toBe(true);
  });
});

describe('the article', () => {
  it('fails a claim that carries no source id', () => {
    const root = sandbox();
    edit(article(root), (text) => text.replace('## Bark\n\n', '## Bark\n\nThe bark is grey.\n\n'));
    const failures = validateArticle(root, SPECIES, readSpecies(root, SPECIES));
    expect(failures.some((failure) => failure.endsWith('claim carries no source id'))).toBe(true);
  });

  it('fails a number that appears in no packet record', () => {
    const root = sandbox();
    edit(article(root), (text) =>
      text.replace('## Bark\n\n', '## Bark\n\nIt reaches 999 m. [OSU-OAK](sources/OSU-OAK.md)\n\n'));
    const failures = validateArticle(root, SPECIES, readSpecies(root, SPECIES));
    expect(failures.some((failure) => failure.includes('the number 999 appears in no packet record'))).toBe(true);
  });

  it('passes a number the packet does record, and an id in a citation is not a number', () => {
    const root = sandbox();
    const species = readSpecies(root, SPECIES);
    const height = species.profile.profiles[0].metrics.height_m.range[1];
    edit(article(root), (text) =>
      text.replace('## Bark\n\n', `## Bark\n\nIt reaches ${height} m. [OSU-OAK](sources/OSU-OAK.md)\n\n`));
    expect(validateArticle(root, SPECIES, species)).toEqual([]);
  });

  it('fails an unfilled section', () => {
    const root = sandbox();
    edit(article(root), (text) => text.replace(/^## Bark\n[\s\S]*?(?=^## Leaves)/m, '## Bark\n\n'));
    const failures = validateArticle(root, SPECIES, readSpecies(root, SPECIES));
    expect(failures).toContain(`${CATALOGUE}/${SPECIES}/ARTICLE.md: section "Bark" is unfilled`);
  });
});
