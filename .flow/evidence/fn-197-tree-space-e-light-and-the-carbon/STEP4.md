# fn-197 step 4: the carbon balance and shedding on it, 2026-10-05

The dome closed on four of five seeds at step 3d, so step 4 followed under the host's rule. It is built as designed (DESIGN-OPTIONS.md, section 5; hazard form, host decision 5).

## What was built (`src/grow/balance.rs`)

- **Each living lateral subtree's balance**, before each cycle's growth: B = (Q − C) / (Q + C).
  - Q is its buds' light: presence × light, as the allocation reads it.
  - C is its wood's upkeep: Σ upkeep(PA) × present metres. Each pencil accumulates the length its units were laid at.
  - It is remembered as M ← 0.5·M + 0.5·B. `MEMORY` = 0.5 is an engine constant with no source, which the host should rule on. Memory starts at 1.
- **The shed draw:**
  - Its yearly hazard is H = κ · (τ − M)₊, so the probability is 1 − e^−H.
  - The draw sits under the lineage's key `SHED` (u64::MAX − 1) and the cycle.
  - Where it fires, every apex in the subtree stops before it grows. The idle rule then drops the subtree, after its PA's shedding delay.
  - Where it does not fire, the draw's lead is a presence (window from the PA's expected wood). Every unit the subtree grows that cycle carries it, so a subtree about to be shed fades first.
  - The seed axis and what carries it on are never tested.
- **New per-PA settings,** all neutral: `upkeep` r (0), `balance_hazard` κ (0, never) and `tolerance` τ (0; the balance lies in −1 to 1).
- **Dormant:** with every κ at 0 nothing runs.

## Neutral

`tests/light.rs` passes: every passed species is unchanged to the bit. Their κ are 0.

## Walks

| Walk | Result |
|---|---|
| Sampled in leaf with the full lay (sag 0.83, limb survival 4.41, sleeping probability 1.01) | Green, 43 s |
| Light walks, the balance now in the leafy walk tree (upkeep 0.3, κ 1 on limbs and twigs) | Red on one item |

- **The light walks' slopes:** upkeep limbs 10.1, twigs 21.0; κ 2.75; tolerance 12.3; leaf area 20.8; extinction 12.5; sky 9.3; λ limbs 28.2.
- **The red item: the trunk's apical control, 42.0 per unit at 0.426.** It is steep, not a jump; it passes the depth-6 refine. With the balance on, the leader's vigour now also decides which laterals' balances fall.
- **This is a gate failure,** reported here and not resolved. The bound and the window are unchanged.

## Renders (oak, 80 years, sky 0.5, φ 1, ψ 1, λ 0.45, full lay on, κ 2, τ 0, under the GPU lock; I viewed every still)

Sheets in `raw/step4/`:

- `explore.png` and `explore2.png`: seed 1 at upkeep 0.25, 0.35, 0.45, 0.5, 1 and 2.
- `seeds-bare.png` and `seeds-whole.png`: the references, round 4 and step 3d, and seeds 1, 7, 2, 3 and 4 at upkeep 0.35 and 0.45.

| Seed 1, upkeep | Nodes | Leaves | Height | Width (m) |
|---|--:|--:|--:|---|
| None (step 3d) | 693k | 0.79M | 13.2 | 19.1 × 17.0 |
| 0.25 | 677k | 0.77M | 13.2 | 19.2 × 17.1 |
| 0.35 | 630k | 0.72M | 13.3 | 19.2 × 17.2 |
| 0.45 | 481k | 0.53M | 13.5 | 18.2 × 15.9 |
| 0.5 | 464k | 0.52M | 13.5 | 18.3 × 15.2 |
| 1 | 127k | 0.14M | 15.2 | 13.0 × 9.5 |
| 2 | 53k | 0.06M | 12.7 | 11.2 × 5.2 |

**Five seeds** (nodes; height × width in m):

| Upkeep | s1 | s7 | s2 | s3 | s4 | Largest ÷ smallest |
|---|---|---|---|---|---|--:|
| 0.35 | 630k | 742k | 1,013k | 906k | 552k | 1.83 |
| 0.45 | 481k, 13.5, 18.2 × 15.9 | 685k, 14.0, 15.5 × 15.5 | 761k, 14.4, 17.0 × 20.2 | 828k, 14.1, 20.4 × 15.3 | 503k, 14.4, 16.8 × 15.1 | 1.72 |

Round 4's kept phytomers were 1.95M (s1) and 2.59M (s7). The node counts here are the pipeline's nodes after dressing, so they compare only with each other.

### Reading

- **The balance bounds size, steeply.** Upkeep 0.25 to 0.35 changes little. From 0.35 to 0.5 the oak loses a quarter to a third of its nodes. At 1 and above it collapses to a pole with a top tuft.
  - The bound comes through a threshold-like turn: the whole crown's balance crosses the tolerance together.
- **R2's seeds within 1.5× is not met:** 1.72 at upkeep 0.45 and 1.83 at 0.35.
- **No clear bole forms.**
  - The oak's trunk-level laterals (fork, leader, limb) have no shedding delay (`oak.rs`). A balance-shed limb there stops but is never dropped.
  - The lower limbs that thin at 0.45 sag to the ground. Seeds 1 and 2 have a long, leafy limb lying across it: its growth was cut, so it stayed long and slender, and it hangs.
  - The trunks are short (13 to 14 m trees, decision 14's height loss), and the crowns start low.
- **The domes of step 3d survive at upkeep 0.35:** seeds 7, 2 and 3 are rounded and closed, seed 2 rounder than at 3d. At 0.45 seed 1's crown pulls in to a smaller, rounder head, and seeds 2 and 3 open toward one side.
- **Not judged a pass:**
  - R2's bound is unmet.
  - The light walk is red on the trunk's λ.
  - No bole forms.
  - Limbs lie on the ground at 0.45.
  - The height is still 30 to 35% below round 4's.

## For the host

1. **The trunk's λ walk (42.0 at 0.426) fails the bound with the balance on.** The choices:
   - narrow λ's walked range;
   - tame the bias's compounding;
   - or accept λ as a species constant that is not walked.
2. **The bole needs the trunk-level laterals to have a shedding delay** (a species value the oak's branch owns), or a balance-shed limb to be dropped regardless of its PA's delay. That is a design call.
3. **R2 (seeds within 1.5×) is unmet at 1.7 to 1.8.** Seed-to-seed spread was already large in round 4: leaves 0.87M to 3.51M.
4. **`MEMORY` = 0.5 has no source.**
5. **The height loss from decision 14 (λ 0.45 with mean sizes)** is still open.
6. **Limbs lying on the ground** when a lower limb's growth is cut but it is not shed.
