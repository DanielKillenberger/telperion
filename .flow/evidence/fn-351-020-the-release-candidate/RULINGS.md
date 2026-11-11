# The owner's rulings that bind 0.2.0

Checked by the host before any rule is designed and before any child is minted. Quotes are the owner's words as the specs and logs record them; "(P)" marks a paraphrase recorded in a spec or AGENTS.md. Where a ruling was later replaced, the line says so. Source: the spec or file named in brackets.

## What the product is

- **13 Oct 2026:** "with this algo we want growth to be part of the product. It's great that we botanically grow trees. This should be locked in." [AGENTS.md]
- **31 Oct 2026:** "It doesn't look improved when integrating in into my website in that style, performance dropped. It doesn't seem worth releasing." and "no i want 0.2.0 to be strictly better. It should be efficient to use in a website coarse environment." [fn-321, fn-237 STATUS]
- **9 Oct 2026:** "I would really like to ship something that is at least as good as the previous version." and "if we get way better fidelity for slightly less performance that's a trade we can live with. But as of now performance is way worse. Hard to justify." [fn-237]
- **1 Nov 2026:** "The problem with the oak being mostly that we cannot do old enough trees. They need to be taller." [fn-321]
- **3 Nov 2026:** "But then are we able to realistically grow 300 year old trees? That's really the goal here." [fn-321]
- **9 Nov 2026:** "I want highest fidelity/performance ratio tradeoff. That's what we're maximizing. Realistic trees generated and rendered fast. That's the goal. Strategy states the goal also about many trees in game engines etc." [fn-321]
- **9 Nov 2026:** "no i don't think we need to get back to the look of .1.4 they look a bit better because 0.2.0 looks bad. But .2 is better conceptually with the botanically accurate way of growing it just needs to be even better and grow larger trees more efficiently" [fn-321]
- **11 Nov 2026:** "speed i really think we should aim for better but in a way that doesn't get us stuck if it can't be reached with the approach we're doing now" [fn-351]

## How it is built

- **One grower, on the CPU; the GPU grower is deleted.** 1 Nov 2026: "no we'd replace the current grower with the new one..? we only want one". 1 Nov 2026 (P): growth on the CPU for every visitor, choice A. 10 Nov 2026 (P): accepted the approach with the device grower deleted. 11 Nov 2026: "i thought we'd remove the gpu grower?" [fn-321, fn-323, fn-347]
- **Nothing per species, nothing hand tuned.** 3 Nov 2026: "I think it's problematic that we're making core settings per preset. This should be a generalized solution that works across the tree space. This shouldn't be per species." 9 Nov 2026: "i don't think we want hand tuned rules. It's going against the engineering principles". 14 Oct 2026: "but are we making up a rule for a specific species? that'd break our design rule no?" [fn-321, fn-282]
- **No artificial limits.** 3 Nov 2026: "remove the shoot limit everywhere.. We don't want artificial limits like that i've said that before" [AGENTS.md]
- **The dead-wood bug is fixed properly.** 10 Nov 2026: "ok well then we need to fix this bug properly. And we don't need byte identical trees to come from the fix." [fn-347]
- **Leafy shoots may be counted.** 10 Nov 2026 (P): accepted that the leafy shoots stop being stored state and become a function of the structure, and that every preset is re-derived on sourced traits. 10 Nov 2026: "3x cheaper is pretty good already". 11 Nov 2026 (plan, section 4 item 3, approved): counted shoots are the growth and not a shortcut; AGENTS.md's 13 Oct sentence on shortcuts is reworded to say so. [fn-321, PLAN-2026-11-11.md]
- **Nothing prebaked; no network for 0.2.0.** 13 Oct 2026: "I just don't want prebaked 3d models." and "i have no problem baking things offline that ship as part of the engine". 9 Nov 2026: "yea i think we need a base generator that works before we can think of traininga a networK?" [fn-268, fn-321]
- **Parameters, never names.** 25 Oct 2026: "We shouldn't hand a species name to the generator and if it's not typed we can't do it." and "Remove everywhere" [fn-317]
- **A path to engines, no plugin.** 1 Nov 2026: "what we're builing must have a modular path to build a plugin or smth to be imported by game engines to generate trees at scale." and "Any imports to game engines are out of scope for this goal." [fn-321]
- **The website downloads at most 250 KB.** 1 Nov 2026, selected: "At most 250 KB" [fn-321]
- **One pipeline.** 5 Oct 2026: "In the end we want one pipeline. So if we choose f1-f3 the old one gets removed." [fn-198]
- **Mantra (8 Sep 2026):** "Minimalist af, efficient af and beautiful" [AGENTS.md]

## How the work is run

- **Who does what.** 9 Nov 2026: "you orchestrate the goal build and have opus do the build work. They should escalate to you for questions." 11 Nov 2026: "i think what you could also do is judge where we need fable intelligence." [AGENTS.md, fn-351]
- **A result does not end a run.** 1 Nov 2026: "why would a result end the run. You should try and achieve a goal and stop only when stuck to where it doesn't seem achievable in the way the goal was initially planned/needs major rethink." [AGENTS.md]
- **Not a patch loop.** 10 Nov 2026: "ok follow your recommendation. Just don't get stuck in a patch the current issue mode. Think high level how we can achieve our goal and make sure we move that direction." 9 Nov 2026: "Don't get bogged down by previous attempts. Don't just make another iterative step." [fn-321]
- **The host judges the look and says so.** 10 Nov 2026, on sheets the host had described neutrally: "today's ok at 120 isn't that bad honestly. visually the new ones are a regression. Today's is a bit jagged but it looks directionally correct. The jaggedness is worse at 40." "every probe is worse than today" "But do i have to tell you that?" 11 Nov 2026: "i want you to visually gauge if it's roughly same visual fidelity or better ideally" "and then evaluate what's wrong" "use jev for specific questions about faults/features of images" [fn-321, fn-347]
- **The host fixes base failures itself.** 1 Nov 2026: "ok pls fix and yes update the rule so you can judge and implement fixes by yourself" [AGENTS.md]
- **Verification is local and light.** 1 Nov 2026: "We don't need to run ci on this branch on github either. Run verification locally. Keep it light and focused. Don't rerun shit that's not been changed etc." [fn-322]
- **Autonomy.** 11 Nov 2026: "I want you to run there autonomously ... I only want you to return to me if you're genuinely stuck." [fn-351]

## The release

- **Nothing merges, is tagged or is published without the owner (P).** [fn-237, fn-313] Prereleases only on his word: "Only when i ask we should release nightly i think?" (24 Oct 2026) [fn-313]
- **A proper release.** 15 Oct 2026: "no let's do a proper 0.2.0 release with this shipped". 27 Oct 2026: "i think we should fix it. We want high quality 0.2.0" [fn-237, fn-319]
- **The film.** 13 Oct 2026: "we can do it with release and embed it into the repos readme while we're at it". 24 Oct 2026, on cut 8: "ok so i think we're good? release ready finally" [fn-237, fn-272]
- **The oak on seed 7.** 15 Oct 2026: "Oak seed 7 does look like it needs fixing" [fn-237]
- **Agents do not drive WebGPU in a browser (P).** [fn-315, fn-294]
- **The repository's size.** 24 Oct 2026, on PR #159: "it's still 250k loc added which is a bit crazy. can we do another cleanup?" [fn-311]

## Verdicts on trees, as given

- 3 Nov 2026, the oak at 200: "yea that's bad" [fn-321]
- 10 Nov 2026, the oak at 300, today's and the probes: "i mean wtf is that ok at 300..?" (oak, his correction) [fn-321]
- 15 Oct 2026: beech "But i think we can move that work to after release."; Telperion "But can accept for now" [fn-237]
- 9 Nov 2026: "They look more organic, but some of them, like the oak, they still just don't look good enough." [fn-336]

## Replaced, kept here so they are not revived

- 1 Nov 2026 "a is fine" (a coarse tree held to the full cycle's statistics): replaced by the 9 and 10 Nov rewrites; the cycle is not the reference.
- 3 Nov 2026 "we can lift the no neural net completely" and the scaffold-plus-network route: replaced on 9 Nov (base generator first) and 10 Nov (no network in the goal).
- 1 Nov 2026 "Stop at 20" children: replaced on 11 Nov (no limit, plan section 4 item 8).
- 13 Oct 2026 "Your budgets seem reasonable" (0.5 s warm budgets): replaced by fn-321's criteria and now by fn-351's aims.
