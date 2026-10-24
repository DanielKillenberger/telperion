#!/usr/bin/env python3
"""usage: digest.py <binary> [presets]: per preset, a digest of each artifact
line of artifact_digest; the skeleton also with every node identity masked."""
import hashlib, re, subprocess, sys
mask = re.compile(rb'identity: NodeIdentity \{ birth: \d+, key: NodeKey\([^)]*\) \}')
presets = sys.argv[2:] or ['european-beech', 'oregon-white-oak', 'norway-spruce', 'date-palm']
for p in presets:
    out = subprocess.run([sys.argv[1], p], capture_output=True, check=True).stdout
    for line in out.split(b'\n'):
        if not line: continue
        k = line.split(b' ', 1)[0].decode()
        raw = hashlib.sha256(line).hexdigest()[:16]
        masked = hashlib.sha256(mask.sub(b'identity: _', line).replace(b', blooms: []', b'')).hexdigest()[:16]
        print(p, k, 'raw', raw, 'masked', masked, flush=True)
