#!/usr/bin/env node
// The code check that owns the catalogue's structure. It walks every species
// folder and fails one that is missing a required file, a required field, a
// source without rights text, a kept image without a hash, an image whose bytes
// do not match its recorded hash, a raster outside `refs/`, or a page that
// differs from its regeneration.
//
//   node scripts/catalogue-check.mjs
//
// Failures read `catalogue/<id>: missing <file>` or
// `catalogue/<id>/<file>: <field> <reason>`, so the species and the field are
// always named. Whether a source says what the packet claims is a Jev question
// under the fn-58 obligations and is not asked here.
import { createHash } from 'node:crypto';
import { readFileSync, readdirSync, existsSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { ROOT, CATALOGUE, speciesIds, renderPages } from './catalogue-pages.mjs';

const REQUIRED = [
  'sources.json',
  'manifest.json',
  'packet/profile.json',
  'packet/references.json',
  'packet/species.json',
  'packet/specimens.json',
  'provenance.json',
  'decisions.json',
  'resolutions.json',
  'pins.json',
  'stills.json',
  'NOTES.md',
  'README.md',
];

const RASTER = /\.(png|jpe?g|webp|gif|tiff?|bmp|avif)$/i;
const LFS_RULE = 'catalogue/**/refs/** filter=lfs diff=lfs merge=lfs -text';

const failures = [];
const fail = (where, message) => failures.push(`${where}: ${message}`);

const isObject = (value) => typeof value === 'object' && value !== null && !Array.isArray(value);
const isEmptyStub = (value) => isObject(value) && value.empty === true;
const filled = (value) => typeof value === 'string' && value.trim() !== '';

/** Report every required field a record is missing or leaves blank. */
function must(where, record, fields) {
  for (const [field, test, reason] of fields) {
    if (!(field in record)) fail(where, `${field} missing`);
    else if (!test(record[field])) fail(where, `${field} ${reason}`);
  }
}

const present = () => true;
const nonEmpty = [filled, 'is empty'];
const list = [(value) => Array.isArray(value), 'is not a list'];

function checkSources(where, id, doc) {
  must(where, doc, [
    ['schema', (v) => v === 'sources', 'is not "sources"'],
    ['schema_version', (v) => v === 1, 'is not 1'],
    ['species', (v) => v === id, 'does not name this species folder'],
    ['sources', ...list],
  ]);
  const ids = new Set();
  for (const source of doc.sources ?? []) {
    const at = `${where} source ${source.id ?? '(unnamed)'}`;
    must(at, source, [
      ['id', ...nonEmpty],
      ['url', ...nonEmpty],
      ['title', ...nonEmpty],
      ['attribution', ...nonEmpty],
      ['rights', ...nonEmpty],
      ['sha256', (v) => v === null || filled(v), 'is neither a checksum nor null'],
      ['verified', present, ''],
      ['use', present, ''],
      ['tables', ...list],
    ]);
    if (ids.has(source.id)) fail(at, 'id is used twice');
    ids.add(source.id);
    for (const table of source.tables ?? []) {
      must(`${at} table ${table.id ?? '(unnamed)'}`, table, [
        ['id', ...nonEmpty],
        ['table_index', (v) => Number.isInteger(v), 'is not a whole number'],
        ['expected_rows', (v) => Number.isInteger(v), 'is not a whole number'],
        ['dimension', ...nonEmpty],
        ['unit', ...nonEmpty],
        ['value_column', (v) => Number.isInteger(v), 'is not a whole number'],
        ['condition', ...nonEmpty],
        ['taxon', ...nonEmpty],
      ]);
    }
  }
  return ids;
}

function checkProfile(where, id, doc) {
  must(where, doc, [
    ['schema_version', (v) => v === 1, 'is not 1'],
    ['definitions', present, ''],
    ['profiles', (v) => Array.isArray(v) && v.length === 1, 'is not exactly one profile'],
  ]);
  const profile = doc.profiles?.[0];
  if (!isObject(profile)) return;
  must(`${where} profile`, profile, [
    ['id', (v) => v === id, 'does not name this species folder'],
    ['scientific_name', ...nonEmpty],
    ['common_name', ...nonEmpty],
    ['readiness', ...nonEmpty],
    ['context', ...nonEmpty],
    ['sources', ...list],
    ['metrics', isObject, 'is not a record of metrics'],
  ]);
}

function checkReferences(where, id, doc, folder, sourceIds) {
  must(where, doc, [
    ['reference_version', ...nonEmpty],
    ['sources', ...list],
    ['references', ...list],
  ]);
  for (const reference of doc.references ?? []) {
    const at = `${where} reference ${reference.id ?? '(unnamed)'}`;
    must(at, reference, [
      ['id', ...nonEmpty],
      ['species_id', (v) => v === id, 'does not name this species folder'],
      ['kind', ...nonEmpty],
      ['source_id', ...nonEmpty],
      ['asset_sha256', ...nonEmpty],
      ['kept', (v) => typeof v === 'boolean', 'is not true or false'],
    ]);
    if (reference.source_id && !sourceIds.has(reference.source_id)) {
      fail(at, `source_id ${reference.source_id} is in no sources.json record`);
    }
    if (reference.kept === true) {
      if (!filled(reference.path)) {
        fail(at, 'path missing for a kept image');
      } else if (!reference.path.startsWith('refs/')) {
        fail(at, `path ${reference.path} is not under refs/`);
      } else if (!filled(reference.asset_sha256)) {
        fail(at, 'asset_sha256 missing for a kept image');
      } else {
        const image = join(folder, reference.path);
        if (!existsSync(image)) {
          fail(at, `path ${reference.path} has no file (run git lfs pull)`);
        } else {
          const bytes = createHash('sha256').update(readFileSync(image)).digest('hex');
          if (bytes !== reference.asset_sha256) {
            fail(at, `path ${reference.path} bytes do not match asset_sha256`);
          }
        }
      }
    } else {
      must(at, reference, [
        ['url', ...nonEmpty],
        ['accessed_at', ...nonEmpty],
      ]);
    }
  }
}

function checkSpecies(where, id, doc, profileSha) {
  must(where, doc, [
    ['id', (v) => v === id, 'does not name this species folder'],
    ['scientific_name', ...nonEmpty],
    ['taxon_rank', ...nonEmpty],
    ['context', ...nonEmpty],
    ['profile_id', (v) => v === id, 'does not name this species folder'],
    ['profile_path', (v) => v === 'packet/profile.json', 'does not point at the folder\'s own profile'],
    ['profile_sha256', (v) => v === profileSha, 'does not match packet/profile.json'],
    ['preset', present, ''],
    ['parameters', isObject, 'is not a record of parameters'],
  ]);
}

function checkSpecimens(where, id, doc) {
  must(where, doc, [
    ['schema', (v) => v === 'specimens', 'is not "specimens"'],
    ['schema_version', (v) => v === 1, 'is not 1'],
    ['benchmark_id', present, ''],
    ['cases', ...list],
  ]);
  for (const record of doc.cases ?? []) {
    must(`${where} case ${record.id ?? '(unnamed)'}`, record, [
      ['id', ...nonEmpty],
      ['species_id', (v) => v === id, 'does not name this species folder'],
      ['seed', (v) => Number.isInteger(v), 'is not a whole number'],
      ['seed_role', ...nonEmpty],
      ['parameter_species_id', ...nonEmpty],
    ]);
  }
}

function checkPins(where, id, doc, profileSha) {
  must(where, doc, [
    ['schema', (v) => v === 'pins', 'is not "pins"'],
    ['schema_version', (v) => v === 1, 'is not 1'],
    ['species', (v) => v === id, 'does not name this species folder'],
    ['profile_sha256', (v) => v === profileSha, 'does not match packet/profile.json'],
  ]);
  if (isEmptyStub(doc)) return;
  const whole = (v) => Number.isInteger(v) && v >= 0;
  const triple = (v) => Array.isArray(v) && v.length === 3 && v.every((n) => typeof n === 'number');
  const hash = (v) => typeof v === 'string' && /^\d+$/.test(v);
  must(where, doc, [
    ['seed', whole, 'is not a whole number'],
    ['pins', isObject, 'is not a record of pins'],
    ['leaf_band', (v) => Array.isArray(v) && v.length === 2 && v.every(whole) && v[0] < v[1],
      'is not a low-to-high pair of counts'],
    ['growth_reference', isObject, 'is not a record'],
  ]);
  if (isObject(doc.pins)) {
    must(`${where} pins`, doc.pins, [
      ['wood_vertices', whole, 'is not a whole number'],
      ['wood_triangles', whole, 'is not a whole number'],
      ['instances', whole, 'is not a whole number'],
      ['min', triple, 'is not three numbers'],
      ['max', triple, 'is not three numbers'],
      // A u64 hash does not survive a double, so it is recorded as digits.
      ['skeleton', hash, 'is not a hash written as digits'],
      ['placement', hash, 'is not a hash written as digits'],
      ['element', hash, 'is not a hash written as digits'],
    ]);
  }
  if (isObject(doc.growth_reference)) {
    must(`${where} growth_reference`, doc.growth_reference, [
      ['age', (v) => typeof v === 'number' && v > 0, 'is not a positive number'],
      ['height_m', (v) => typeof v === 'number' && v > 0, 'is not a positive number'],
    ]);
  }
}

function checkStills(where, id, doc) {
  must(where, doc, [
    ['schema', (v) => v === 'stills', 'is not "stills"'],
    ['schema_version', (v) => v === 1, 'is not 1'],
    ['species', (v) => v === id, 'does not name this species folder'],
    ['stills', ...list],
  ]);
  for (const still of doc.stills ?? []) {
    must(`${where} still ${still.path ?? '(unnamed)'}`, still, [
      ['path', ...nonEmpty],
      ['sha256', ...nonEmpty],
      ['preset', ...nonEmpty],
      ['seed', (v) => Number.isInteger(v), 'is not a whole number'],
      ['view', ...nonEmpty],
      ['visual_status', ...nonEmpty],
    ]);
  }
}

/** A pipeline artifact is either an admitted record or the empty stub. */
function checkStubbable(where, doc, schema, fields) {
  if (isEmptyStub(doc)) {
    must(where, doc, [
      ['schema', (v) => v === schema, `is not "${schema}"`],
      ['schema_version', (v) => v === 1, 'is not 1'],
    ]);
    return;
  }
  must(where, doc, fields);
}

function rasters(folder, species) {
  for (const entry of readdirSync(folder, { withFileTypes: true })) {
    const path = join(folder, entry.name);
    if (entry.isDirectory()) {
      if (relative(join(ROOT, CATALOGUE, species), path) === 'refs') continue;
      rasters(path, species);
    } else if (RASTER.test(entry.name)) {
      fail(`${CATALOGUE}/${species}/${relative(join(ROOT, CATALOGUE, species), path)}`,
        'is a raster outside refs/');
    }
  }
}

function run(root = ROOT) {
  failures.length = 0;
  const ids = speciesIds(root);
  if (ids.length === 0) {
    fail(CATALOGUE, 'holds no species folder');
    return failures;
  }

  const attributes = join(root, '.gitattributes');
  if (!existsSync(attributes)) {
    fail('.gitattributes', `missing the rule \`${LFS_RULE}\``);
  } else if (!readFileSync(attributes, 'utf8').includes(LFS_RULE)) {
    fail('.gitattributes', `missing the rule \`${LFS_RULE}\``);
  }

  // A folder that is missing a required file cannot be rendered, so the page
  // comparison below is held back until the missing file is restored; without
  // this the regeneration throws on the absent record and the run dies with a
  // stack trace instead of naming the species and the file.
  let everyFolderComplete = true;

  for (const id of ids) {
    const folder = join(root, CATALOGUE, id);
    const where = `${CATALOGUE}/${id}`;
    let complete = true;
    for (const file of REQUIRED) {
      const path = join(folder, file);
      if (!existsSync(path) || !statSync(path).isFile()) {
        fail(where, `missing ${file}`);
        complete = false;
      }
    }
    if (!complete) {
      everyFolderComplete = false;
      continue;
    }

    const read = (file) => {
      try {
        return JSON.parse(readFileSync(join(folder, file), 'utf8'));
      } catch (error) {
        fail(`${where}/${file}`, `is not readable JSON (${error.message})`);
        return null;
      }
    };
    const docs = Object.fromEntries(
      REQUIRED.filter((file) => file.endsWith('.json')).map((file) => [file, read(file)]),
    );
    if (Object.values(docs).some((doc) => doc === null)) continue;

    const profileSha = createHash('sha256')
      .update(readFileSync(join(folder, 'packet/profile.json')))
      .digest('hex');

    const sourceIds = checkSources(`${where}/sources.json`, id, docs['sources.json']);
    checkProfile(`${where}/packet/profile.json`, id, docs['packet/profile.json']);
    checkReferences(`${where}/packet/references.json`, id, docs['packet/references.json'], folder, sourceIds);
    checkSpecies(`${where}/packet/species.json`, id, docs['packet/species.json'], profileSha);
    checkSpecimens(`${where}/packet/specimens.json`, id, docs['packet/specimens.json']);
    checkPins(`${where}/pins.json`, id, docs['pins.json'], profileSha);
    checkStills(`${where}/stills.json`, id, docs['stills.json']);
    checkStubbable(`${where}/manifest.json`, docs['manifest.json'], 'manifest', [
      ['species', (v) => v === id, 'does not name this species folder'],
      ['taxon', isObject, 'is not a taxon record'],
      ['sources', ...list],
      ['fields', ...list],
      ['model', ...nonEmpty],
    ]);
    checkStubbable(`${where}/provenance.json`, docs['provenance.json'], 'provenance', [
      ['entries', isObject, 'is not a record keyed by JSON Pointer'],
    ]);
    checkStubbable(`${where}/decisions.json`, docs['decisions.json'], 'decisions', [
      ['decisions', ...list],
    ]);
    checkStubbable(`${where}/resolutions.json`, docs['resolutions.json'], 'resolutions', [
      ['resolutions', ...list],
    ]);

    if (readFileSync(join(folder, 'NOTES.md'), 'utf8').trim() === '') {
      fail(`${where}/NOTES.md`, 'holds no note');
    }
    rasters(folder, id);
  }

  if (everyFolderComplete) {
    for (const [path, content] of renderPages(root)) {
      const committed = existsSync(join(root, path)) ? readFileSync(join(root, path), 'utf8') : null;
      if (committed !== content) fail(path, 'differs from regeneration');
    }
  }
  return failures;
}

export { run };

if (process.argv[1] && resolve(process.argv[1]) === resolve(new URL(import.meta.url).pathname)) {
  const found = run();
  if (found.length > 0) {
    for (const message of found) console.error(message);
    console.error(`\ncatalogue: ${found.length} failure${found.length === 1 ? '' : 's'}`);
    process.exit(1);
  }
  console.log(`catalogue: ${speciesIds().length} species pass the structure check`);
}
