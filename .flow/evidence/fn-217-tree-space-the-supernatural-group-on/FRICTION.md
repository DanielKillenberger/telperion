# Friction: fn-217

## 2026-10-08, task 3: aiming a close-up at one inflorescence

- **Doing:** the horn-form check (DESIGN.md 7.5), close stills of one raceme on a flowering beech.
- **Slowed by:** nothing names where a drawn floret stands. `Tree::blooms` lists every candidate node, flowering or not, so a camera aimed at one met empty twigs. A low flowering share does not thin the racemes but shrinks every floret (presence grows in past its draw), so a sparse test tree drew specks. Leaves stood in front of the chosen floret until they were shrunk to specks.
- **Cost:** five still rounds of about one minute each plus one 1 m 47 s wait on the GPU lock, and about 25k tokens of reading stills that showed nothing.
- **Would remove it:** a helper that returns the drawn florets' positions and sizes (the organ's instances decoded, filtered by size), and a stated note on `/space/ages/<k>/flowering` that a share under the window grows every floret in rather than thinning them.

## 2026-10-08, task 3: the GPU lock held by a sleep

- **Doing:** the review fix pass, a red/green rerun of `rim_at_distance`.
- **Slowed by:** the shared GPU lock was held by `flock ... sh -c 'echo held; sleep 2700'`, a deliberate 45-minute hold, with a `space_walk` queued behind it. My first rerun waited out the tool's 600 s limit and moved to the background, where it would have compiled the sources I was still editing, so I stopped it.
- **Cost:** about 10 minutes of waiting so far, and a red proof that had to be redone.
- **Would remove it:** a stated owner and end time on a deliberate hold (a note beside the lock file), or a second lock for short focused GPU tests.
