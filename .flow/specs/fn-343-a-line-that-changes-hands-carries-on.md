# A line that changes hands carries on without an elbow

## Conversation Evidence

> owner, 10 Nov 2026: "Today's is a bit jagged but it looks directionally correct. The jaggedness is worse at 40."
> owner, 10 Nov 2026: "every probe is worse than today"
> owner, 10 Nov 2026: "But do i have to tell you that?"
> fn-342 RESULT.md: "At 40, 1,025 of 1,382 such forks stand where the bearer's growing tip had stopped."

## Goal & Context

Child of goal fn-321. fn-342 measured what makes today's oak jagged: on wood 1 cm or thicker, 67 to 74% of the joints turning 10 degrees or more are a lateral taking over its bearer's line, turning by its full insertion angle of 34 to 57 degrees, and three quarters of those stand where the bearer's tip had stopped. A limb's own relays turn 9 to 15 degrees and read as smooth. The host's read of the leafless stills agrees: the jaggedness the eye sees is hard elbows on branch wood, with straight runs between them. [paraphrase]

In a tree that grows this way the shoot that carries a line on after its tip stops does so nearly in line with it, and the wood laid over the junction in later years buries what angle is left. The grower already has that mechanism for limbs and boughs (`relay`, `takeover`, the relay's frame). This child finds why branch wood does not use it and makes a line that changes hands carry on as one. [inferred]

## Acceptance Criteria

- **R1 (why the tip stops and what takes over, and back to the host):** on today's oak at 40 and 120, seeds 1001 and 1007, for the take-over joints fn-342 counted: what stopped the bearer's tip (its life ending, the balance, shade, a draw, another cause named from the code), with each cause's share; which lateral took the line and by what rule it became the thicker; and why the limb and bough mechanism (`relay`, `takeover`) did not apply. With it, on paper, the change the host's direction implies (below), the settings it touches, and its expected effect on the count of joints of 10 degrees or more per metre. No code is built before the host answers. [inferred]
- **R2 (the probe):** on the host's word, the change is built as a probe that alters the tree by degree and is dormant at a neutral value. [inferred]
- **R3 (the read):** leafless and leafed stills, today's beside the probe, the oak at 40 and 120 on seeds 1001 and 1007 at the whole and mid views, and every other preset at 40 on one seed. fn-342's kink table again under the probe; height, spread, trunk and current-year leaf beside today's; one-thread seconds. The worker views every still and says plainly whether each probe tree is better, the same or worse than today's, and why. [inferred]
- **R4 (the host judges first):** the result goes to the host, who views every sheet. Nothing goes to the owner unless the host finds the probe's trees better than today's on every species shown; a species it harms is a failed read, reported as one. [paraphrase]
- **R5 (verification):** the probe leaves the crates as a patch in evidence; where any code stays the gate is green and the codex review ends SHIP. [paraphrase]

## Boundaries

- Today's structure is the baseline: girth, sag, the balance and the foliage are not changed here. [paraphrase]
- No value fitted per species and none tuned to a sheet. [paraphrase]
- The oak at 300 is not this child's. [inferred]
- Heavy work runs through `telperion-heavy`. Nothing merges to master, nothing is tagged or published.

## Decision Context

The host's direction, to be checked by R1 and not yet a rule: where a tip stops and a lateral inherits its line, that lateral is the line's continuation and takes the continuation's geometry (the relay's frame and the take-over's pull back onto the line) at every rank, in place of standing at its insertion angle for good. The host does not know why branch wood's tips stop so often (the oak's branch row has abortion 0), which is why R1 comes first. [inferred]

Settled by the owner's verdict on fn-341: a measure moving toward a published range is no evidence a tree looks better; the eye judges, the host's first. [paraphrase]

## Open Questions

None.

## Result

Closed 10 Nov 2026 without a fix. The evidence holds `R1-NOTE.md`, `R2-ARITHMETIC.md`, `R3-ABLATION.md` and the ablation sheets, which the host viewed (E's leafless and leafed).

- **The title's premise was wrong.** The lateral that takes the line is a side branch born about 3 years earlier, 0.25 to 0.39 m below its bearer's end; it is the line only because the pipe model makes it the thicker. On thick wood the bearer's tip mostly has not stopped: at 12 years the branch row moves on to the shoot age (32% of the take-overs at 40, 52% at 120). fn-342's "where the tip had stopped" counted those.
- **Sharing a fork's angle by section fails on paper:** sharp joints per metre go from 0.44 to 2.20 at 40 and from 0.62 to 1.84 at 120, since a lateral at 0.55 to 0.73 of its bearer's radius is the common case and every bearer then zigzags. Murray 1927 was read in full; at the oak's pipe exponent its form gives no angle.
- **What the eye says (the ablation):** switching off limb relays, wander or sag does not make the oak read less jagged. Halving the branch and twig rows' insertion angles does, at both ages and both seeds, with height, trunk and leaf within 5% of today's, and it costs the oak its wide-angled twiggy look. That variant is an instrument and not a candidate.
- **How a take-over is drawn:** a relay stays in its parent's run and its corner is rounded; a lateral always starts a run of its own, even when it is the thicker wood past the joint, so its insertion angle is drawn as a hard corner between two tubes while the thinner bearer runs on smoothly as the line (`surface/paths.rs`).
- **Open, for a later child:** a rule for how wood that leaves at a wide angle goes on from there. Today it holds a straight line from a hard corner. The two facts above are where that child starts.
- R2's probe was not built and R3 to R5 did not run, for that reason. No code stays; no gate and no review ran.

## Friction

- **The first walk logged only the wood it took** (6 minutes for a second walk). `probes/fork.patch` logs every wood leaving a walked node; a later child reuses it.
- **A variant four times today's wood was grown before the device refused it** (2 minutes). A census that stops a run outgrowing today's is written into fn-344's R2.

