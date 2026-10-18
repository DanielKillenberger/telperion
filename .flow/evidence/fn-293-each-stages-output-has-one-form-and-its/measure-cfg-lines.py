#!/usr/bin/env python3
"""Lines of one crate's src/ that compile into its non-test build (fn-293 R5).

measure-lines.sh counts files by name, so it still counts a module gated
whole by `#[cfg(test)]` and every `#[cfg(test)]` item inside a file. This
walks the module tree from lib.rs, skips a `mod` declared under
`#[cfg(test)]` (or `cfg(all(test, ...))`), and drops each item so gated:
the attribute, through the end of the item (its matching brace, or its
semicolon). Comments and doc comments are counted as the script counts them.
Usage: measure-cfg-lines.py <crate-dir>
"""
import re
import sys
from pathlib import Path

GATE = re.compile(r"^\s*#\[cfg\((test|all\(test\b[^\]]*)\)\]\s*$")
MOD = re.compile(r"^\s*(pub(\([^)]*\))?\s+)?mod\s+(\w+)\s*;")
PATH = re.compile(r'^\s*#\[path\s*=\s*"([^"]+)"\]')


def item_end(lines, i):
    """Index past the item that starts at or after line i."""
    depth, opened = 0, False
    while i < len(lines):
        code = lines[i].split("//", 1)[0].rstrip()
        for ch in code:
            if ch == "{":
                depth += 1
                opened = True
            elif ch == "}":
                depth -= 1
        i += 1
        if depth == 0 and (opened or code.endswith(";")):
            return i
    return i


def count(path: Path, mod_dir: Path) -> int:
    lines = path.read_text().splitlines()
    total, i = 0, 0
    while i < len(lines):
        line = lines[i]
        if GATE.match(line):
            j = i + 1
            while j < len(lines) and lines[j].lstrip().startswith(("#[", "///")):
                j += 1
            i = item_end(lines, j)
            continue
        m = MOD.match(line)
        if m:
            name = m.group(3)
            prev = lines[i - 1] if i else ""
            custom = PATH.match(prev)
            if custom:
                child = (path.parent / custom.group(1)).resolve()
                total += count(child, child.parent / child.stem)
            else:
                for child in (mod_dir / f"{name}.rs", mod_dir / name / "mod.rs"):
                    if child.exists():
                        total += count(child, mod_dir / name)
                        break
        total += 1
        i += 1
    return total


def main():
    src = Path(sys.argv[1]) / "src"
    print(count(src / "lib.rs", src))


if __name__ == "__main__":
    main()
