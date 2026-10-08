**Verdict: accept with changes.** A has the right scope and a credible performance opportunity. The current arithmetic specification does **not yet establish identity by construction**. Its induction is sound only after the operations, ranges and exceptional cases below have unique, implementable meanings.

The measured 50–56% movable share justifies a bounded proof build. It does not establish a speedup, especially because the re-lay share is apportioned and the cost of the new integer arithmetic remains unknown.

**1. Where the identity argument holds, and where it fails**

The contract correctly closes the feedback loop through geometry, mechanics, light, allocation and balance. Applying the same arithmetic above and below the cut, retaining the four phases, and sharing one CPU sweep would make identical cycle state produce identical next-cycle state.

The main failures are in [§2.3’s arithmetic](/home/daniel/Projects/telperion/.worktrees/f2b/.flow/evidence/fn-211-tree-space-f2b-growth-years-cheap/CONTRACT-A.md:133) and its uses:

- **Some intermediates cannot fit their specified width.** Four Q32 factors representing one multiply to raw `2^128`, already outside i128. That affects the one-rounding scale product; epitony needs still more width. Likewise, W/W division to Q32 cannot generally shift its numerator left 32 inside i128. Specify wider intermediates or an exact multiply/divide algorithm, including its rounding. Checking an already-overflowed product is too late.
- **A wrapped result cannot reveal the true sum.** Eight positive terms of raw `2^125` wrap to zero in 128 bits. Checking that zero against `2^126` accepts it. The proposed read-time range check needs additional information.
- **Angles lack sufficient semantics.** Wrapped orientations, signed rotation amounts, elevation differences and angular rates cannot all use an undifferentiated u32. Define signed representatives, interpolation and half-angle rules, and where wrapping occurs. In particular, fractional multiplication must not discard complete turns prematurely.
- **The radian conversion contains a concrete precision error.** Encoding `2π / 2^32` as Q32 produces raw **6**, rather than approximately 6.283185. Used literally in sag, that introduces about **4.5% error**. Use a sufficiently precise constant and an explicitly scaled product.
- **Small-vector normalization is inaccurate.** For raw Q32 vector `(1,1,0)`, the specified integer length is `1`; normalization consequently produces `(1,1,0)` in real units, whose length is √2. Scale before taking the root, or compute the normalized quotient with enough guard precision.
- **The zero cases are incomplete.** `ripe` needs today’s `ripening ≤ 0 → 1`; scalar `asin(cap/reach)` is not defined by the vector-elevation formula; half-angle tangents need endpoint handling. Zero light must preserve certain survival when viability is one, and shade response must remain dormant when φ is zero. [Today’s survival code explicitly preserves certain survival.](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/grow/advance.rs:48)

These are contract defects, rather than questions that an implementation should answer independently.

**2. The seven choices**

1. **Every axis uses fixed point — accept.**  
   Every movable feedback operation needs the same arithmetic on either side of the cut. Extend this across supported parameter combinations, including paths currently dormant in an unlit tree. Preserve operation placement at handovers: an exchanged value must not receive an extra rounding merely because it crossed the cut.

   Keeping terminal geometry in f64 is compatible with the stated identity requirement, provided it cannot feed growth or alter botanical identities. Keep shared stage logic; copying the layout implementation into separate rough and final algorithms would create maintenance and pipeline problems.

2. **Q32, W, turns, CORDIC and isqrt — accept the approach, revise the specification.**  
   Q32 positions and W accumulators are reasonable starting formats. They do not, by themselves, supply precision and range for every intermediate.

   CORDIC needs literal tables, exact update equations, simultaneous-update semantics, zero-direction ties, output rounding and input normalization. Shifting arbitrary `atan2` inputs into Q62 can overflow; vectoring also increases their magnitude.

   W `log2`/`exp2` needs an actual algorithm and error bound. Shifting a Q62 exponential into W does not recover the two missing fractional bits or restore values that already underflowed.

   **Keep small mechanics in sufficient precision.** Shortening a W lever to Q32 before measuring it discards positive components below `2^-32`, while arithmetic shifting a tiny negative component produces `-1`. That introduces directional bias and can suppress fine-twig sag. A small moment divided by `r⁴` need not produce a small bend. Normalize from W, or retain its magnitude through a specified scaling.

   Also reconcile the moment bound assuming positions below `2^37` raw Q32, approximately 32 metres, with §2.10’s much larger allowed position range. The present bounds do not establish one consistent supported domain.

3. **Wrapping sums with range checks — change.**  
   Wrapping addition provides grouping-independent residues. To recover exact sums, prove a bound before accumulation, or retain enough extra information to detect range violations.

   For signed moments, a checked bound on the sum of absolute term magnitudes is one sufficient approach. Checking overflow on each signed partial alone can make refusal depend on grouping. The acceptance/refusal result must also be independent of cut and order.

   Separate numerical overflow from intentional probability saturation. Saturating a reduction is problematic; a specified pointwise probability clamp is deterministic. §2.10’s blanket statement about saturation conflicts with its own exponential and hazard rules.

4. **CPU f64 sweep, rounded once — accept for the first implementation.**  
   Both executors can consume the same canonical integer field from one authoritative CPU operation. Preserve the sweep’s direction order, lattice extent, boundary rules and integer-to-f64 conversion.

   State the guarantee precisely: CPU/GPU placement identity using the same host sweep and supplied constants. This does not establish bitwise identity across different host floating-point implementations.

   Delete the speculative device-sweep appendix from the binding first contract, or mark it explicitly non-normative. Moving the sweep later requires its own completed arithmetic specification and measurements.

5. **Corner weights sum to one — accept, correct the conservation claim.**  
   Seven floored weights plus a residual eighth is deterministic, nonnegative and exactly normalized. It also makes sampling a constant-one field return one.

   **It does not make rounded deposits conserve area exactly.** Each subsequent `area × weight` conversion to W can lose a remainder. I checked an integer example with Q32 area raw `1` and three approximately one-third fractions: the eight deposits lose three W units.

   If exact conservation is required, distribute the residual at the deposit level too. Otherwise state the bounded loss accurately. Specify the lattice’s upper corner, empty case and checked dimension product as well.

6. **D16 integer threshold and Q32 kept share — accept.**  
   This aligns the shedding decision and its surviving presence with the existing decision contract. Specify threshold equality and endpoints, inherited kept-share multiplication, and the exact existing window conversion.

   Treat it as a reference change. Today’s balance uses an f64 probability and a lead from that probability; the proposed presence uses the integer threshold. That can change later growth even when the immediate shed decision stays the same.

7. **Move the re-lay to the device — include it in the intended resident executor.**  
   The re-lay writes frames and walks back into growth, as [the current implementation shows](/home/daniel/Projects/telperion/.worktrees/f2b/crates/telperion-space/src/grow/relay.rs:1). Its arithmetic therefore belongs in A regardless of initial execution placement.

   A CPU-only re-lay is useful while proving the reference. A production CPU re-lay over resident fine geometry would require structural transfers and scatter-back, revisiting the established transfer constraint. The 34–38% movable share without re-lay does not price those transfers.

   Preserve realized section/load exchanges and the unbent-place → gather → bent-place dependency. Measure the actual device schedule before crediting the apportioned 14.5–20% re-lay saving.

**3. What changes in today’s trees, and how to show it**

This changes the numerical reference for geometry and feedback across species. Expected changes include positions, frames, radii, sag, light, sizes, balance memories and presences. Threshold-adjacent cases can change topology. The present contract cannot promise merely rounding-level movement: its normalization, angle-conversion and small-moment issues can cause larger changes.

Keep two comparisons separate:

- **New CPU versus new GPU:** exact feedback state and botanical identity after every phase, across cuts, orderings, chunkings and retries.
- **New CPU versus today:** the 28-tree comparison, reporting topology changes by lineage separately from displacement and radius changes on matching elements. Include light, allocation, shedding and ground-contact checks.

View the four species at 80, both seeds, bare and in leaf, using canonical ordering and matched views. Inspect close views of fine twigs and contacts; include motion where applicable. Exercise parameter neighborhoods around zero shade response, zero sag, zero ripening, degenerate frames and decision thresholds. Existing seeds alone cannot establish behavior across the continuous tree space.

Measure CPU regression, device runtime, transfer bytes, peak memory and shipped artifact size. The ceiling measured today’s arithmetic, whose cost will change. Re-pin hashes after this evidence establishes the new reference.

**4. The smallest first build step**

Start with a **small CPU/GPU arithmetic conformance probe**, before converting the whole CPU generator.

Use exact integer expected results for signed shifts, wide products/division, normalization, angle conversions and reduction/refusal behavior. Include the counterexamples above. Compare every result word and error status against a small actual device mirror. Floating-point accuracy tests are a separate check.

Then connect the smallest feedback slice: canonical leaf position and area → integer deposits → the shared CPU sweep → integer sampling → D16 threshold and kept share. Vary deposit order and chunking, and force a buffer retry. This proves the arithmetic boundary and exchange mechanism without committing to the whole port.

That first step does **not** prove whole-tree identity. The next integration proof should use a small lit specimen spanning a scheduled re-lay and compare complete phase state. Only then expand to the four lit oaks and performance measurement.

I read the supplied contracts and relevant source and ran small integer counterexamples. I ran no tree builds or visual comparisons and modified no files.