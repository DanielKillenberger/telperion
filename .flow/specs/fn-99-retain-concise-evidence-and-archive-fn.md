# Retain concise evidence and archive fn-91 run output

## Goal and context
The owner requested a cleanup PR after fn-91 merged hundreds of evidence files. Keep the repository useful for review and reproduction without carrying every experimental run. [user]

## Acceptance
- R1: Archive removed fn-91 artifacts locally with checksums and a recovery path from their original Git revision; retain final reports, decisions and verification scripts with their required inputs. [paraphrase]
- R2: Remove bulky run output from the working tree without changing engine code, existing test fixtures, or unrelated captured specs. [paraphrase]
- R3: Document and enforce a retention convention that keeps future raw evidence out of normal commits; check retained references and open a cleanup PR. [paraphrase]

## Boundaries
No Git history rewrite. No demo media added. The local archive is not a durable shared hosting service; the existing Git revision remains the portable recovery source. [inferred]
