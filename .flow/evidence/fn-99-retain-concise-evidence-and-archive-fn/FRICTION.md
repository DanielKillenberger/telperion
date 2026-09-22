# Friction

## 2026-09-21 — Local SSH agent unavailable
Fetching master waited for an SSH agent which then refused signing. Cost: about two minutes. An HTTPS fetch succeeded without changing repository remotes; this run will push via the authenticated HTTPS helper. A working local SSH agent would remove the delay. This is a local setup issue, not a proposed repository spec.
