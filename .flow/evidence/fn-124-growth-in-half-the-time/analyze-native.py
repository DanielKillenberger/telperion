"""Symbolize growth_sampler.rs stacks with inline frames and attribute the
skeleton stage. Usage: analyze-native.py <binary> <samples.txt> [rows] [--json out]
Prints a stage split (every skeleton sample lands in exactly one row), then
inclusive and self shares per source function under pipeline::skeleton.
With line tables only, addr2line names the innermost inline level after the
physical function; its location is right, so a leaf is labelled by location."""
import collections, json, re, subprocess, sys
binary, path = sys.argv[1], sys.argv[2]
rows = int(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3].isdigit() else 40
lines = open(path).read().splitlines()
head = json.loads(lines[0]); stacks = [l.split() for l in lines[1:] if l.strip()]
# Word two is the top of the stack at the interrupt: a return address when
# the leaf had pushed no frame. Kept only where it names a call site that
# the frame-pointer chain skipped.
tops = [s[1] for s in stacks]; stacks = [[s[0]] + s[2:] for s in stacks]
addrs = sorted({a for s in stacks for a in s} | {format(int(t, 16) - 1, 'x') for t in tops if int(t, 16) < 1 << 40 and int(t, 16) > 0})
out = subprocess.run(['addr2line', '-a', '-f', '-i', '-C', '-e', binary], input='\n'.join('0x' + a for a in addrs),
                     capture_output=True, text=True, check=True).stdout.splitlines()
clean = lambda n: re.sub(r'\[[0-9a-f]{16}\]|telperion_core::', '', n)
levels, cur, i = {}, None, 0  # address -> [(name, file, file:line)] innermost first
while i < len(out):
    if out[i].startswith('0x'):
        cur = format(int(out[i], 16), 'x'); levels[cur] = []; i += 1; continue
    loc = out[i + 1].split(' ')[0]
    rel = loc.split('crates/telperion-core/src/')[-1].split('/src/')[-1]
    levels[cur].append((clean(out[i]), re.sub(r':(\d+|\?)$', '', rel), loc.split('/')[-1]))
    i += 2
UNKNOWN = [('??', '??', '??')]
def logical(stack):
    first = levels.get(stack[0], UNKNOWN)
    lead = (first[0][0], first[0][1]) if len(first) == 1 else (f"@{first[0][2]}", first[0][1])
    return [lead] + [(n, f) for n, f, _ in first[1:]] + [(n, f) for a in stack[1:] for n, f, _ in levels.get(a, UNKNOWN)]
is_ = lambda n, short: n == short or n.endswith('::' + short)
def stage(frames):
    names = [n for n, _ in frames]; files = [f for _, f in frames]
    has = lambda short: any(is_(n, short) for n in names)
    if 'branching/local/advance.rs' in files:
        if has('run'):
            if has('admits') or has('rejected'):
                return 'local advance / planner run / envelope admission'
            if has('heading'):
                return 'local advance / planner run / heading and turn limit'
            return 'local advance / planner run / rest (stations, storage, resampling)'
        if has('admits') or has('rejected'):
            return 'local advance / admission outside the planner'
        if has('heading'):
            return 'local advance / twig heading outside the planner'
        if has('validate_range'):
            return 'local advance / validation'
        return 'local advance / loop body (births, laterals, queue)'
    if any(f.startswith('branching/scaffold') for f in files):
        return 'scaffold advance'
    if 'branching/local/seed.rs' in files:
        return 'local seeding'
    if has('solve'):
        return 'radius solve'
    if has('identify_range') or has('identify') or has('remap_after_shedding') or has('remap'):
        return 'identity and remap'
    if has('finish'):
        return 'finish (shedding, tips)'
    if has('new'):
        return 'specimen setup'
    return 'other: ' + names[-1]
def restore(stack, top):
    t = int(top, 16)
    if not 0 < t < 1 << 40:
        return stack
    ret = format(t - 1, 'x')
    if levels.get(ret, UNKNOWN)[0][0] == '??' or ret in stack[1:2]:
        return stack
    return [stack[0], ret] + stack[1:]
stacks = [restore(s, t) for s, t in zip(stacks, tops)]
leaves = collections.defaultdict(collections.Counter)
total = 0; incl = collections.Counter(); excl = collections.Counter(); stages = collections.Counter()
for s in stacks:
    frames = logical(s)
    hit = [k for k, (n, _) in enumerate(frames) if n == 'skeleton' or n.startswith('pipeline::skeleton')]
    if not hit:
        continue
    total += 1
    below = frames[:hit[-1]] or [frames[hit[-1]]]
    excl[below[0][0]] += 1
    for n in {n for n, _ in below}:
        incl[n] += 1
    stages[stage(below[::-1])] += 1
    leaves[stage(below[::-1])][' < '.join(n for n, _ in below[:3])] += 1
ms = head['ms_per_build']
pct = lambda n: 100 * n / total
print(f"{head['preset']} {head['seed']}: {total} skeleton samples over {head['builds']} builds, {ms} ms per build (sampler on)")
print(f"{'share%':>7} {'~ms':>6}  stage")
named = {k: n for k, n in stages.items() if not k.startswith('other')}
lost = collections.Counter({k[7:]: n for k, n in stages.items() if k.startswith('other')})
for k, n in sorted(named.items(), key=lambda kv: -kv[1]):
    print(f"{pct(n):7.1f} {pct(n) * ms / 100:6.2f}  {k}")
print(f"{pct(sum(lost.values())):7.1f} {pct(sum(lost.values())) * ms / 100:6.2f}  stage unknown: a frame was skipped at a prologue; leaves "
      + ', '.join(f'{k} {pct(n):.1f}' for k, n in lost.most_common(4)))
print(f"stage attribution {pct(sum(named.values())):.1f}%; function attribution {100 - pct(excl['??']):.1f}% ('??' is code outside the executable: libc allocator and memcpy)")
print(f"\n{'incl%':>6} {'self%':>6}  function (self of an inlined leaf is labelled @file:line)")
for n, c in incl.most_common(rows):
    print(f"{pct(c):6.1f} {pct(excl[n]):6.1f}  {n[:120]}")
print(f"self rows sum {pct(sum(excl.values())):.1f}%; unresolved '??' self {pct(excl['??']):.1f}%")
if '--leaves' in sys.argv:
    for k in sorted(named, key=lambda k: -named[k])[:4]:
        print(f"\n{k}: " + '; '.join(f"{l} {pct(n):.1f}" for l, n in leaves[k].most_common(8)))
if '--json' in sys.argv:
    json.dump({'fixture': f"{head['preset']}-{head['seed']}", 'samples': total, 'ms_per_build': ms,
               'stages_pct': {k: round(pct(n), 2) for k, n in named.items()},
               'stage_unknown_pct': round(pct(sum(lost.values())), 2), 'unresolved_self_pct': round(pct(excl['??']), 2),
               'functions_incl_pct': {n: round(pct(c), 2) for n, c in incl.most_common(60)}},
              open(sys.argv[sys.argv.index('--json') + 1], 'w'), indent=1)
