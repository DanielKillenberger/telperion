# The film's draw time: where cut 7's +51 ms a frame goes (fn-310 task 6)

**Cause.** The extra time is spent writing the PNG, not drawing the frame. Cut 7's frames compress worse, so the film's `write_png` (png 0.18.1, the encoder's defaults) takes about 53 ms longer per frame. Most of that comes from the sky's dither (D4): it shifts every sky pixel by its own hash, so PNG's row filters can no longer predict the sky. The ground's mottle and the haze add about 5 ms. Render and read-back time is unchanged within noise. The sun's map is redrawn exactly as often as before, and the number of renders is the same. So no per-frame cost was found in the shipped renderer. The cost sits in the film's own write path, and the trigger is the shipped renderer's output: a dithered sky.

## How it was measured

- **The film path, not the frames probe.** `examples/film` gained `--frames <list>`, which draws only the frames named, through the same code. Its cost line now splits draw into two parts. The first is render: `render()` with its read-back, plus how many renders ran and how many drew the sun's map rather than reusing it. The second is write: crossfade, overlay and `write_png`.
- **Three sides, each built in the `ci` profile under its own target directory.**
  - base: `201288747` (cut 6's renderer; `c4bc857c2` differs from it in one test file only), a fresh `git archive` in scratch.
  - HEAD: `83eab841a` (the renderer is `c047afedb`'s, cut 7's).
  - nodither: HEAD in a scratch copy, with the sky's `(1.0 + offset / 96.0)` multiplied out to `1.0`. This is a measurement build only, never committed.
  - The film example is identical on all three sides; only the cost line's patch was copied in.
- **Machine and settings.** RTX 3080, driver 610.57.04, Vulkan. Each run went through `telperion-heavy 5 --gpu 3 --threads 2` with `__GL_SHADER_DISK_CACHE=0`, one run at a time. The order was rotated each round.
- **The stretch.** Segment 0 (beech grown 1-40), frames 60-119: 60 frames, 12 trees, 75 renders (15 crossfade frames draw two years).
- **Where the rows are.** `raw/film-draw/` holds the scripts (`ab.sh`, `ab3.sh`, `parts.sh`, `nodither.sh`, `pngtime.rs`) and every run's log.

## Numbers

Segment 0, frames 60-119, shader cache off. The first table is the rotated round at 10 ms precision (`s0c`); seconds are over the 60 frames.

| side | render (75 renders) | sun maps drawn | write | draw |
|---|--:|--:|--:|--:|
| base | 1.09, 1.06, 1.09 s | 24 | 1.44, 1.49, 1.45 s | 2.5-2.6 s |
| HEAD | 1.05, 0.99, 1.00 s | 24 | 4.62, 4.62, 4.59 s | 5.6-5.7 s |
| nodither | 1.12, 1.06, 0.87 s | 24 | 1.75, 1.75, 1.76 s | 2.6-2.9 s |

Two earlier base/HEAD rounds at 0.1 s precision agree: base write 1.4 s, HEAD write 4.6 s, render 0.8-1.2 s on both sides.

- **The rise.** Write rises by 3.15 s over 60 frames (means of the three rounds), which is +52 ms a frame. Over cut 7's 1,050 frames that is about 55 s; the film's draw rose by 54 s (87 s to 141 s).
- **Render.** Render's spread overlaps across all three sides (0.87-1.12 s, about 12-15 ms a render), so it does not move.
- **The sun's map.** It is drawn 24 times in 75 renders on every side. The person in the map's key causes no extra redraws.

**PNG encode alone** (`pngtime`: decode each written frame and time its encode again with `write_png`'s settings, in memory):

| frames | mean PNG | mean encode |
|---|--:|--:|
| base, 60-119 | 160,775 B | 20.8 ms |
| HEAD, 60-119 | 356,243 B | 74.1 ms |
| nodither, 60-119 | 198,062 B | 26.1 ms |

The encode alone adds 53.3 ms a frame, the whole write rise. The dither accounts for 48.0 ms of it; nodither still sits 5.3 ms above base. The pictures grow 2.2 times in bytes, and cut 7's mp4 is 5 % larger than cut 6's (32,908,133 B against 31,324,321 B).

**Scene parts on one still** (headless, oak at 40, seed 7, 1280x720, the film's sun 115°/60°, one PNG each, `raw/film-draw/parts/`):

| still | PNG | encode |
|---|--:|--:|
| base | 420,321 B | 38.2 ms |
| HEAD | 570,748 B | 75.8 ms |
| HEAD, person off | 566,354 B | 75.1 ms |
| HEAD, ground variation 0 | 533,950 B | 69.7 ms |
| HEAD, sky glow 0 | 570,748 B | 76.8 ms (the same bytes: the glow lies outside this frame) |
| HEAD, haze 0 | 578,392 B | 79.1 ms |
| HEAD, glow, variation and haze 0 | 515,491 B | 66.8 ms |
| nodither | 475,248 B | 46.4 ms |
| nodither, glow, variation and haze 0 | 419,886 B | 38.1 ms |

With the dither off and the three row parts at zero, the still is back to base's bytes and encode time. The person costs nothing measurable.

## Candidates, each against the numbers

- **(a) The sun's map redrawn every frame:** no. 24 maps in 75 renders on base, HEAD and nodither alike, and render time is unchanged.
- **(b) The person drawn in both crossfade renders:** the renders are the same 75 on both sides, and the person's own encode share is under 1 ms. The ghosting at 825 is a placement matter (D5, task .7), not a cost.
- **(c) Ground noise and sky at 1280x720 on the device:** render time does not move. Their cost shows up only in the PNG: the dither's about 48 ms a frame, and the mottle and haze about 5 ms.
- **(d) The film's own path:** yes. The write grows with how poorly the picture compresses.

## For the host

The cost lands in the film's PNG writer. It does not touch the harness, which encodes nothing; the harness measured +0.05 ms. Any fix is the host's call; this task changes nothing beyond measurement. Three directions were seen, none tried:

- A faster PNG compression setting in the film's writer.
- Encoding frames on worker threads while the next one renders.
- Piping raw frames to ffmpeg.

The dither itself is D4's design, and each still a reader saves is about twice as large.

## Frame selection and the before frames

- **Selection matches the full run.** `--frames 105,255,720,825` builds only segments 0, 1 and 7. It grows segment 7's tree only to the years those frames show, builds one walk tree for 255, and assembles no video. All four frames are byte-identical (`cmp`) to cut 7's full-run frames in fn-272's `raw/cut7/`, and their `frames.tsv` rows match apart from build times. The 60 frames of the timed stretch (60-119) are byte-identical to cut 7's as well.
- **Errors are refused by name before the device is requested.** For example, `--frames: "1050" is not a frame from 0 to 1049` and `--frames: "20-10" runs backwards`.
- **The before frames.** They are at `raw/film-before/frame-00{105,255,720,825}.png`, 1280x720, rendered through the film path at HEAD. The command was `telperion-heavy 5 --gpu 3 --threads 2 -- env __GL_SHADER_DISK_CACHE=0 target/ci/examples/film crates/telperion-render/examples/film/release.json .flow/evidence/fn-310-a-person-for-scale-and-a-nicer-world/raw/film-before --frames 105,255,720,825`. It built in 8.3 s and drew in 1.0 s.
