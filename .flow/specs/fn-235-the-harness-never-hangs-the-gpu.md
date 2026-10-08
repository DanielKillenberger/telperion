# The harness never hangs the GPU

## Goal & Context

On 2026-10-08 the owner could not open the harness (`npm run dev` on the birch branch, fn-213 round 14, which carries F4 up to fn-223): every load took a long time, then reported "the GPU device was lost: A valid external Instance reference no longer exists", and Chrome's GPU process crashed until Chrome blocked WebGPU. The host reproduced it in a separate headless Chrome on the RTX 3080 with the GPU otherwise idle: `?species=silver-birch&age=20` sat at "building..." for 90 s, the page responsive, while the same tree builds natively in well under a second, so GPU work never finished and Chrome's watchdog killed the GPU process. Afterwards no new Chrome on the machine got a WebGPU adapter, even a fresh headless profile on a blank page. Separately, the kernel logged hundreds of NVIDIA "can't alloc VA space for mapping" errors whenever the workers' GPU test runs ran beside the browser. Checked after a clean reboot (host, 2026-10-08 23:45): headless Chrome with the hardware adapter, `?species=silver-birch&age=20` on the birch branch, sat at "building..." for 60 s again. After one such hang every new Chrome, even a fresh profile on a blank page, gets no WebGPU adapter ("A valid external Instance reference no longer exists") until the machine reboots, while native Vulkan keeps working (a GPU test passes in 2.7 s). So each browser repro costs the owner a reboot. Which GPU step hangs, and whether F4 and other species hang too, is unknown. Host-created release blocker (2026-10-08): the harness is the owner's judging tool and the web package is the product.

## Requirements

- **R1 (repro first, natively):** because a browser hang wedges WebGPU until reboot, the hang is found natively: the browser's build path (the wasm entry's device route, `crates/telperion-render/src/web.rs`) run on native wgpu under the browser's device limits and features, on the birch branch's birch at 20, with a timeout around every device wait so a hang fails the test instead of wedging the driver. The step that never completes is named; a standalone test is red on it before any fix. If the native run does not hang, stop and report with what differs from the browser before any browser repro. A browser repro is run at most once per fix, only after the native test is green, and only with the owner's go (it may cost a reboot).
- **R2 (fix):** the cause fixed so a build either finishes or fails by name; no GPU dispatch whose length is unbounded in the browser.
- **R3 (budget):** the browser's device budget (`fine/budget.rs`, 8.5 GiB, measured with the 10 GiB card otherwise empty) no longer assumes the card is the page's alone; a build over what the device can hold fails by name, never by device loss.
- **R4 (tests beside the browser):** GPU tests that push the device to its limits (out-of-memory probes) do not run on the shared lock while a harness session is open, or are bounded so they cannot exhaust the driver's address space; stated in docs.
- **R5:** the owner opens beech, birch and oak at 40 and 80 in the harness without a device loss; gate and npm test green.

## Boundaries

Harness and browser GPU path only. Release blocker for fn-198.
