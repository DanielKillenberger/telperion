# fn-264 friction

## 2026-10-12, worker: flowctl not on PATH

- **Doing:** reading the spec with `flowctl cat` from F4.
- **Slowed:** `flowctl: command not found` in the dispatched shell; found by searching the plugin cache.
- **Cost:** about 1 minute.
- **Would remove it:** the dispatch naming the flowctl path, or the plugin's `scripts/` on the agent shell's PATH.
