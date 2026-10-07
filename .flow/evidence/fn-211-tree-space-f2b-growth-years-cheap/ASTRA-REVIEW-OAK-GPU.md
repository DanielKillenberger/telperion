**Verdict: A is sound with a complete arithmetic contract, but it is not the only exact design. Keep C now. Investigate a CPU-owned feedback loop before committing to A. Declining D is right under the chosen identity requirement.**

Here, “identity” means identical botanical identities and decision state, with final GPU geometry within the agreed screen. Bit-identical final geometry would require a larger contract.

1. **A must cover everything that can feed a future decision.**

The dependency identified in TASK5-GPU.md is real. Light makes rough geometry part of the decision machinery. Astra’s accepted review already includes decision-relevant positions, allocation and presences in canonical arithmetic. The task’s shorter arithmetic list is incomplete.

The contract needs to specify these parts together:

| Part | Required coverage |
|---|---|
| Positions and frames | Coordinate units and ranges; parameter conversion; attachment and relay interpolation; heading, side, roll and rank; normalization, rotations, trig, tropisms and degenerate-vector handling. Rounding only the resulting position is insufficient. |
| Re-lay and sag | Scale, girth, retained pipes, leaf-light mean, radii, mass, signed moments, lever gathering, bending, and which unbent frame the bent pass reads. Include every value scattered back into growth state. |
| Light deposits | Exact leaf positions and areas; lattice origin, spacing, extent and negative-coordinate flooring; corner weights and their products; density conversion and accumulation. |
| Light sweep | Sky directions and weights, ray offsets, bilinear interpolation, optical-depth recurrence, attenuation, direction combination, boundary conditions and zero extinction. Constants may be computed once on the CPU and supplied identically. |
| Bud sampling | Identical cell indices, interpolation weights, outside-field values and integer light representation. No intervening f32 calculation may alter these. |
| Wood and balance | Internode lengths and scales, accumulated wood, upkeep, collected light, reduction order, remembered balance, hazard, threshold and retained presence. |
| Allocation and sizes | Surviving demand, λ splits, incoming vigour, `(v/p)^ψ`, both per-PA normalizers, final sizes and all zero-denominator cases. These values change subsequent positions and wood. |

For every operation, specify widths, signedness, intermediate precision, rounding, division, clamps, transcendental algorithms and overflow behavior. Existing Q32 presences and fixed-point thresholds remain part of that contract.

**Order-free sums require identical terms.** Today’s `Sum` converts already-computed floating-point terms to integers. That fixes regrouping on the CPU; it does not make independently computed f32 and f64 terms equal. Also, signed saturating addition is generally order-dependent when overflow occurs. Sag moments need sufficient range or explicit overflow handling, not an unqualified reuse of that argument.

Keep Astra’s four phase boundaries, same-cycle waking semantics and re-lay schedule. Buffer retries must never duplicate deposits or growth.

Then identity has an inductive argument: identical starting state produces identical phase inputs, integer results, decisions and next-cycle state, regardless of placement. Tests verify the implementations of that argument.

The final lay may retain approximate geometry only where its outputs cannot return to growth or decide botanical identity. Preserve the ground-placement contract and exact attachment requirements. Fixed point guarantees agreement between executors; **it does not establish that movement from today’s tree is visually negligible**. Measure that separately under Generator evolution.

2. **A smaller exact hybrid exists, with a substantial exchange cost.**

The CPU can own the complete rough layout and feedback calculation, including **fine leaves and fine bud positions**, while the GPU executes integer growth decisions and constructs growth records.

Two sampling boundaries are possible:

- The CPU samples each bud and supplies its canonical integer light.
- The CPU supplies exact integer cell indices and interpolation weights, plus a canonical field; the GPU performs the integer sampling.

Supplying only an integer field while letting the GPU sample at f32 positions does not suffice.

Nor does exact light alone close the loop. Balance, upkeep, sizes and mechanics must either remain CPU-owned or use shared canonical arithmetic. For minimum porting, the CPU could supply exact decision thresholds and presences, retain authoritative sizes and geometry, and receive the GPU’s new botanical records. Supplying thresholds directly can preserve existing CPU calculations where introducing quantized light would change them.

This is one algorithm with stages placed on different processors, provided it uses the existing passes and one authoritative state. It must not grow a second CPU tree merely to verify the GPU’s tree.

However, this hybrid **revisits an explicit host decision**. FINE-CONTRACT decisions 5 and 6 forbid production per-node readback. CPU rough layout needs new growth records each cycle and enough retained fine structure for re-lays. Per-bud light or sampling records travel down. That replaces the present per-root exchange with traffic proportional to active buds and new phytomers.

The smaller design is therefore smaller in arithmetic implementation, not necessarily faster or smaller in memory.

**Coarse quantization with a proven margin is conditionally sound.** For a light quantizer \(Q\), an approximate value \(\hat L\), and a rigorous error bound \(\epsilon\), a sufficient condition is:

\[
Q([\hat L-\epsilon,\hat L+\epsilon])=\{q\}.
\]

Both executions then obtain the same integer light. But the bound must include deposits, accumulated field error, positions, re-lays and earlier feedback. The reported 25 µm observation is not such a bound.

A margin around today’s survival decision alone is insufficient. Different thresholds can produce the same immediate decision but different Q32 presences, which affect future growth.

Continuous retuning can approach quantization boundaries arbitrarily closely. A coarse grid provides no universal positive margin. An uncertified case therefore needs explicit refusal or an exact resolution step designed into the executor. That is additional machinery, not a free guarantee from quantization.

3. **The costs favor C now and a bounded hybrid investigation next.**

| Design | Implementation and runtime cost |
|---|---|
| **C: exact CPU stepping** | Already built. PHASES reports oak 80 at roughly 4.39–4.56 s for seed 1 and 9.74–11.04 s for seed 7 on all cores, under the reported load. No GPU growth gain. |
| **A: canonical feedback arithmetic** | Largest arithmetic port and verification burden. TASK5-GPU estimates three to four sessions for arithmetic plus two to three for the stepped executor. Those are estimates. Runtime and artifact-size costs remain unmeasured. |
| **CPU-owned feedback hybrid** | Avoids most geometry arithmetic porting, but retains CPU layout, mechanics and field work, adds mirrored structural storage and per-bud/per-node traffic, and accelerates only the remaining growth work. Speedup is unknown. |
| **Quantization with certification** | Adds error propagation, margin checks and exact handling or refusal of ambiguous cases. Could be cheap where margins are large; no evidence yet establishes coverage or total cost. |
| **D: f32 feedback** | Avoids much canonical arithmetic work but still needs the stepped GPU executor. Buys no identity guarantee; full runtime and full-executor divergence remain unmeasured. |

TASK5-GPU’s **113–282 ms synchronization measurement covers the proposed scalar/lattice schedule**. It cannot price the hybrid’s larger transfers. Likewise, the earlier 287 ms sweep measurement is only one retained CPU cost, not the hybrid’s total.

My recommendation is to keep C and first measure how much CPU time would remain under the hybrid, alongside its required record counts and bytes. If little time is available to save, delete that proposal before building it. If the owner retains the no-per-node-transfer constraint and wants light-reading growth on the GPU, **A is the strongest long-term choice**. Prove and measure its complete CPU feedback contract before undertaking the device mirror.

4. **Declining D is right.**

The sensitivity probe supports “these four trees tolerated these perturbations.” It does not establish full-device equivalence, a reliable flip rate, or behavior across seeds and retunes. Its linear extrapolation is not a correctness argument.

[fn-125](/home/daniel/Projects/telperion/.worktrees/f2b/.flow/specs/fn-125-the-generator-hands-engines-a-tree-they.md:63) makes the CPU executor the reference. F1 then explicitly requires botanical identity while allowing bounded geometric differences. A missing axis or subtree breaks that distinction.

Generator evolution permits changing the reference algorithm with evidence. It does not automatically approve backend-dependent botanical results. D could become a different owner-approved product contract, but it does not satisfy this one.

This is a design verdict based on the documents and relevant source reads. I ran no new measurements and modified no files.