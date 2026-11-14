---
title: "A device budget is checked before allocation, charging buffers let go"
date: "2026-10-12"
track: bug
category: runtime-errors
module: crates/telperion-render/src/buffer.rs
tags: [budget, device-memory, webgpu, ledger]
problem_type: runtime-error
symptoms: Light-reading species admitted beside the drawn tree with no budget held; GPU hangs near a full card
root_cause: No prediction read as fits; checks after moves; let-go buffers uncharged while their encoder was open
resolution_type: fix
---

## Problem
Moving every species' build to the device exposed the browser's device budget. Admission predicted a build's peak from `expected_kept`, which is none for every species that reads light (the oak, the ordinary), and it took "no prediction" as "fits beside the drawn tree". So a multi-gigabyte device build could start beside a drawn tree with nothing holding it to the page's 6 GiB, and near a full card such builds hung the GPU (Xid 109) rather than being refused.

## What Didn't Work
- Checking the bytes held after each move: by then the move's old tables were dropped, so its peak (old and new together) was never seen.
- Dropping an `Owned` subtracted its bytes from the ledger at once, although the buffer is destroyed only when the open copy encoder closes, so even a check before allocation undercounted a move.

## Solution
- **Admission** (`resident/budget.rs`): with no expectation, a build makes room first wherever a tree is drawn, and is refused where the page already holds its budget.
- **The build** carries the budget as a cap on the ledger (`Ledger::capped`, lifted when the build ends). `io::buffer` refuses by name any buffer that would pass it beside every byte held, before it is made.
- **The ledger** subtracts a buffer's bytes when it destroys it (`Ledger::destroy`), not when it is let go.

## Prevention
- A budget is checked before an allocation, against every byte still alive on the device, including buffers waiting on an open encoder. A check after the fact misses transient peaks.
- An unavailable prediction never means "fits".
- Tests: `a_buffer_let_go_counts_against_the_cap_until_it_is_destroyed`, `a_build_with_no_expectation_is_held_to_the_budget_as_it_grows`.
