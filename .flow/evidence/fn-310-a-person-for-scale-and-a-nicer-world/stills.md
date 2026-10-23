# Final stills (fn-310 task 3, R5)

`final-stills.jpg` holds four stills at branch `6d73657e5`: the oak at 40 and Telperion at 80, seed 7, at 1280x720. Each tree is drawn under the default row (variant C: sun 135 degrees, 30 degrees up) and under the film's sun (115 degrees, 60 degrees up, the sun on the owner's concept sheet). The single PNGs are under `raw/stills/final/`. I viewed every still.

Each still is one command, run through `telperion-heavy 5 --gpu 3 --threads 2`, with `age-40.json` holding `{"age":40}` and `age-80.json` holding `{"age":80}`:

```bash
cargo build --profile ci -p telperion-render --example headless
target/ci/examples/headless --preset oregon-white-oak --seed 7 --size 1280x720 --family age-40.json --out oak-40-default.png
target/ci/examples/headless --preset oregon-white-oak --seed 7 --size 1280x720 --family age-40.json --scene '{"sunAzimuth":115,"sunElevation":60}' --out oak-40-film.png
target/ci/examples/headless --preset telperion --seed 7 --size 1280x720 --family age-80.json --out telperion-80-default.png
target/ci/examples/headless --preset telperion --seed 7 --size 1280x720 --family age-80.json --scene '{"sunAzimuth":115,"sunElevation":60}' --out telperion-80-film.png
montage -label '%t' -pointsize 18 <the four> -tile 2x2 -geometry 960x540+4+4 sheet.png
magick sheet.png -quality 82 final-stills.jpg
```

What the stills show:

- In all four the person stands to the camera's left of the trunk, outside the crown and its shadow.
- Under the default sun the shadow falls to the left and behind the trunk.
- Under the film's high sun it pools under the crown.
- The sky glows toward the sun, and the ground carries soft tonal patches that fade into the horizon haze.
- At Telperion's framing the person is small beside a 46 m tree. `scale-check.md` measures its projected height.

## The person's shadow (task 5)

`person-shadow.jpg` holds the oak at 40 and Telperion at 80, seed 7, 1280x720, under the film's sun (115 degrees, 60 degrees up), drawn at `a192d41bd` with the same `headless` commands as above and `--scene '{"sunAzimuth":115,"sunElevation":60}'`. The bottom row enlarges the person in each still (3x on the oak, 6x on Telperion, nearest-neighbour). The single PNGs and the full-quality sheet are under `raw/stills/` (`person-shadow-*.png`, `person-shadow.jpg`). I viewed every still.

- Under the high film sun the person's shadow is short and falls away from the camera, to the right of its legs and behind them.
- At Telperion's framing the person is about 8 px tall, and its shadow is a few pixels beside its feet, visible only in the enlargement.
