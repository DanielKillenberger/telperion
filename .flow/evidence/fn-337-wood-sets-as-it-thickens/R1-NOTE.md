# fn-337 R1: the rule and what it rests on

Draft by the worker, 9 Nov 2026, before any code. Sources are listed below with what was read; a claim not checked against a source's text says so.

## The rule as the spec states it

A phytomer's bend is the sum, over the years it has stood, of each year's added moment over its radius to the fourth in that year; what a year has bent stays.

## What the mechanics give: the rule bends more, not less

Today's bend (`sag::turn`, small-deflection limit) has curvature `sag * M_T / r_T^4`: the final moment `M_T` on the final radius `r_T`. The rule's is `sag * Σ_y ΔM_y / r_y^4`. A phytomer's radius only grows, so `r_y <= r_T` in every year, each term is at least `ΔM_y / r_T^4`, and the sum is at least today's bend. The two are equal only where the radius never changed while load was added. [inferred: arithmetic on the spec's own formula]

How much more, with a radius growing in proportion to age and a moment as age to the power m, from year 1 (worker's arithmetic, a `python3` one-liner summing yearly steps, 9 Nov 2026):

| m | ratio at 40 | ratio at 300 |
|---:|---:|---:|
| 2 | 514 | 28,960 |
| 3 | 54 | 420 |
| 4 (a self-similar limb: mass r²L, lever L, r and L each in proportion to age) | 10.1 | 18.0 |
| 5 | 4.2 | 4.8 |
| 6 | 2.7 | 3.0 |

So the spec's premise, "on a young tree this is close to today's bend, and on an old one far less", has the direction reversed: early load meets thin wood and bends it more per unit, late load meets thick wood and bends it less, and the early bend is never taken back. In the large-deflection form the engine uses, the extra bend is held by the cap and the hang backstop, so limbs would reach hanging sooner rather than turn without bound. [inferred]

## Sources

Read by a research subagent of this worker, 9 Nov 2026, through firecrawl. CHECKED means the quoted text was read; UNCHECKED means only an abstract, metadata or nothing.

**The incremental formulation is the published one.**
- Alméras T., Fournier M. (2009) J. Theor. Biol. 256:370-381; accepted manuscript https://hal.science/hal-00554517v1/document, §2.4: "A given piece of wood contributes to support only from the time it is created, and does not support the preexisting loads." And: "The elementary load increments ... during an elementary time step dt can be computed, and the resulting elementary deformation increments ... can be deduced using equation (5), assuming the stiffness terms K_ij are constant during the time step." CHECKED.
- Jirasek C., Prusinkiewicz P., Moulia B. (2000), "Integrating biomechanics into developmental plant models expressed using L-systems", https://algorithmicbotany.org/papers/biomechanics00.pdf: "a new layer added by secondary growth is molded on the existing branch segment ... the rest shape of an axis after a step of secondary growth partially memorizes the actual shape of this axis under load", their eq. (24), stiffness C = EI with radial growth by the pipe model. CHECKED. On (24), a layer added under the same load leaves the bent curvature unchanged, and the next increment bends it by dM / (C_old + C_new): the spec's rule exactly. [worker's subagent's algebra, inferred]
- Fourcaud T., Lac P. (2003) Trees 17:23-30, doi:10.1007/s00468-002-0202-6, abstract (https://agritrop.cirad.fr/513527/): "a new equilibrium was written considering the increment of weight due to the mass of new wood layers ... using a cumulative process taking into account the past history of each growth ring." UNCHECKED beyond the abstract; part II not reached.
- Fournier M., Baillères H., Chanson B. (1994) Biomimetics 2:229-251. UNCHECKED (metadata only).

**The stiffness is the section at that time, I = π D^4 / 64.**
- Alméras and Fournier 2009, eqs. (11)-(12), §3.1: "K20 = E·I", "I = π·D⁴/64", dC_Y/dD = dB_Y / (E π D⁴/64). CHECKED.
- Alméras T., Costes E., Salles J.-C. (2004) Ann. Bot. 93:455, doi:10.1093/aob/mch054: "Δc(s) = Δm(s)/k(s) ... For a circular cross-section ... k(s) = πED⁴/64." CHECKED.
- Van Rooij et al. (2023) Peer Community Journal 3:e78, doi:10.24072/pcjournal.308, eq. 17a: "db/dR = 4·λ_M·ν_M/(E·π)·R^(ν_M−5)" for a moment λ_M R^ν_M. CHECKED. Integrated against today's M_T / r_T^4, it gives the ratio ν/(ν−4) for ν > 4, as the worker's table above. [the ratio is inferred]

**The direction (more bend than today) is not stated as such in any text read.** It rests on the arithmetic above. Alméras et al. 2004 say a stem's shape depends "not only on the amount of diameter growth and load, but also on the relative dynamics of these phenomena" (CHECKED); a published comparison with numbers was not found (UNCHECKED). Alméras, Gril and Costes (2002) Trees 16:5-15 (abstract, CHECKED) report that small-deflection beams underestimate a loaded stem's slope change by 48 %, large displacement by 29 % and viscoelasticity by 14 %: creep, also left out, pushes the same way.

**Without reaction the incremental sag droops without end.**
- Alméras and Fournier 2009, introduction: "without this control process, the increasing mass of growing trees would make the stem bend more and more ... trees would adopt a weeping habit because of the interaction between growth and gravity (Fournier et al., 1994a; Fournier et al., 2006)." CHECKED.
- Their gravitropic performance (eq. 37, §4.4), the maturation-strain curvature over the weight curvature: "At the critical steady limit Pg=1, the tree maintains its angle when it grows." CHECKED.
- Van Rooij et al. 2023: "Hardwoods produce 'tension wood', inducing a much higher tensile stress on one side ... tension wood is formed on the upper side while compression wood is formed on the lower side"; eccentric radial growth can be "as efficient as the formation of reaction wood" in wild cherry. CHECKED.

Not checked at all: Moulia, Coutand and Lenne 2006; Alméras et al. 2018; Fourcaud et al. 2003 part II; Fournier 1994 and 2001.

## What the rule leaves out

Reaction wood and maturation strain, which in the sources hold a growing limb's angle against exactly the bend this rule adds (Pg ≥ 1), so the rule alone, without them, makes every limb sink further each year towards a weeping habit.

## Verdict for the host

The sources support the rule's mechanics (each increment of load bends the section as it then stands, and the bend is kept), but not the spec's expected effect: by the same mechanics the rule bends an old limb more than today's settle, never less, and the sources name the missing counter-term as the reason real trees do not weep. Per R1, this returns to the host before any code.
