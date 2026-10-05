# fn-205: girth from secondary growth by share

## The established radius, as built

The spec's `established_radius` is "the radius the phytomer had when it was laid down at the apex (its pipe radius at its first cycle)". At the moment the apex lays a phytomer down it carries nothing above it, so its pipe radius then is its own pipe: `pipe * scale * ripe(1 year)`. That is what `girth.rs` uses. The other reading, the pipe-model radius at the end of its first cycle counting the rest of that year's growth unit above it, would give a stem at `secondary` 0 a yearly sawtooth: the base of each year's unit of 12 phytomers would be 12^(1/4) = 1.86 times its top, at exponent 4. The load carried on to the parent stays the pipe model's section at every share.

## R1: neutral at 1, every existing tree byte-identical

The engine structures of the beech, the spruce and the palm values at ages 10, 40 and 80, seeds 1 and 7, written as debug text and hashed (a scratch test, removed), are identical before and after:

```
DIGEST beech 10 1 59be7d5712c70431
DIGEST beech 10 7 2438a7b3ead8b7ce
DIGEST beech 40 1 e3cf0c1dcd6f8c41
DIGEST beech 40 7 4d26b542a4d410f2
DIGEST beech 80 1 b6844ecf2d882c9e
DIGEST beech 80 7 ad992c865a59f8aa
DIGEST spruce 10 1 8f4056652f7ddbed
DIGEST spruce 10 7 c93cd09fe7d0c39f
DIGEST spruce 40 1 0ec7bcb20e1046a1
DIGEST spruce 40 7 ca82e7f1d27c549a
DIGEST spruce 80 1 5bfec966ebef3f67
DIGEST spruce 80 7 b8bd74671bfc7ca6
DIGEST palm 10 1 551ec9c40b16e89c
DIGEST palm 10 7 a6efd65dd9ae826a
DIGEST palm 40 1 f5d307e16a333581
DIGEST palm 40 7 4ae658ade90310c2
DIGEST palm 80 1 e62f5a6448981503
DIGEST palm 80 7 37048580f3ccd06e
```

A's oracle tests and B's walk tests pass unchanged (`cargo test --profile ci -p telperion-space`). The beech and spruce stills are drawn from these structures, so they are unchanged.

## R2

`tests/form.rs`, `a_stem_without_secondary_growth_is_a_column`: red first (no field), then green. At 0 every phytomer's radius is its established radius and the base over the apex is 1; at 1 every radius is the pipe model's sum.

## R3

`states[p].form.secondary` is walked from 0 to 1 at every PA of the walk tree, and girth moves no shape there (slope 0.00: the walk tree does not sag). A girth reaches the shape only through sag, so a fourth walk, `states[1].form.secondary, sag 1e-4`, walks it on sagging limbs: steepest slope 9.53 per unit (seed 1 at 0.295), under the bound of 30, and the finer split shrinks (no jump).
