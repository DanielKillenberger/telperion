import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

/* release.yml carries two triggers (fn-313): a green Tests run publishes the
 * release under latest, and a dispatch publishes a prerelease under next.
 * Each job's `if` must admit only its own event, so a dispatch never starts
 * the publish job and a Tests run never starts the prerelease. */
const text = readFileSync(new URL('../.github/workflows/release.yml', import.meta.url), 'utf8');
const lines = text.split('\n');

/** The top-level keys of a block opened at `start`, at the given indent. */
function keys(start, indent) {
  const found = [];
  for (let i = start + 1; i < lines.length; i++) {
    const line = lines[i];
    if (line.trim() === '' || line.trimStart().startsWith('#')) continue;
    const depth = line.length - line.trimStart().length;
    if (depth < indent) break;
    const m = depth === indent && line.match(/^\s*([\w-]+):/);
    if (m) found.push({ name: m[1], line: i });
  }
  return found;
}

/** A job's `if` condition, inline or folded over the following lines. */
function condition(job) {
  const field = keys(job.line, 4).find(k => k.name === 'if');
  if (!field) return '';
  const inline = lines[field.line].replace(/^\s*if:\s*/, '');
  if (inline !== '>-') return inline;
  const body = [];
  for (let i = field.line + 1; i < lines.length && /^\s{6}/.test(lines[i]); i++) body.push(lines[i].trim());
  return body.join(' ');
}

const top = keys(-1, 0);
const on = keys(top.find(k => k.name === 'on').line, 2).map(k => k.name);
const jobs = Object.fromEntries(keys(top.find(k => k.name === 'jobs').line, 2).map(j => [j.name, condition(j)]));

describe('release.yml triggers', () => {
  it('runs on a Tests run and on a dispatch only', () => {
    expect(on.sort()).toEqual(['workflow_dispatch', 'workflow_run']);
  });

  it('holds only the publish and prerelease jobs', () => {
    expect(Object.keys(jobs).sort()).toEqual(['prerelease', 'publish']);
  });

  it('starts publish only from a successful Tests run, never from a dispatch', () => {
    // A dispatch's event has no workflow_run, so this conjunct is false.
    expect(jobs.publish).toContain("github.event.workflow_run.conclusion == 'success' &&");
    expect(jobs.publish).not.toContain('||');
    expect(jobs.publish).not.toContain('workflow_dispatch');
  });

  it('starts prerelease only from a dispatch', () => {
    expect(jobs.prerelease).toMatch(/^github\.event_name == 'workflow_dispatch' &&/);
    expect(jobs.prerelease).not.toContain('||');
  });

  it('never names latest as a dist-tag', () => {
    expect(text).not.toMatch(/--tag\s+["']?latest/);
    expect(text).toMatch(/^\s+DIST_TAG: next$/m);
  });
});
