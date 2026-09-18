# Friction reports, fn-82 (date palm as a real species)

Entries follow the friction rule in CLAUDE.md; the owner decides which become specs.

## 2026-09-18 23:32 discover needs a seed the skill does not name

- **Doing:** starting skill step 2, `discover` for `date-palm`.
- **Hindered by:** `species-pipeline discover` reads `DIR/manifest.json` and will not start without it, but `.claude/skills/add-species/SKILL.md` step 2 says "run discover, read its proposal, draft DIR/manifest.json" and never says to write the seed first. The seed shape (species, taxon, two fields, no sources) lives in `crates/telperion-jev/tests/pipeline_fixes.rs` as a comment on the 2026-09-18 ash run, and the admitted ash manifest is on another branch. This worktree's `DIR` held only an empty `driver-1.log`.
- **Cost:** about ten minutes of reading the discover stage, the ash test fixture, and `git show fn-56-european-ash-as-a-real-species:.../manifest.json` before any Firecrawl call.
- **What would remove it:** the skill's step 2 names the seed write, and a one-file seed template sits next to the runbook (species, taxon, the two growth fields, empty sources), so a cheap agent copies it instead of reconstructing it from a test.
- **Early return:** not taken; the seed is now written and discover can run.

## 2026-09-18 23:33 known sources and Wikipedia force a hand edit of the draft

- **Doing:** reading `discover.json` and drafting `DIR/manifest.json` from the proposal.
- **Hindered by:** each field's candidate list opened with 11 known hits (oak, spruce, fn-11 growth papers) that cannot be date-palm evidence. Jev then ranked Wikipedia first for `height_m`, which the species spec forbids as a citation. The research index for `dbh_m` returned avocado biochar, an insecticide paper, olive radiocarbon, and a Canary Island date palm page (`Phoenix canariensis`). The machine draft therefore named Wikipedia and the UA arboretum; a person still has to drop Wikipedia, skip the wrong taxon, and pick IFAS plus the Al-Madinah morphology paper by hand.
- **Cost:** the discover itself was 10.6 s; the hand edit was a few minutes of reading 42 hits. No extra credits.
- **What would remove it:** known sources filtered by taxon before ranking; a citation-policy prior so Wikipedia cannot be `ranked_first`; research queries that drop social posts and a sister species.
- **Early return:** not taken; the edit is the skill's draft step and the credit spend was already done.

## 2026-09-18 23:45 gate examples are not in the worktree binary set

- **Doing:** about to run `gate --example` after verify.
- **Hindered by:** `target/release/species-pipeline` is present, but `target/release/examples/species_measure` and `geometry_benchmark` are not. The runbook lists those two example builds before any stage; the previous driver only built the pipeline binary. Gate cannot run until they exist.
- **Cost:** 22.5 s wall clock for `cargo build --release -p telperion-core --example species_measure --example geometry_benchmark`. Incremental; core recompiled. Zero Firecrawl.
- **What would remove it:** the species-pipeline build script, or the skill's "build once", also builds the two examples the gate calls. A cheap swarm agent that is handed only `species-pipeline` stops at gate for a missing binary, not for a palm capability.
- **Early return:** not taken; the build is the runbook's listed prerequisite, not a workaround.

## 2026-09-18 23:47 gate passed capability because the required list was empty

- **Doing:** reading `gate.json` after `gate --example`, to name the capability line for the host's gap loop.
- **Hindered by:** `required`, `supported` and `missing` are all `[]`. The spec already names Corner and a frond; the admitted manifest has `growth_form: palm` and no `engineering.required_capabilities`. Gate only compares that list, so it never asked whether the generator draws a Corner palm, a frond, or a trunk without secondary thickening. The halt is `onboarding-gate/registry` and `onboarding-gate/seeds`. Seeds cannot exist until generate, which the same decisions block.
- **Cost:** none in credits. A few minutes reading `gate.rs` and `required_capabilities` so the report does not invent a capability line the artifact does not carry.
- **What would remove it:** the seed or the spec's model and organs become the required list before discover, so a palm files `onboarding-gate/capability` with a named line (`architectural-model:Corner`, or `frond`, or `no-secondary-thickening`) and a cheap driver can hand that id to the host.
- **Early return:** taken on the gap loop, as this leg's brief requires. The two `onboarding-gate` ids are recorded. The form gap is not among them.


## 2026-09-18 — host: the capability vocabulary cannot express a form gap

- **Doing:** taking the gap loop from the two `onboarding-gate` ids the driver
  recorded, to route the palm's form gap.
- **Hindered by:** there is no form gap to route, and there cannot be one yet.
  Two things compose. First, `required_capabilities` reads the generate
  stage's `species.json` when it exists, else the manifest's `engineering`
  entry, else nothing; on a first run neither exists, because gate blocks
  generate and nothing derives the entry from the species spec, so `required`
  is empty. Second, and deeper, `supported` comes from
  `geometry_benchmark --support <preset>`, which calls `Preset::from_id` and
  returns `{"implemented":false,"capabilities":[]}` with exit code 0 for a
  preset that is not registered. So both lists are empty, `missing` is empty,
  and capability passes silently for every new species.
  Even filling the required list would not reach it. The vocabulary is
  `woody-axes`, pushed unconditionally for every preset, plus `lobed-blade`,
  `four-sided-needle`, `alternate-petiole`, `radial-peg` and
  `tiered-secondary`, each derived from a threshold on an existing preset's
  own parameters. Those are leaf and attachment descriptors of a branching
  woody tree. There is no term for an unbranched stem, for Corner's model, for
  a trunk without secondary thickening, or for a frond, so a palm's actual
  needs cannot be written down in it. The check answers "does this registered
  preset's value table produce a lobed blade", not "can the generator draw this
  form".
- **Cost:** the whole literature chain across two legs, 9 estimated Firecrawl
  credits and 40 Jev calls, to reach a gate that could not ask the question the
  species was chosen to ask. The gap loop itself is sound and opened the gap
  record correctly; it has nothing true to route.
- **What would remove it:** one spec, proposed and not written. The supported
  set has to describe the generator rather than an already-registered preset,
  the vocabulary has to carry architectural form and organ terms beside the
  leaf descriptors, and the required list has to be derived from the species
  spec's stated model and organs at seed time, before the literature stages.
  fn-35 already records that no architectural-model coverage file exists and
  that the 23-model list becomes its own spec when a species first names an
  unsupported model; the date palm is that trigger.
- **Early return:** taken. The loop is not run on the two bookkeeping gates,
  because options for "register the preset" would mint a spec to do what fn-82
  already exists to do, and `seeds` only says that `generate`, which the same
  gate blocks, has not run.

## 2026-09-19 — capability assessment is a source read, and the laterals rail lies

- **Doing:** the CAPABILITY ASSESSMENT stage the runbook skipped, for
  Phoenix dactylifera, writing `engineering.required_capabilities` and
  `packet/capability.json`.
- **Hindered by:** the stage is in `docs/species-onboarding.md`'s handoff
  table and is not one of the eleven pipeline commands, so walking the
  runbook never stops on it. The habit table's `laterals_per_station`
  rail is 1..=12, which reads as "a stem that bears no laterals is
  unreachable" until `scaffold.rs` is opened and `lateral_orders=0` is
  seen to skip `station()` entirely. A cheap driver that only reads
  `validate()` files a false `unsupported-anatomy` on the unbranched
  stem. `capabilities()` still only derives six names from a registered
  preset, so every name we write is missing, including `woody-axes`,
  because `date-palm` is not registered.
- **Cost:** the reading, not credits. Gate itself was 0.256 s and 0 Jev.
  The earlier literature chain remains the wasted spend this stage should
  have sat in front of.
- **What would remove it:** a pipeline command, or a seed-time derivation
  from the spec's model and organs, so the required list exists before
  discover. A one-page trait-use note next to the rails
  (`lateral_orders=0` means no laterals; `laterals_per_station` is inert
  then) so the next cheap driver does not re-read `scaffold.rs`.
- **Early return:** not taken. The assessment was the assigned work and
  finished. The new `onboarding-gate/capability` decision was left for
  the host.
