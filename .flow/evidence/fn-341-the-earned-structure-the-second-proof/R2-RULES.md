# fn-341 R2: the rules on paper

Drafted by the worker on 10 Nov 2026 for the host. Nothing here is chosen. For each rule the note gives:
- the rule as its source states it;
- a formula over a branch's present state;
- the settings it would replace;
- the traits it needs;
- arithmetic on one stated branch of the oak;
- the options the worker sees, with the one the source's own statement implies marked.

No probe has been built. The arithmetic is `probes/r2.py`, with its output in ignored `raw/r2.out`.

**The stated branches** are all from the oak at 80, seed 1001 (`raw/r1-oregon-white-oak-80.log`):
- **The trunk:** DBH 1.10 m, height 17.4 m.
- **The fork at 4.72 m:** wood below 1.10 m; two limbs of 0.71 and 0.53 m; the carrier on, 0.86 m by the pipe sum at exponent 2.6.
- **The limb:** the 0.53 m one, 12.7 m along its thickest wood. Its tip is 8.0 m out and 9.1 m up, a chord 41° from vertical; its base leaves 26° from vertical.

Rules are ordered as R1 ranks what they would move, girth first.

## 1. Girth (R1's furthest measure)

**As the sources state it (checked).**
- **Shinozaki et al. 1964, part II, the pipe model:** "the total cross-sectional area of stems and branches at a certain horizon … was always proportional to the total amount of leaves existing above that horizon". Below the crown, "the cross-sectional area of the trunks continues to increase toward the base … most probably due to the accumulation of disused pipes in the trunk which were once connected with the leaves born by the lower branches already shed off years ago".
- **White 1998 ¶4 and ¶6:** once the crown is at its optimum, annual rings "are of the same cross-sectional area"; "Thickness of the stem is a constant non-reversible feature of tree growth".
- **Pressler's law** ("ring area growth … proportional to the quantity of foliage above this point") was read only as Lehnebach et al. 2018 quote it, so it is **unchecked** against Pressler or Larson. Lehnebach also say it "is not generally valid".
- **Functional form.** Shinozaki gives a proportionality in words, with no equation for the disused pipes. No source read gives a ring area per unit of leaf for oak or beech.

**Formula over present state.** At a section: active pipe area = A_L(above) / R, where A_L is the leaf area above and R the measured leaf-to-sapwood ratio. The section's area never falls. Today's model by contrast is Σ (per-phytomer pipe)^e over the wood held, with shed pipes dropped (R1, "What sets each measure").

**Settings it would replace.** Every age's `Form::pipe`, `Form::ripening` and the oak's `retained` 0. Possibly `Form::exponent` and `leaf_girth`.

**Traits needed.**
- **R for *Q. garryana*:** 0.23 m² cm⁻² at 40 years and 10 m, 0.37 at 250 years and 25 m (Phillips et al. 2003, Table 1, checked).
- **R for *F. sylvatica*:** 0.331 (Bartelink 1997, checked), but 0.54 for the same data in McDowell et al. 2002 (checked). The two disagree.
- **R for *Q. robur*:** none found.

**Arithmetic, the oak's trunk at 1.3 m, seed 1001** (`raw/r2.out`). The leaf is the cycle's current-year leaf on the same trees, from fn-340's READ lines: 121.6 m² at 40 and 442.6 m² at 80.
- **At 40:** sapwood 329 to 529 cm², a sapwood-only DBH of 20 to 26 cm. The model's section is 2,624 cm² (DBH 0.58 m), 5 to 8 times that.
- **At 80:** sapwood 1,196 to 1,924 cm², DBH 39 to 49 cm before any disused pipe. The model's section is 9,538 cm², 5 to 8 times that.
- **Direction and size:** the trunk falls toward White's 28 to 40 cm at 40, and Phillips' measured 23 cm. At 80 the sapwood alone sits at the lower end of White's 56 to 79 cm, with the disused pipes adding to it. The crown spread for the trunk (R1 item 5) would move toward Pretzsch's relation by the same factor. [inferred]

**Options the worker sees** (the host's choice):
- **(a) Pipe per leaf, disused pipes kept: implied by Shinozaki's statement.** A section's area is the most it has ever carried plus what shed branches left, never less. Needs R. How a disused pipe is counted (each leaf-year's pipe kept, or the section's maximum) is not given by the source.
- **(b) Today's per-phytomer pipe, with `retained` at 1** (Pałubicki keeps shed pipes whole). The direction is right, so the trunk never thins. The size goes the wrong way: `oak.rs` records that at 0.5 the stem stood 2 to 4 times Silvics' DBH.
- **(c) Pressler: each year's ring area proportional to the leaf above, summed.** Needs a ring-area-per-leaf coefficient. None was found measured for oak or beech, so it would be an estimate.

## 2. Posture by gravitropic performance

**As the source states it (checked; fn-338 `R1-NOTE.md`).** Alméras and Fournier 2009, eq. 37: Pg = (1/8g) · Ē · Δα · f · (1/L) · (1/sinφ) · (m+1)(2n+1)/(1+b_H) · D/H². ">1 growth creates an upward curving process, if it is <1 growth creates a downward bending". Eq. 39 gives the steepest lean held: sinφ_max = Ē Δα f (m+1)(2n+1) D / (12 g L H²). The source defines it for a stem. Using a branch's length as H is the worker's application, not the source's. [inferred]

**Formula over present state.** For each limb section: D and the length H borne beyond it. While its lean φ ≤ φ_max it holds its angle. Beyond that, growth bends it.

**Settings it would replace.** `Form::sag` (applied once at the settle, about 80 times an elastic beam's), and possibly `straightening` and `erection`.

**Traits.**
- **Δα, f, L, m and n:** none for *Quercus* or *Fagus*. Only Fig. 8's tropical angiosperm means exist: Ē 12.3 GPa, Δα 0.17%, f 1.54, L 1,340 kg m⁻³, m 1.62, n 0.66.
- **Green modulus:** only for American relatives, *Q. alba* 8.6 GPa and *F. grandifolia* 9.5 GPa (Wood Handbook Table 5-3a, checked).
- **Beech growth strain:** about 1,800 µstrain marks tension wood (Thibaut and Gril 2021, a review), with no tension-minus-opposite value.

**Arithmetic** (`raw/r2.out`, Fig. 8's means):
- **The coefficient** Ē Δα f (m+1)(2n+1)/(12 g L) is 1,241 m. It reproduces the source's own check: 0.20 m at 30 m gives 16°, where Fig. 8 says the limit is 15°.
- **The stated limb**, D 0.527 m, H 12.7 m: sinφ_max 4.0, so it holds any lean. At half its girth it is 2.0 and still holds any lean.
- **The trunk:** 4.5.
- **Where a branch stops holding horizontal:** past 7.9 m at 5 cm, 11.1 m at 10 cm, 19.3 m at 30 cm.
- **With *Q. alba*'s 8.6 GPa** the coefficient falls by 0.70 and nothing above changes sign.
- **Direction on R1 (inferred):**
  - The oak's limbs would stop sagging and hold their growth angle: 25 to 31° at the base, against today's chord of 41° on the stated limb.
  - Only thin, long wood would bend.
  - The crown spread at 80 to 120, now 1.2 times the range, would fall if the sag carries much of it. The probe did not separate sag from growth angle.

**Options:**
- **(a) Eq. 39 per section, as a limit on lean: the source's statement.** At the oak's sizes it binds almost nowhere.
- **(b) The source's incremental bend:** fn-338's sum of leftover increments, path-dependent, which bends more than (a) past R₀.
- **(c) A settle form**, fn-338's premise, which bends less.

fn-338's three questions to the host (the form, how the share is scaled, one value or two wood types) stand unanswered.

## 3. Vigour across orders setting how much a shoot grows

**As the sources state it (checked).** Pałubicki et al. 2009 §4.2:
- v_base = αQ_base;
- v_m = v·λQ_m / (λQ_m + (1−λ)Q_l), v_l = v·(1−λ)Q_l / (λQ_m + (1−λ)Q_l);
- "The integer part of the amount v of the resource reaching a bud determines the number of metamers produced by this bud: n = ⌊v⌋. The length l of these internodes is calculated as l = v/n."

Borchert and Honda 1984 split a flux by a fixed ratio Rf (0.7 by default) and use it to decide whether a branch goes on branching, not how much it grows. The functional form is given in both.

**Today (read, `allocation.rs`).** The split is already Pałubicki's, with λ `apical_control` (0.45 on the oak). But a bud's size is then (r/r̄_g)^ψ, normalised so that each age's mean is 1: "light moves growth within an order, never from one order to another" (`sizes`). The node count stays the zone's draw, and the size scales internode length and girth only.

**Formula over present state.** A bud's growth from its own v. In Pałubicki's form: n = ⌊v⌋ metamers, each of length v/n. The age's own row would be read for what the shoot is, not how much of it grows.

**Settings it would replace.**
- The per-age normalisation in `sizes` and ψ (`shade_size`).
- Each age's node counts per unit (`NodeLaw` ranges) and its fixed `internode`, in Pałubicki's form.
- α, which today cancels (host decision 15) and would return as the scale linking light to metamers.

**Traits.** λ, which Pałubicki sets at 0.46 to 0.54 in Fig. 7 with no measured value for any species, and α. Neither has a measured value for oak or beech.

**Arithmetic, the fork at 4.72 m** (`raw/r2.out`). The light per subtree was not read by the probe, so each wood's section r² stands in for its light (an estimate). The carrier, 0.86 m, takes Q_m share 0.486 against the two limbs' 0.514.
- At λ 0.45 the carrier gets 43.6% of the vigour, the limbs 56.4%.
- Today that split moves growth only within the leader's age and within the limb age, each against its own mean. Without the normalisation the limbs' age would gain against the leader's in proportion to its light.
- **Direction (inferred):** more spread against height. It also bears on R1 items 5 and 6. The size cannot be put on paper without the tree's light, so a run is needed.

**Options:**
- **(a) Pałubicki's n = ⌊v⌋, l = v/n, one α for the tree: the source's statement.** Every unit's length then follows light, and the age's node counts go.
- **(b) Keep today's node draw and scale its size by v against the whole tree's mean** in place of the age's mean: the smallest change.
- **(c) GreenLab's trophic form** (Mathieu et al. 2009, checked): phytomers per unit u = ⌊β + β′ q⌋. It needs β and β′ per age, which are not measured.

## 4. Rank by vigour

**What the sources state (checked).**
- **Barthélémy and Caraglio 2007:** physiological age "may be estimated a posteriori"; it "may not be an intrinsic property of this meristem itself but the result of all morphogenetic interactions between plant parts"; vigorous axes "may be considered as 'physiologically young'". They also state "there is neither automatic nor direct correlation between the privileged position and the relative vigour of lateral branches".
- **Pałubicki's order** is topological only.
- **GreenLab (Mathieu et al. 2009)** ties the number of active buds, b = ⌊α + α′ p Q/D⌋, and a branch's life span, τ = ⌊γ + γ′ q⌋, to the trophic state. It sends the physiological age's change to Mathieu 2006, which was not read.
- **No source read gives a functional form for a lateral's rank or physiological age from its vigour.**

**Settings it would replace.** The oak's trunk timer (lifespan 12), the fork's 2 years and its limb probability p 0.8, the leader's 6 years and its p 0.08, and the limb's own p 0.007. On the beech: the trunk's 9 years, the leader's 40, and the fork's. These set R1's fork height (about 4.5 m whatever the light), limb count and lowest limb.

**Arithmetic.** None is possible without a form. The fork's split above gives each limb a vigour share of 0.20 to 0.36 of the fork's (the stand-in light). That is the input any form would read.

**Options (none source-implied):**
- **(a) A lateral takes the next age up when its vigour passes a share of its bearer's.** It needs a threshold no source gives.
- **(b) GreenLab's life span τ = ⌊γ + γ′ q⌋ in place of each timer**, so a vigorous stem stays trunk longer. It needs γ and γ′, not measured.
- **(c) Rank as a continuous value read from vigour each year.** The age's rows are interpolated, which the tree space's continuity allows. It has no source.

## 5. Height slowing as the path from the root lengthens

**What the sources state (checked).**
- **Ryan and Yoder 1997:** longer paths raise hydraulic resistance; stomata "close earlier in the day or earlier in a drought cycle. This closure lowers stomatal conductance and photosynthesis, reducing carbon available for wood growth". The paper was read through a directed query that found no equation.
- **Ryan, Phillips and Bond 2006:** G_C = K_L ΔΨ / D, and "Gravitational potential adds 0.01 MPa m⁻¹". **Phillips et al. 2003** give K_L = 1/A_L · 1/Σ (h_i / (k_S,i A_S,i)), with h_i the path length.
- **Koch et al. 2004:** midday Ψ = −0.00973 h − 0.712, reaching −1.9 MPa at 122 m (redwood), with leaf traits linear or exponential in height.
- **No source gives height growth as a function of path length.** Height enters only through conductance and leaf traits.

**Formula over present state, if one were taken.** A bud's assimilation is scaled by its path's conductance, K_L from Σ h_i/(k_S A_S) along its path to the root, or by the gravity term 0.01 MPa per metre of height against a limiting Ψ. Each needs a link from conductance to growth that no source read gives.

**Settings it would replace.** None directly: nothing limits height today. It would act on the oak's 500-year limb life at a constant internode.

**Traits.** k_S (sapwood-specific conductivity) and a limiting leaf water potential for oak and beech: none read. Koch's −1.9 MPa limit is for redwood.

**Arithmetic.**
- The oak at 300 stands at 40.0 m (31.7 to 47.8), 1.15 times *Q. garryana*'s potential maximum of 34.8 m (Gould 2011).
- Under Koch's gravity term alone, 40 m adds 0.39 MPa to the gradient against 0.34 MPa at 34.8 m.
- Whether that stops growth depends on the limiting Ψ, which is unread for oak.

**Options (none source-implied):**
- **(a) Conductance by path length in the carbon balance:** today's `upkeep` per metre gains a hydraulic term. It needs k_S and the link from conductance to assimilation.
- **(b) A species' measured potential height** (34.8 m for *Q. garryana*) as the asymptote of a stated curve. That is a measured value, but the curve's form would be chosen.
- **(c) Leave height to the other rules.** With a lighter trunk (rule 1) and vigour shared across orders (rule 3), measure first whether height still overshoots.

## For the host

Not decided here:
- which rules enter the probe, and in what order;
- whether girth is a fifth rule;
- each rule's option;
- what stands in where a source gives no form: rank (4) and height (5).

## Sources added by R2

Each is checked: read in the source's own text by the literature pass, with quotes kept in the worker's session.
- **Pałubicki et al. 2009:** https://algorithmicbotany.org/papers/selforg.sig2009.pdf, §4.2.
- **Borchert and Honda 1984, Bot. Gaz. 145:** eqs. 1 to 4; the text layer is garbled.
- **Barthélémy and Caraglio 2007, Ann. Bot. 99:** PMC2802949.
- **Mathieu et al. 2009, Ann. Bot. 103:** PMC2685317.
- **Ryan and Yoder 1997, BioScience 47:** read through a directed query only.
- **Ryan, Phillips and Bond 2006, PCE 29:** treesearch 61135.
- **Koch et al. 2004, Nature 428:** Table 1; the text layer is garbled.
- **Phillips et al. 2003, Funct. Ecol. 17:** Table 1.
- **Shinozaki et al. 1964, Jpn. J. Ecol. 14, part II.**
- **Bartelink 1997, Ann. Sci. For. 54.**
- **McDowell et al. 2002, Oecologia:** Table 1.
- **Wood Handbook 2010:** Table 5-3a.
- **Thibaut and Gril 2021, Peer Community J. 1:** e46.
- **Lehnebach et al. 2018:** quoting Pressler, so that quote is unchecked against Pressler.
