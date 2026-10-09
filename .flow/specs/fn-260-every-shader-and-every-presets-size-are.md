# Every shader and every preset's size are checked without a device

## Goal & Context

fn-248's FRICTION.md (2026-10-09): two WGSL locals shadowed builtins (`floor` in `dress.wgsl`, `length` in `stations.wgsl`); naga reports that only when the shader is built on the GPU, so each cost a full GPU run. And Telperion's age was set from `expected_kept`, a mean over seeds, while the preset grows at seed 42 and kept 4.30M against the browser's 4M ceiling, which only `npm test` caught, after the gate, costing a round of about 20 minutes. Both are obvious fixes whose cause the entry names and which change no product behaviour (AGENTS.md, "Friction reports"; host spec, 2026-10-09). fn-254 to fn-256 rewrite every growth shader, so the first check saves the most there.

## Requirements

- **R1:** an ordinary test (no device) parses and validates every shipped WGSL module through naga, as the device would build it (the same composition, defines and entry points); a deliberately shadowed builtin in a fixture fails it.
- **R2:** an ordinary cargo test grows each shipped preset at its own preset seed and age and checks its kept phytomers against the browser's ceiling (`WASM_KEPT`), with the margin printed; it fails before `npm test` would.
- **R3:** both run inside the workspace gate's time budget (measured); gate and npm test green.

## Boundaries

Checks only; no shader or preset changes.
