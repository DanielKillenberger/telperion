# Growth off one core in the browser

## Goal & Context

fn-212 task 10 put fine geometry on the device in production, natively and in the browser: spruce 80 from a parameter change to a drawn tree went from about 9-11 s to 1.5-2.6 s natively, beech 40 to 80 in the harness from 5.7 s to 1.4 s. But in the browser the spruce 40 to 70 still takes 14.2 s, about 8.9 s of it growth on a single core (settle takes one thread on wasm, D6). The owner's goal: adjusting parameters in the harness yields near-instant results. fn-209 built and proved a GPU executor that grows fine roots identically to the CPU reference (spruce 80 growth 0.37 s against 1.1 s natively), but it is test-only.

## Requirements

- **R1 (measure first):** where the browser's 8.9 s goes (structure above the cut, fine roots, settle phases), by the in-page performance marks; the same build natively on one thread for comparison.
- **R2:** fine roots grown on the device in production, natively and in the browser, through fn-209's GPU executor and fn-212's async `Roots` (one driver), for every family the device settle accepts; the oak and palm keep the CPU with the reason named.
- **R3:** the browser's spruce 40 to 70 and to 80, beech 40 to 80, from a parameter change to a drawn frame, before and after; identical trees to the CPU reference by digest; the memory budget of fn-212 task 10 holds.
- **R4:** the gate, npm test.

## Boundaries

Wasm threads (SharedArrayBuffer, cross-origin isolation) are out unless R1 shows the device cannot carry the cost; then it is a host decision. Depends on fn-212 and fn-211 landing on F4.