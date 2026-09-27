# fn-177 friction

## 2026-09-27, worker, task .1 (R1 measurement, then a design return)

- Doing: measuring radius along the plane candidate's thickest limbs, split into the tip-count and length-taper parts, before building any row.
- Slowed by: (1) Nothing in the repository reads radius along a limb, so a measurement example had to be written first (`examples/limb_girth.rs`). (2) Radius feeds growth, so rebuilding a family with `lengthTaper` 0 changes the topology at some settings (3 of 8 seeds at `lateralShare` 0.01). The exact split is only available where the wood is unchanged, and the tool reports null elsewhere. (3) The command guard refused a loop that redirected to a shell-expanded path, and the loop had to be rewritten as a script.
- Cost: about 25 minutes, and no product code.
- Would have removed it: a radius-profile read in the species metrics (radius at shares of a limb's path), and a spec whose architecture claim ("per limb system") had been checked against the candidate. On this candidate the thick wood is codominant stems, which fn-61's limb bound does not cover. That check is exactly what R1 does, so running R1 before the spec was marked ready would have settled the scope first.
