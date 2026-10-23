# fn-272: the release film, its README embed and the owner's verdict

The film is `crates/telperion-render/examples/film.rs` rendering `crates/telperion-render/examples/film/release.json` (R1, R1a, R1b, R2). How every cut was made and what was seen in it is in FILM.md; friction is in FRICTION.md.

## The cut the owner accepted: cut 6

Rendered on the film branch at c4bc857c2 (the release branch with fn-309 merged) with `telperion-heavy 8 --gpu 4 --threads 4 -- cargo run --profile ci -p telperion-render --example film -- crates/telperion-render/examples/film/release.json <evidence>/raw/cut6`.

- **Cost:** 35.0 s, 1,050 frames, 1280x720 at 30 fps, 31 MB, 36.2 min wall. No frame refused. The largest builds were Laurelin to Telperion at 1,214.7 s (peak resident 4,918 MB) and spruce to birch at 340.6 s. The device peaked at 4,228 MB, while the Laurelin grows. The per-segment table is in FILM.md, "Cut 6".
- **Oracle (R3, fn-251's band on every walk frame, 22.5 min):** 481 of 510 frames meet it. Every frame of beech to spruce, spruce to birch, oak to palm, palm to Laurelin and Laurelin to Telperion meets it. Birch to oak meets it at 61 of 90: from t = 0.20 to 0.9985 the kept count is up to 40.8 % over its ceiling of 487,001, against fn-251's named exception of 21 %. The owner accepted this in motion (below). Every frame's counts equal cut 5's.

## Videos (outside the repository)

| file | what |
|---|---|
| `/home/daniel/Videos/telperion/cut5.mp4` | cut 5, 31,978,377 bytes |
| `/home/daniel/Videos/telperion/cut6.mp4` | cut 6 as rendered, 31,324,321 bytes |
| `/home/daniel/Videos/telperion/cut6-readme.mp4` | cut 6 for the README: H.264 at CRF 30, no audio, 35 s, 6,865,516 bytes; the owner's upload |
| `/home/daniel/Videos/telperion/cut6-poster.jpg` | the poster still, the oak at 40, 52,624 bytes |

The owner uploaded the last two to PR #159 as GitHub user-attachments: the video at https://github.com/user-attachments/assets/47c668ef-1174-4dfc-9d89-cf11f4834613 and the poster at https://github.com/user-attachments/assets/3b654cc5-e7c4-44c4-b32f-c46007fd703f. Both resolve without signing in: a ranged GET returns 206 with `video/mp4` of 6,865,516 bytes and `image/jpeg`.

## R5: the owner's verdicts

- Cut 3 (2026-10-19): "the idea i think is right just some execution flaws".
- Cut 4 (2026-10-19): "mostly good. As we transition to the palm tree there's some flying leaves that don't make sense."
- Cut 5 (2026-10-22): "i thought i still saw bare/flying leaves in the palm". fn-309 fixed this.
- Cut 6 (2026-10-23): **"looked good"**, which covers the birch-to-oak stretch. It is recorded in the spec's Decisions at 36c77b4eb and settles R5.

## R4: the embed

README.md, under the one-line description:

1. The video's user-attachments URL on a line of its own.
2. A paragraph giving the film's closing line, "Every frame grown live by the engine", the path through species space with its growing and shrinking, and what the corner map is: species space on its first two principal components, which hold 62.9 % of the variance (cut 6's map, `raw/cut6-render.log`). This is R1b's variance share.
3. The same paragraph ends with the poster still, linked to the video, as the fallback where the player does not show.

Nothing is committed to the repository but text, so CI's size budget is untouched.

**The claim "GitHub renders a user-attachments URL standing alone on a line as an inline player" holds.**

- GitHub's changelog (https://github.blog/changelog/2021-05-13-video-uploads-now-generally-available) says video upload is "supported everywhere you can author Markdown in GitHub", including "repository Markdown files such as READMEs". Neither it nor the docs (https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/attaching-files) say how the video displays. The docs give the limit as 10 MB on a free plan and recommend H.264; the README cut is 6.9 MB H.264.
- What settles it is GitHub's own renderer: `POST /markdown` (mode `gfm`, context `DanielKillenberger/telperion`) on the README's first nine lines. It returns one `<video controls muted>` for the bare URL, and an `<a href=video><img src=poster></a>` inside the paragraph for the still.

**Found while checking:** a linked still on a line of its own is also rendered as a video player, whether written in Markdown or as an HTML `<a><img></a>`, because its link is a user-attachments video. Written that way the README would show two players. The still therefore sits at the end of a sentence, where GitHub keeps it an image. On GitHub the viewer sees the player and, under it, the small linked still. Dropping the fallback, or linking the still elsewhere, is the owner's choice.

**npm:** the package publishes README.md (`npm pack --dry-run` lists it). npm's renderer does not turn a video URL into a player. It should show the bare URL as a link and the poster as an image linked to the video. This is unverified: npm's renderer was not run here, and its first chance to show the page is the next publish.

**crates.io:** no crate sets `readme` and no crate directory holds a README.md, so crates.io shows none of this.

**Tests:** no test or check reads the root README. `catalogue:check` reads `catalogue/*/README.md`; `test-dist.mjs` only names the README in a comment. So `npm test` was not run, and the change is docs only, so no gate was run.
