"""R4 medians: native stage timings (warm samples) and the binding's timings,
base against candidate, with each median's change in percent."""
import json
import statistics
import sys

OUT = '/tmp/claude-1000/-home-daniel-Projects-telperion/464c173f-e1a8-46dd-b41b-cc247c8ef785/scratchpad/out/'


def rows(path):
    return [json.loads(l) for l in open(OUT + path) if l.strip()]


def native():
    table = {}
    for side in ('base', 'cand'):
        for r in rows(f'r4{sys.argv[2] if len(sys.argv) > 2 else ""}-{side}.jsonl'):
            if r.get('event') == 'rss':
                table.setdefault(r['preset'], {}).setdefault(side, {}).setdefault('rss_kb', []).append(r['kb'])
            if r.get('event') != 'sample' or r['cold_process_first_build']:
                continue
            ms = r['milliseconds']
            if side == 'base':
                stages = {'growth': ms['growth'], 'surface': ms['surface'],
                          'placement': ms['placement_including_attachment'],
                          'cull': ms['cull'] + ms['foliage_bounds'], 'total': ms['total']}
            else:
                stages = {'growth': ms['growth'], 'surface': ms['surface'],
                          'placement': ms['rings'] + ms['placement'],
                          'cull': ms['cull'], 'total': ms['total']}
            cell = table.setdefault(r['preset'], {}).setdefault(side, {})
            cell.setdefault('hash', set()).add(r['output_fnv1a64'])
            for k, v in stages.items():
                cell.setdefault(k, []).append(v)
    print('native (median of 5 warm samples, ms; peak RSS MB)')
    for preset, sides in table.items():
        b, c = sides['base'], sides['cand']
        same = 'same bytes' if b['hash'] == c['hash'] and len(b['hash']) == 1 else f"HASH {b['hash']} {c['hash']}"
        parts = []
        for k in ('growth', 'surface', 'placement', 'cull', 'total'):
            mb, mc = statistics.median(b[k]), statistics.median(c[k])
            parts.append(f"{k} {mb:.0f}->{mc:.0f} ({(mc / mb - 1) * 100:+.1f}%)")
        rss = f"rss median {statistics.median(b['rss_kb']) / 1024:.0f}->{statistics.median(c['rss_kb']) / 1024:.0f} MB"
        print(f"{preset}: {'; '.join(parts)}; {rss}; {same}")


def wasm():
    table = {}
    for side in ('base', 'cand'):
        for r in rows(f'r4w-{side}.jsonl'):
            cell = table.setdefault((r['id'], r['outputs']), {}).setdefault(side, {})
            cell.setdefault('hash', set()).add(r['hash'])
            for k, v in r['timings'].items():
                cell.setdefault(k, []).append(v)
    print('wasm binding (median of 5, ms)')
    for (preset, outputs), sides in table.items():
        b, c = sides['base'], sides['cand']
        parts = []
        for k in ('growthMs', 'surfaceMs', 'planMs', 'foliageMs', 'fieldMs', 'coreMs'):
            mb, mc = statistics.median(b[k]), statistics.median(c[k])
            if mb == 0 and mc == 0:
                continue
            parts.append(f"{k[:-2]} {mb:.0f}->{mc:.0f} ({(mc / mb - 1) * 100 if mb else 0:+.1f}%)")
        same = 'same bytes' if b['hash'] == c['hash'] else 'HASH DIFF'
        print(f"{preset} {outputs}: {'; '.join(parts)}; {same}")


if __name__ == '__main__':
    {'native': native, 'wasm': wasm}[sys.argv[1]]()
