# fn-113 friction

## 2026-09-23 - worker, the bound audit

- Doing: reading the generator's validated bound for each of the 75 preset-span dial rows.
- Hindered by: the bounds are spread over ten validators, and there is no one call that checks a family's rows without growing a tree. Canopy rows are checked only inside placement, and `attractors` and `step` only inside `Specimen::new`. The new test sends each row group to its own validator, and a one-node tree stands in for the canopy's tree.
- Cost: about 25 minutes of reading before any edit.
- Would remove it: a validation-only entry point in `telperion-core` (a family's rows checked without growth), which a dial table could test against directly. Whether one should exist is the host's call.
