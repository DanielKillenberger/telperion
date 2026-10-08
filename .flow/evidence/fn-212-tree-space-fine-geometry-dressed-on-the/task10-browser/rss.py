# Samples the headless Chromium's processes (the playwright profile's browser and
# its descendants) every 0.2 s; prints each process type's peak RSS in MB at exit.
import os, sys, time, signal
peak = {}
def procs():
    out = {}
    for p in os.listdir('/proc'):
        if not p.isdigit(): continue
        try:
            cmd = open(f'/proc/{p}/cmdline','rb').read().split(b'\0')
            ppid = int(open(f'/proc/{p}/stat').read().rsplit(')',1)[1].split()[1])
            out[int(p)] = (ppid, [c.decode(errors='ignore') for c in cmd])
        except Exception: pass
    return out
def kind(cmd):
    for c in cmd:
        if c.startswith('--type='): return c[7:] + ('-'+c for c in cmd if c.startswith('--utility-sub-type=')).__next__() if False else c[7:]
    return 'browser'
def rss(p):
    for l in open(f'/proc/{p}/status'):
        if l.startswith('VmRSS:'): return int(l.split()[1])//1024
    return 0
stop = False
signal.signal(signal.SIGTERM, lambda *a: globals().__setitem__('stop', True))
end = time.time() + float(sys.argv[1])
while not stop and time.time() < end and not os.path.exists(sys.argv[2]):
    ps = procs()
    roots = [p for p,(pp,c) in ps.items() if any('playwright_chromiumdev_profile' in x for x in c) and not any(x.startswith('--type=') for x in c)]
    tree = set(roots); grew = True
    while grew:
        grew = False
        for p,(pp,c) in ps.items():
            if pp in tree and p not in tree: tree.add(p); grew = True
    for p in tree:
        try:
            k = kind(ps[p][1]); peak[k] = max(peak.get(k,0), rss(p))
        except Exception: pass
    time.sleep(0.2)
print(peak)
