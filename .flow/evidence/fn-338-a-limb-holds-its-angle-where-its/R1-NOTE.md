# fn-338 R1: gravitropic performance, the premise, and the engine's posture terms

Draft by the worker, 9 Nov 2026, before any code. CHECKED means the quoted text or equation was read in the source; inferred means it follows from what was read (the arithmetic is in `probes/r1_premise.py`, its output in ignored `raw/r1_premise.out`); UNCHECKED means not read.

Source: Alméras T., Fournier M. (2009) J. Theor. Biol. 256:370-381, accepted manuscript https://hal.science/hal-00554517v1/document, read in full (text, equations, tables, figure captions; the extraction is in ignored `raw/almeras-fournier-2009.md`). The PDF extraction loses most equation numbers; the numbers below follow the text's own references ("equations (19) and (31)", "(28)", "(37)", "(38)", "(39)"). Gravitropic performance is defined in §4.3 ("Biomechanical traits determining the gravitropic performance"); §4.4 is the conclusions.

## What the source states

**The weight's side, per growth increment (CHECKED).** Eq. 12: `dC_Y/dD = (dB_Y/dD) / (E · π D⁴/64)`. The curvature changes by the *increment* of moment over the stiffness at that time; §2.4: "A given piece of wood contributes to support only from the time it is created, and does not support the preexisting loads." At the stem base, with taper `D(s) = D₀(1 − s/H)ⁿ`, distal mass `M_tot(1 − s/H)ᵐ`, load ratio `L = M_tot/V` and `b_H = (dH/H)/(dD/D₀)`, eq. 28 `B_Y = (π/4) g L sinφ H² D₀² / ((m+1)(2n+1))` and eq. 31 `dC_Y^weight/dD = 32 g L sinφ (1 + b_H) / (E (m+1)(2n+1)) · H²/D₀³`. They assume `n`, `m`, `L` constant and ignore the change in orientation during a step.

**The reaction's side, per growth increment (CHECKED).** The ring's maturation strain is `α(θ) = ᾱ + Δα/2 · cosθ`, its stiffness `E(θ) = Ē(1 + k_E cosθ)`, its thickness `dR(1 + k_O cosθ)`. Eq. 18: `dB_Y^matur = −π/2 · Ē · R² · dR · (Δα(1 + ¾ k_E k_O) + 2ᾱ(k_E + k_O))`. Eq. 19: `dC_Y^matur/dD = −4 e_r / D²`, with eq. 20 `e_r = Δα · f · Ē/E` and eq. 21 `f = 1 + ¾ k_E k_O + (k_E + k_O) · 2ᾱ/Δα` (1 for a homogeneous concentric ring). Neutral line at the centre, circular section, curvature in the vertical plane only (bilateral symmetry), small strains, elastic.

**Gravitropic performance (CHECKED).** "we define the gravitropic performance Pg as the ratio between the rate of correction dC_Y^matur/dD and the rate of disturbance dC_Y^weight/dD", defined so that the traits "combine independently of the growth rate". ">1 growth creates an upward curving process, if it is <1 growth creates a downward bending, and if it is =1 the stem section is at biomechanical equilibrium, i.e. the curvature does not change when the stem grows." At the stem base, eq. 37:

`Pg = (1/8g) · Ē · Δα · f · (1/L) · (1/sinφ) · (m+1)(2n+1)/(1 + b_H) · D/H²`

Eq. 38 carries it to any section through a "virtual truncated tree"; with `3n − m` between −0.14 and 0.56 in saplings of several species (Jaouen et al. 2007), "Pg does not vary much along the stem". §4.3: "Biomechanical parameters can exhibit adaptive or ontogenetic variations, but are contained within relatively narrow ranges ... so that their maximal value can be regarded as constant during growth"; to stay straight and tilted a stem keeps `D/H²` constant, `H ∝ D^½` (`b_H = ½`), and eq. 39 gives the steepest sustainable lean `sinφ_max = Ē Δα f (m+1)(2n+1) D / (12 g L H²)`. Usually Pg = 1 "can be achieved just by adjusting the level of gravitropic reaction, i.e. the values of Δα, f and E". Conclusions: "At the critical steady limit Pg=1, the tree maintains its angle when it grows."

Eq. 37 checks against eqs. 19 and 30 (`probes/r1_premise.py` §2, to six figures). [inferred]

**Values they report.**
- Poplar (*Populus* sp.), nine 30-year-old plantation trees at Amance, at breast height, Table 1 (CHECKED): D 0.22 to 0.27 m, lean 0.6° to 4.6°, E 5,895 to 8,715 MPa, Δα 156 to 1,022 (the column has no unit; read as microstrain it reproduces the table's correction rates to rounding, so microstrain, inferred), disturbance 0.034 to 0.050 m⁻², correction −0.012 to −0.083 m⁻². Text: "mean rate of disturbance = 0.040, SD=0.005, mean rate of correction = −0.044, SD=0.021"; "The correction process has the largest variability". The moment fits `B_Y = 2.76·10⁶ D^4.96` (Fig. 4, R² 0.73) across the trees, taken as the ontogenetic trend. Per tree, Pg is 0.30 to 2.13, mean 1.11 (inferred from Table 1). Their reading: weak reactors "are predicted to bend more and more", strong ones "are probably in an active up-righting process", and the sample is "close to the biomechanical equilibrium".
- By wood type, Table 2 (from Alméras et al. 2005, 11 angiosperm and 4 gymnosperm trees of unnamed species; CHECKED): angiosperms ᾱ −0.14 % mean (−0.18 max), Δα 0.17 % (0.28), k_O −0.23 (−0.45), k_E −0.09 (−0.25); gymnosperms ᾱ 0.05 % (0.07), Δα 0.15 % (0.20), k_O 0.31 (0.44), k_E −0.18 (−0.26). Form factor (Fig. 5 and text): angiosperms 1 to 2.22, gymnosperms "no more than 1.26"; compression wood is "intrinsically less efficient because of its lower stiffness". A two-sector ring at its best is 1.27 times the sinusoidal one (§4.2).
- Pith and radial gradient (CHECKED): `Ē/E = 1/(1 + (γ_p − 1)ρ_p⁴)`, significant past a pith of 30 % of the radius, "e.g. to 3-fold if the pith extends into 90%"; a linear juvenile-wood gradient changes it by "a few percent" (eq. 36).
- Fig. 8 (CHECKED from the caption and text only; the extracted plot values are not trusted): mean parameters Δα 0.17 %, f 1.54, E 1.23·10¹⁰ N m⁻², m 1.62, n 0.66, L 1,340 kg m⁻³ (15 tropical angiosperm species, Jaouen et al. 2007), where "a mean stem of 20 cm is limited by gravitropism if its angle exceeds 15°, and its maximal height is 30 m"; *Sextonia rubra* (E 8.5·10⁹, m 0.63, n 0.3, L 1,438) "at an angle of 10°, with a maximal height of 20 m".
- Nothing for *Quercus*, *Fagus*, *Betula* or *Picea*: the paper gives no oak, beech, birch or spruce value.

## The premise, line by line

The premise: the weight's curvature goes as the moment over r⁴, the reaction's as the wood's growth over r², so their ratio over a limb's life goes as r³ over the moment, the age cancelling; the bend is then the sag of the moment left over once the wood has held a share of r³, and nothing where it holds it all.

1. **"The weight's curvature goes as the moment over the radius to the fourth": not in the source's terms.** In eq. 12 the weight's curvature *change* goes as the moment's *increment* over the radius to the fourth at that time. The moment over the final radius to the fourth is the engine's settle, which fn-337 showed is smaller than the source's sum of increments. [CHECKED against eq. 12 and §2.4]
2. **"The reaction's as the wood's growth over the radius squared": yes.** Eq. 19 in radius: `dC^matur = −2 e_r dR/R²`. [CHECKED, rearrangement inferred]
3. **"Their ratio goes as the radius cubed over the moment": does not follow as written.** The ratio of the two rates is `Pg = (π/2) Ē Δα f · R² / (dM/dR)`: the radius squared over the moment's growth per unit radius, not r³/M. It becomes `(π/2) Ē Δα f R³ / (k M)` only if the section's moment grows as a power of its radius over its life, `M ∝ R^k`, and then carries `1/k`; at the stem base under their allometry `k = 2(1 + b_H)` (`probes/r1_premise.py` §2 reproduces eq. 37 this way). [inferred]
4. **"The limb's age cancelling": what cancels in the source is the growth rate (dD/dt), not the age.** Both rates are per diameter increment, which is why Pg is defined. `dM/dR` is a history quantity, and Pg changes as the limb grows: under `M ∝ R^k`, `Pg ∝ R^{3−k}`, so a limb whose moment grows faster than r³ loses performance as it grows (the source's `H ∝ D^½` condition is `k = 3`). [CHECKED for the growth rate; the rest inferred]
5. **The premise's conclusion holds exactly by another route.** Eq. 18 gives the reaction's moment increment `(π/2) Ē Δα f R² dR`, so with `Ē Δα f` constant over the section's life (§4.3's "maximal value ... constant during growth"), the moment the reaction can balance from radius 0 to R is `M* = (π/6) · Ē · Δα · f · R³`: a share of the radius cubed, with no `k`, no age and no growth rate. The leftover increment is then an exact differential, `dM · (1 − Pg) = d(M − M*)`. [inferred from eq. 18]
6. **Dividing the leftover by the final radius to the fourth does not follow.** The source's curvature is the sum of each leftover increment over the stiffness at its time, `∫ d(M − M*)/(E π R⁴/4)`, not `(M − M*)/(E π R_T⁴/4)`. This is fn-337's finding again, now on the leftover moment. [inferred]
7. **"Nothing where the wood holds it all" is a reading, and it starts the bend at the wrong radius.** The clamp matches §4.3's regulated reaction (adjusted to hold the angle up to its maximum). But under regulation the bend starts where Pg at its maximum falls to 1, `R_c`, not where M reaches M*, `R₀`: under `M ∝ R^k`, `R_c/R₀ = (3/k)^{1/(k−3)}`, 0.75 at k = 4, 0.77 at 5, 0.79 at 6. Between them the source bends and the premise does not. The source also has Pg > 1 bending a limb up when the reaction is not regulated down (the poplars "probably in an active up-righting process"); the clamp leaves that out. [CHECKED for the text; the radii inferred]

How far apart, on one section with `M = R^k` in units where `M* = R³` (so `R₀ = 1`) and with sag folded out (`probes/r1_premise.py` §4; today is `M/R⁴`, the premise `max(0, M − M*)/R⁴`, the source `∫ max(0, dM − dM*)/R⁴`):

| k | R/R₀ | today | premise | source | source / premise |
|---:|---:|---:|---:|---:|---:|
| 4 | 1.0 | 1.00 | 0 | 0.15 | — |
| 4 | 2.0 | 1.00 | 0.50 | 1.42 | 2.85 |
| 4 | 4.0 | 1.00 | 0.75 | 3.45 | 4.59 |
| 5 | 2.0 | 2.00 | 1.50 | 3.75 | 2.50 |
| 5 | 4.0 | 4.00 | 3.75 | 13.00 | 3.47 |
| 6 | 2.0 | 4.00 | 3.50 | 7.83 | 2.24 |
| 6 | 4.0 | 16.00 | 15.75 | 43.08 | 2.74 |

So by the source's own mechanics, a limb whose moment grows as r⁴ or faster ends, once well past `R₀`, bending more than today's settle even with its reaction at its maximum; the premise always bends less than today. The reaction does bound the source's sum from below (it starts at `R_c`, not at radius 0), which removes fn-337's unbounded growth at k = 4. [inferred]

**Verdict.** The premise's arithmetic as stated fails at steps 1, 3 and 4, but its conclusion, a held moment proportional to r³ that leaves no `k`, age or growth rate, follows exactly from eq. 18 under constant maximal `Ē Δα f`. What the premise's settle form adds beyond the source is the substitution of the final moment over the final r⁴ for the sum of increments (steps 6 and 7). That substitution is what makes it read only the settle's state and give the same tree however the age was reached; the source's bend is path-dependent.

## What the source's expression needs that the settle does not hold

- **A reaction capacity `Ē · Δα · f`**, a wood trait the source gives by wood type (Table 2, Fig. 5), not by the engine's species. No preset row holds it.
- **The settle's units.** The settle's mass is `(r² + leaves²) · length` (`sag.rs`, `gather`), a volume without π or density, so its moment `M_e` is in m⁴ and the real moment is `ρ g π M_e`. `M*` in those units is `s · R³` with `s = Ē Δα f / (6 ρ g)`: at Ē 10 GPa and ρ 1,000 kg m⁻³, about 440 m for the angiosperm mean (e_r ≈ 0.0026), 1,060 m at the angiosperm maximum, 260 m and 430 m for gymnosperms (`probes/r1_premise.py` §5). The density lives only inside `sag`. [inferred]
- **Whether the reaction is measured against `sag` or against the wood's real stiffness.** The engine's elastic sag would be `4ρg/E` (with the settle's mass convention): 3.1·10⁻⁶ per metre at 800 kg m⁻³ and 10 GPa, 4.1·10⁻⁶ at 1,050. The oak limb's 3·10⁻⁴ is 73 to 96 times that. The spec's "about a millionth per metre" is low by about π (the settle's mass has no π, so `E π R⁴/4` meets `ρ g π`); its direction holds, and 300 times is about 80 times. [inferred; the host's number was unchecked] If `s` comes from real E and density, the sag's extra 70 to 100 times droops only the leftover; if the share is written as `s · sag`, it is the dimensionless `(2/3) e_r · (sag / elastic sag)`, and that factor has to come from somewhere. This is a design question for the host.
- **The history**: each leftover increment over its own r⁴ (step 6), and `k` or the moment's growth against the radius for where the bend starts (step 7). The settle has only the final M and r.
- **Primary growth.** The source's reaction comes from secondary growth only ("Two different 'motors' ... hydrostatic pressure and differential growth in primary tissues, and wood maturation in lignified axes"); the current year's shoot has none. The pith factor raises `e_r` for young shoots ("e.g. to 3-fold if the pith extends into 90%", §4.2).
- **Out-of-plane bending**: excluded by bilateral symmetry. The settle's torque is the lever crossed with gravity, a horizontal axis, so its bend is in the vertical plane already. [read in `sag/turn.rs`, `geometry/sagged.rs`]
- **The lean**: in the source it enters through `sinφ` in the moment; the settle's lever already carries it. [read]

## What the engine already has that acts as posture control

All four are applied while laying a phytomer in `Layer::step` (`geometry/lay.rs`), in the settle's lays (`geometry/place.rs`, `start`) and in the year loop's sketch (`grow/draw.rs`), which has no sag. Order within a step: tropism, then wander, then sag (both on the running direction, which carries to the next phytomer), then the pulls and take-over (on this phytomer's heading only, which does not carry). [read]

- **`tropism`** (`Form::tropism`, `Form::elevation`, per PA): each internode turns the running direction towards the PA's elevation by the share `1 − e^(−tropism · turning · length)`, `turning` the axis's draw (`lineage::tropism`, one without `tropism_spread`), eased inside a cone near straight down (`toward_elevation`). It reads the PA's rate and elevation, the axis's lineage draw and the internode's length, never radius, load or age. Because sag turns the running direction and tropism acts on it at the next node, tropism already takes back part of each node's sag along the axis, by length. [read]
- **`straightening`** (`PaState::straightening`): the base of a lateral is pulled towards the vertical by this share, the pull fading linearly to none over the axis's scaled reach (`geometry::reaches`, its phytomers' scales plus its relays' as far as each still carries it on); a relay takes it by its blend (`Axis::carried_on`), a continuation and the seed none. It reads the PA's share, the axis's origin and blend, and its reach. Applied to each phytomer's heading, not carried. [read, `geometry::bend`, `Layer::step`]
- **`erection`** (`PaState::erection`): Troll's secondary erection, combined with straightening as `1 − (1 − straightening)(1 − (1 − e^(−erection · years)))`, `years` the structure's age less the axis's birth and its sleep; the seed takes it too. It is the only posture term that reads age, and it lifts, by the same fading pull, whatever the load. [read, `geometry::bend`; `place.rs` `start`, `grow/draw.rs` `bent`]
- **`takeover`** (`PaState::takeover`, fn-286): a relay's heading is blended back onto its parent's running direction at the parent's tip, by takeover times the relay's blend, fading as its own pull fades, the lifts sharing what it leaves; a continuation carries on its relay's take-over until that reach is spent (`geometry::taken`, `joined`). It reads the PA's share, the relay's blend and its parent's end direction; relays only. [read]

None reads a phytomer's radius or moment, so none answers the load the way the source's reaction does. The oak's limb PA, the one with sag 3·10⁻⁴, has straightening 0.15, takeover 0, tropism 0.5 towards elevation 1.05; its bough (sag 5·10⁻⁵) takeover 0.75, tropism 0.48 towards 0.9. [read, `oak.rs`]

## Other sources

Not re-read for this note; fn-337's R1-NOTE.md records what it checked of Jirasek, Prusinkiewicz and Moulia (2000) and Van Rooij et al. (2023). Alméras et al. 2005 (Table 2's source), Jaouen et al. 2007 (Fig. 8's parameters), Coutand et al. 2007, Fournier et al. 1994a and Moulia et al. 2006: UNCHECKED.

## Questions for the host

1. Whether the probe takes the premise's settle form (`max(0, M − s R³)` over the final r⁴), knowing it bends less than the source's incremental form by 2 to 5 times past `R₀` and ignores the bend between `R_c` and `R₀`.
2. Whether the share is set from real wood (`s = Ē Δα f / (6 ρ g)`, needing a density and a reaction capacity) or relative to `sag`, and so whether the sag's 70 to 100 times over elastic droops the leftover only.
3. Whether one value for every species is acceptable when the source separates angiosperms and gymnosperms (Table 2, Fig. 5) and gives no value for any preset's species.
