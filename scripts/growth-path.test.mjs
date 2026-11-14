import { describe, expect, it } from 'vitest';
import { files, imports } from './growth-path.mjs';

/* The growth path's files (fn-366 R2), on scripts written here: the field
 * entry, its static imports followed once each, and the core's module. */
const dist = {
  'field.js': 'import { T as e } from "./core.js";\nimport{s}from"./voxelize.js";\nconst u = new URL("./telperion.wasm", import.meta.url);',
  'core.js': 'import "./module.js";\nexport { w } from "./module.js";\nconst r = () => import("./render.js");',
  'module.js': 'import { x } from "./core.js";',
  'voxelize.js': 'export const s = 1;',
};
const read = async (/** @type {string} */ f) => {
  if (!(f in dist)) throw Error(`read ${f}`);
  return dist[/** @type {keyof typeof dist} */ (f)];
};

describe('growth path', () => {
  it('reads static imports and re-exports, not a dynamic import', () => {
    expect(imports(dist['core.js'])).toEqual(['module.js', 'module.js']);
    expect(imports(dist['field.js'])).toEqual(['core.js', 'voxelize.js']);
  });

  it('follows the entry once through a cycle and ends with the module it names', async () => {
    expect(await files(read)).toEqual(['field.js', 'core.js', 'voxelize.js', 'module.js', 'telperion.wasm']);
  });

  it('fails when no script on the path names the module', async () => {
    const unnamed = async (/** @type {string} */ f) => (f === 'field.js' ? 'export const a = 1;' : read(f));
    await expect(files(unnamed)).rejects.toThrow('no script on the field path names ./telperion.wasm');
  });
});
