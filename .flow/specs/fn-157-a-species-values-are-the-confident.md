## Conversation Evidence

> owner (2026-09-25): "How can it be so hard to gather some data on trees and determine average values"
> owner (2026-09-25): "why are we not searching documentation regarding the species download it all that we can find and compose the highest confidence aggregate from those?"
> owner (2026-09-25): "ok go.."

## Goal & Context
<!-- scope: business -->

A species' starting values should be what the literature agrees on, and a run should get them without a person checking each number. Today the literature stage searches the web separately for each field, admits a source only after a licence check, and lets Jev choose one quote per field. On fn-149's recorded beech run (2026-09-25) that took a trunk diameter of 4.36 m and a crown of 25.9 m from one heritage-tree page describing a single giant beech, read a leaf length as the leaf width, and left the height unsourced; the first run's only admitted source was Wikipedia. Each fix made the stage run further and none made its numbers more trustworthy. This spec replaces the stage's shape: gather everything written about the species, read each document once, and compose each value as the confident aggregate across sources. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->

**What exists, checked 2026-09-25 on the fn-149 branch (`055a414e`).** [checked]
- The literature stages are discover, fetch, extract, screen, quality, select, verify and fit, with search-again rounds per unmet field (`crates/telperion-jev/src/pipeline/stages/`). Discover proposes sources per field; a source is read only after admission, which requires a rights class (`pipeline/rights.rs`). The Czech forestry journal PDF found for the beech was refused as rights class `none` and never read.
- Select keeps one value per field from Jev's ranking of candidate spans; a contradicted value becomes the range of the sources' spans (`keep-range`) or is dropped (`unsourced`).
- The recorded beech profile (`.flow/evidence/fn-149-one-species-runner-set-stages-from-a/raw/beech-record/`) has dbh 4.36 m and crown width 25.9 m from P1 (`extension.wsu.edu/clark/.../heritage-tree`), leaf width 7.6-15 cm from P2 (NC State), and no height.

**The shape.** [inferred]
1. **Gather.** One broad search per species (flora, silvics and forestry manuals, arboreta, extension pages, papers; Wikipedia only as a lead to its references), downloading every useful document, bounded by a document count and the Firecrawl budget the preflight reports.
2. **Read and extract.** Each document is read once. Code finds every candidate span; Jev labels each with its field, its unit and its context: typical or record/single specimen, mature or at a stated age, open-grown or stand-grown, and the organ it describes. Jev never chooses between sources and never supplies a number.
3. **Aggregate in code.** Per field: the value is the median of the typical, mature values from independent sources; the range is their spread; confidence is the count of agreeing independent sources and their dispersion. A value far outside the others is set aside and noted. Record and single-specimen values are kept as the field's maximum, never as its typical value. Every field cites the sources behind it.
4. **Rights.** Reading a document to extract facts needs no licence. Only a copy stored in the catalogue does: open-licence sources are copied, the rest are cited by URL.

**Replaced.** Per-field discovery and search-again rounds, source admission before reading, the quality bars, per-field selection, and the decision kinds built around them. **Kept.** The fetch retry and pacing, Wikipedia as a lead, record and replay, the preflight, the reference photograph search, and the profile's output shape, so Start, Tune and Gaps read it unchanged. [inferred]

**Unknown.** [unknown]
- The document count and the agreement threshold that make a value confident; measured on the beech and one shipped species.
- Whether the aggregate needs per-source weighting (a flora above a nursery page) or the median over independent sources is enough.

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The beech, run from a bare seed, gets height, trunk diameter, crown width and leaf length and width from at least three independent sources each where the literature has them, and every value sits inside the range standard floras give for the species. [inferred]
- **R2:** A single-specimen or record value (the heritage beech's 4.36 m trunk) is kept as the field's maximum and never as its typical value; a test over the recorded pages shows it. [inferred]
- **R3:** A source stating no licence is read and cited; only open-licence sources are copied into the catalogue. [inferred]
- **R4:** Each field records its sources, its spread and its confidence, and Start derives the tree from the aggregate without code changes downstream. [inferred]
- **R5:** The run is recorded and replayed offline in the workspace gate, and a shipped species (the oak or the ash) run the same way lands within its catalogue ranges. [inferred]
- **R6:** The replaced stages and decision kinds are gone, not disabled; lines before and after are reported; the workspace gate and `npm test` are green. [inferred]

## Boundaries
<!-- scope: business -->

- Not the species runner's structure (fn-149), tuning, or the generator. Not photo input (fn-147). One species' values and how they are composed.

## Strategy Alignment

- Serves "Fidelity" and "The catalogue": a species starts from what its literature agrees on. [strategy:Our approach]
