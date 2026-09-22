#!/usr/bin/env python3
"""Decode a PNG with stdlib and count non-sky samples on the four edges."""
import json, struct, sys, zlib
from pathlib import Path


def decode(path):
    data = Path(path).read_bytes()
    off = 8
    w = h = col = None
    idat = b""
    while off < len(data):
        ln = struct.unpack(">I", data[off : off + 4])[0]
        typ = data[off + 4 : off + 8]
        chunk = data[off + 8 : off + 8 + ln]
        off += 12 + ln
        if typ == b"IHDR":
            w, h, _bit, col = struct.unpack(">IIBB", chunk[:10])
        elif typ == b"IDAT":
            idat += chunk
        elif typ == b"IEND":
            break
    raw = zlib.decompress(idat)
    bpp = 3 if col == 2 else 4
    stride = 1 + w * bpp

    def paeth(a, b, c):
        p = a + b - c
        return min((a, b, c), key=lambda v: abs(p - v))

    prev = bytearray(w * bpp)
    rows = []
    for y in range(h):
        f = raw[y * stride]
        scan = bytearray(raw[y * stride + 1 : (y + 1) * stride])
        out = bytearray(w * bpp)
        for x in range(len(scan)):
            a = out[x - bpp] if x >= bpp else 0
            b = prev[x]
            c = prev[x - bpp] if x >= bpp else 0
            v = scan[x]
            if f == 1:
                v = (v + a) & 255
            elif f == 2:
                v = (v + b) & 255
            elif f == 3:
                v = (v + ((a + b) // 2)) & 255
            elif f == 4:
                v = (v + paeth(a, b, c)) & 255
            out[x] = v
        prev = out
        rows.append(bytes(out))
    return w, h, bpp, rows


def sky(r, g, b):
    return r > 160 and g > 170 and b > 190 and b >= g and abs(r - g) < 40


def rgb(row, x, bpp):
    i = x * bpp
    return row[i], row[i + 1], row[i + 2]


def count_row(row, w, bpp):
    return sum(1 for x in range(w) if not sky(*rgb(row, x, bpp)))


def count_col(rows, x, h, bpp):
    return sum(1 for y in range(h) if not sky(*rgb(rows[y], x, bpp)))


def inspect(path):
    w, h, bpp, rows = decode(path)
    top = [count_row(rows[y], w, bpp) for y in range(min(3, h))]
    bot = [count_row(rows[h - 1 - y], w, bpp) for y in range(min(3, h))]
    left = [count_col(rows, x, h, bpp) for x in range(min(3, w))]
    right = [count_col(rows, w - 1 - x, h, bpp) for x in range(min(3, w))]
    clipped = top[0] > 0 or left[0] > 0 or right[0] > 0
    # Ground contact on the bottom edge is expected; not a clip.
    return {
        "path": str(path),
        "size": [w, h],
        "top_nonsky": top,
        "bottom_nonsky": bot,
        "left_nonsky": left,
        "right_nonsky": right,
        "framing": "clipped" if clipped else "complete",
    }


if __name__ == "__main__":
    print(json.dumps([inspect(p) for p in sys.argv[1:]], indent=2))
