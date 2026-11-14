"""fn-351 speed round 2: a command's counted cost, read from the kernel's
counters (`perf_event_open`, user space only, children inherited), since
`perf` is not installed and CPU seconds spread a quarter on a loaded desk
(fn-368 FRICTION.md, 03:18). Prints the command's own output, then one line:

    COUNT instructions <n> cycles <n> cache_misses <n> llc_load_misses <n> wall <s>

    python3 count.py <command...>
"""
import ctypes
import os
import struct
import sys
import time

PERF_TYPE_HARDWARE, PERF_TYPE_HW_CACHE = 0, 3
EVENTS = [
    ("instructions", PERF_TYPE_HARDWARE, 1),
    ("cycles", PERF_TYPE_HARDWARE, 0),
    ("cache_misses", PERF_TYPE_HARDWARE, 3),
    # LL cache, read, miss: 2 | 0 << 8 | 1 << 16.
    ("llc_load_misses", PERF_TYPE_HW_CACHE, 2 | (1 << 16)),
]
# disabled, inherit, exclude_kernel, exclude_hv, enable_on_exec.
FLAGS = 1 | (1 << 1) | (1 << 5) | (1 << 6) | (1 << 12)
SYS_PERF_EVENT_OPEN = 298  # x86-64

libc = ctypes.CDLL(None, use_errno=True)


def open_counter(kind, config):
    attr = bytearray(128)
    struct.pack_into("IIQ", attr, 0, kind, 128, config)
    struct.pack_into("Q", attr, 40, FLAGS)
    buf = (ctypes.c_char * 128).from_buffer(attr)
    fd = libc.syscall(SYS_PERF_EVENT_OPEN, buf, 0, -1, -1, 0)
    return fd if fd >= 0 else None


read, write = os.pipe()
pid = os.fork()
if pid == 0:
    os.close(write)
    os.read(read, 1)
    os.execvp(sys.argv[1], sys.argv[1:])
os.close(read)
# The counters are opened on this process and inherited by nothing: open
# them on the child instead, before it execs.
fds = []
for name, kind, config in EVENTS:
    attr = bytearray(128)
    struct.pack_into("IIQ", attr, 0, kind, 128, config)
    struct.pack_into("Q", attr, 40, FLAGS)
    buf = (ctypes.c_char * 128).from_buffer(attr)
    fd = libc.syscall(SYS_PERF_EVENT_OPEN, buf, pid, -1, -1, 0)
    fds.append((name, fd))
started = time.monotonic()
os.write(write, b"x")
os.close(write)
_, status = os.waitpid(pid, 0)
wall = time.monotonic() - started
parts = []
for name, fd in fds:
    if fd < 0:
        parts.append(f"{name} none")
        continue
    parts.append(f"{name} {struct.unpack('Q', os.read(fd, 8))[0]}")
print("COUNT " + " ".join(parts) + f" wall {wall:.3f}")
sys.exit(os.waitstatus_to_exitcode(status))
