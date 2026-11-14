"""fn-351 speed round 2: a flat sampling profile of one single-threaded
command, since neither `perf` nor a working `samply` is on the desk
(FRICTION.md, 14 Nov 13:12): the kernel samples the command's instruction
pointer on its cycles counter (`perf_event_open`, one small ring buffer,
user space only), and `llvm-symbolizer` names each address, inlined frames
included, from each mapped object's symbols and line tables.

    python3 sample.py <out.tsv> <binary> [args...]

With `SAMPLE_UNTIL=<text>` in the environment the sampling stops once the
command prints that text (within the 20 ms the loop sleeps): `early` prints
`cpu:` when the tree is grown, before it formats every phytomer for its
digest, which is no part of the growth.

Writes one line a sampled address: `count <tab> innermost <tab> ... <tab>
outermost <tab> [object]`; read it back with sample_report.py.
"""
import collections
import ctypes
import fcntl
import mmap
import os
import struct
import subprocess
import sys
import time

FREQ = 5000
PAGES = 64
# disabled, exclude_kernel, exclude_hv, freq, enable_on_exec.
FLAGS = 1 | (1 << 5) | (1 << 6) | (1 << 10) | (1 << 12)
SYS_PERF_EVENT_OPEN = 298  # x86-64
PERF_RECORD_SAMPLE = 9
PERF_RECORD_LOST = 2

out, command = sys.argv[1], sys.argv[2:]
libc = ctypes.CDLL(None, use_errno=True)
gate_r, gate_w = os.pipe()
said_r, said_w = os.pipe()
until = os.environ.get("SAMPLE_UNTIL", "").encode()
pid = os.fork()
if pid == 0:
    os.close(gate_w)
    os.read(gate_r, 1)
    os.dup2(said_w, 1)
    os.execvp(command[0], command)
os.close(gate_r)
os.close(said_w)
os.set_blocking(said_r, False)
attr = bytearray(128)
# type HARDWARE, size, config CPU_CYCLES, sample_freq, sample_type IP.
struct.pack_into("IIQQQ", attr, 0, 0, 128, 0, FREQ, 1)
struct.pack_into("Q", attr, 40, FLAGS)
fd = libc.syscall(
    SYS_PERF_EVENT_OPEN, (ctypes.c_char * 128).from_buffer(attr), pid, -1, -1, 0
)
if fd < 0:
    sys.exit(f"perf_event_open: errno {ctypes.get_errno()}")
page = mmap.PAGESIZE
ring = mmap.mmap(fd, (1 + PAGES) * page, mmap.MAP_SHARED, mmap.PROT_READ | mmap.PROT_WRITE)
size = PAGES * page
counts = collections.Counter()
lost = 0


def at(offset, length):
    """`length` bytes of the ring's data from `offset`, around its end."""
    offset %= size
    first = bytes(ring[page + offset : page + min(offset + length, size)])
    return first + bytes(ring[page : page + length - len(first)])


def drain():
    global lost
    head = struct.unpack_from("Q", ring, 1024)[0]
    tail = struct.unpack_from("Q", ring, 1032)[0]
    while tail < head:
        kind, _, length = struct.unpack("IHH", at(tail, 8))
        if kind == PERF_RECORD_SAMPLE:
            counts[struct.unpack("Q", at(tail + 8, 8))[0]] += 1
        elif kind == PERF_RECORD_LOST:
            lost += struct.unpack("QQ", at(tail + 8, 16))[1]
        tail += length
    struct.pack_into("Q", ring, 1032, tail)


def mappings():
    """The command's mapped objects: (start, end, base, path), the base
    where the object's first mapping begins."""
    rows, bases = [], {}
    try:
        text = open(f"/proc/{pid}/maps").read()
    except OSError:
        return []
    for line in text.splitlines():
        parts = line.split()
        if len(parts) < 6 or not parts[5].startswith("/"):
            continue
        start, end = (int(x, 16) for x in parts[0].split("-"))
        bases.setdefault(parts[5], start)
        rows.append((start, end, parts[5]))
    return [(s, e, bases[p], p) for s, e, p in rows]


def heard():
    """The command's output passed on; whether it said `until`."""
    try:
        said = os.read(said_r, 1 << 16)
    except BlockingIOError:
        return False
    sys.stdout.buffer.write(said)
    sys.stdout.flush()
    return bool(until) and until in said


os.write(gate_w, b"x")
os.close(gate_w)
maps = []
while True:
    done, status = os.waitpid(pid, os.WNOHANG)
    if heard():
        fcntl.ioctl(fd, 0x2401, 0)  # PERF_EVENT_IOC_DISABLE
    drain()
    if done:
        break
    maps = mappings() or maps
    time.sleep(0.02)
drain()
while heard():
    pass

by_object = collections.defaultdict(dict)
unmapped = 0
for ip, n in counts.items():
    for start, end, base, path in maps:
        if start <= ip < end:
            by_object[path][ip - base] = n
            break
    else:
        unmapped += n
total = sum(counts.values())
print(f"SAMPLES {total} at {FREQ} Hz, {unmapped} unmapped, {lost} lost")
with open(out, "w") as f:
    for path, addresses in by_object.items():
        asked = "\n".join(hex(a) for a in addresses) + "\n"
        named = subprocess.run(
            ["llvm-symbolizer", "--obj=" + path, "--functions=linkage", "--demangle", "--inlines"],
            input=asked, capture_output=True, text=True,
        ).stdout
        groups = [g for g in named.split("\n\n") if g.strip()]
        for n, group in zip(addresses.values(), groups):
            frames = group.strip().split("\n")[0::2]
            f.write("\t".join([str(n)] + frames + [f"[{os.path.basename(path)}]"]) + "\n")
    if unmapped:
        f.write(f"{unmapped}\t[unmapped]\n")
sys.exit(os.waitstatus_to_exitcode(status))
