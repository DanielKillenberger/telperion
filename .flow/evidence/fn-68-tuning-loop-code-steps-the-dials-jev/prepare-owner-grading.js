// Offline only: anonymized text agreement study, not visual correctness or role qualification.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';
import cp from 'node:child_process';
const root = import.meta.dirname;
const canon = x => JSON.stringify(Array.isArray(x) ? x.map(sort) : sort(x));
function sort(x) { return Array.isArray(x) ? x.map(sort) : x && typeof x === 'object' ? Object.fromEntries(Object.keys(x).sort().map(k => [k, sort(x[k])])) : x; }
const hash = b => crypto.createHash('sha256').update(b).digest('hex');
function save(name, value) {
  const dest = path.join(root, name), content = JSON.stringify(value, null, 2) + '\n';
  if (fs.existsSync(dest)) {
    const old = fs.readFileSync(dest, 'utf8');
    if (old !== content) cp.execFileSync('apply_patch', [], { input: '*** Begin Patch\n*** Update File: ' + dest + '\n@@\n' + old.trimEnd().split('\n').map(x => '-' + x).join('\n') + '\n' + content.trimEnd().split('\n').map(x => '+' + x).join('\n') + '\n*** End Patch' });
    return;
  }
  cp.execFileSync('apply_patch', [], { input: '*** Begin Patch\n*** Add File: ' + dest + '\n' + content.trimEnd().split('\n').map(x => '+' + x).join('\n') + '\n*** End Patch' });
}
const sources = [
  ['Fable medium', 'joint-blind-fable-journal.json'],
  ['Astra high', 'joint-blind-astra-high-result.json'],
  ['Sol medium', 'joint-blind-result.json'],
  ['Grok high', 'joint-blind-grok-journal.json'],
  ['Astra medium', 'joint-blind-astra-result.json'],
  ['Opus high', 'joint-blind-opus-journal.json'],
];
const verdicts = {}, mapping = {};
sources.forEach(([label, file], i) => {
  const raw = fs.readFileSync(path.join(root, file)), doc = JSON.parse(raw), a = doc.answer || doc.assessment;
  const id = 'V' + (i + 1);
  // Keep every assessment sentence; remove transport, model, image paths, and metadata.
  verdicts[id] = { statuses: a.passes || a.cells.map(c => c[1]), defects: a.defects, observations: a.observations || doc.observations, findings: a.findings };
  mapping[id] = { label, source: file, source_sha256: hash(raw) };
});
const state = {
  owner_verbatim: [
    "i strongly disagree with accepting the crown shape. It's vastly different and the droop also seems absolutely key.",
    "I think we want to identify the BIGGEST gaps that allows to make a believable generation of this tree. ANd for the beech we have clear misalignment of the crown and drooping with leaves that doesn't make it believable.\nI'm sure there's more but i think if we get that and material tuned we're already pretty close.",
    "what was its critiques? because i didn't accept thinking it looked photorealistic. That shouldn't be the goal. Not sure what a reasonable acceptance gate is. I guess accept it if it's recognizable at the quality that was set by previous species"
  ],
  interpretation_not_quote: 'Crown morphology and hanging leafy form are primary believability gaps; material matters next. Agreement concerns the expressed assessment, not merely issuing FAIL. Do not require any particular physical leaf-weight explanation. This text-only study cannot establish image truth, reviewer generalization, or correctness of causal hypotheses.',
  verdicts
};
const criteria = {
  crown: ['Accepts the disputed crown morphology as adequate or harmless variation.', 'Does not resolve the crown mismatch, or gives conflicting acceptance and criticism.', 'Identifies a crown morphology mismatch but treats it as secondary or optional.', 'Treats mismatched crown morphology as a major gap preventing believable reference character.'],
  hanging: ['Accepts absent or limited hanging leafy form as adequate or harmless variation.', 'Does not identify hanging leafy form as a mismatch, or leaves conflicting assessments unresolved.', 'Identifies missing hanging leafy form but treats it as secondary or optional.', 'Treats missing hanging leafy form as a major gap preventing believable reference character.'],
  material: ['Accepts the disputed material appearance as adequate with no meaningful refinement needed.', 'Does not identify material mismatch, or gives unresolved conflicting assessments.', 'Identifies material mismatch but treats correction as optional refinement.', 'Identifies material mismatch as a needed correction for believable reference character.'],
  standard: ['Requires photographic matching or rejects the established catalogue quality standard.', 'The acceptance standard is absent or contradicted within the verdict.', 'Uses recognizability and catalogue quality but relaxes defining reference-character differences as mere non-photorealism.', 'Uses believable reference character at catalogue quality, distinguishing defining mismatches from optional photographic detail.']
};
const questions = {};
for (const id of Object.keys(verdicts)) for (const dimension of Object.keys(criteria)) {
  questions[id + '_' + dimension] = { type: 'score', instructions: `How does the ${dimension} assessment in \`verdicts.${id}\` agree with \`owner_verbatim\`? Apply \`interpretation_not_quote\`. Judge this dimension only, using the full verdict including contradictions; do not reward verbosity or an overall FAIL alone.`, criteria: criteria[dimension] };
}
const serializedState = canon(state), serializedQuestions = canon(questions);
if (/claude|grok|astra|fable|opus|gpt-|tokens|host.grade/i.test(serializedState)) throw Error('Identity/grade leak');
if (Object.keys(questions).length !== 24 || Object.values(verdicts).some(v => !v.defects || !v.observations || !v.findings)) throw Error('Missing assessment data');
save('owner-grading-state.json', state); save('owner-grading-questions.json', questions); save('owner-grading-map.json', mapping);
save('owner-grading-preflight.json', {
  status: 'OFFLINE_HOST_REVIEW_REQUIRED', prior_actual_tokens: 464835, cumulative_cap: 520000, max_calls: 1,
  state_bytes: Buffer.byteLength(serializedState), questions_bytes: Buffer.byteLength(serializedQuestions),
  reservation: Buffer.byteLength(serializedState) + Buffer.byteLength(serializedQuestions) + 1024,
  state_sha256: hash(serializedState), questions_sha256: hash(serializedQuestions),
  ranking: 'Primary mean=(crown+hanging)/2; secondary material and acceptance-standard shown independently. Sort by primary mean, then secondary mean only for exact ties. No pass cutoff, no automatic role qualification. Report all distributions/confidences; near scores are not proven differences.',
  disclosure: 'Single known development specimen, text agreement with owner, not image re-evaluation. Fixed shuffled anonymous order; mapping excluded from request. No owner causal hypothesis required.',
  invocation: 'bash -ic: /tmp/fn68-owner-grading <evidence directory>; same shared caller with single-shot Transport',
  accounting: 'Persist reservation before one dispatch; settle immutable shared caller ledger input+output usage and resolved model; unknown/error retains reservation and stops. Evidence-only Once transport blocks any second HTTP send; no retries.',
  docs: ['https://docs.typesafe.ai/primitives/score.md', 'https://docs.typesafe.ai/patterns/composite-scoring.md', 'https://docs.typesafe.ai/confidence.md', 'https://docs.typesafe.ai/api.md']
});
console.log(JSON.stringify({ state_bytes: Buffer.byteLength(serializedState), questions_bytes: Buffer.byteLength(serializedQuestions), reservation: Buffer.byteLength(serializedState) + Buffer.byteLength(serializedQuestions) + 1024 }));
