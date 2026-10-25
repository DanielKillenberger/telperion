# fn-293 baseline digests

Recorded at `bf84ef6ca` (the plan commits over `f8c014681`; no generator source differs), with the task-1 digest tools (artifact projections; `Tree.pipe` is not digested). Profile `ci`, never `--release`. Each digest was run twice and both runs agree line for line; every later task compares against these lines.

## Running the digests (friction fix, 2026-10-19)

Wall times on this desk, warm `target/ci`, from the run logs' timestamps; launch the long ones in the background from the start.

| digest | covers | wall time | declare |
| --- | --- | --- | --- |
| `artifact_digest`, 8 presets x 3 seeds | core: skeleton, element, wood, leaves, field (n = 36), structure; not the curve | about 8 min for the 24 runs | none |
| `generation_digest` | pipeline full request, field alone, and the mesh (`Request::mesh()`, the only CPU digest of the curve) | about 10 min | none |
| `resident::digest_tests::digest` | the device build: placed leaves, packed curve points, clusters, sections, demand | about 15 s of test after its build | `telperion-heavy 10 --gpu 6` (`--gpu 7` waited 35 min behind 2 GB of desktop VRAM) |

## Core: `artifact_digest <preset> <seed> | sha256sum`

`cargo run --profile ci -p telperion-core --example artifact_digest -- <preset> <seed> | sha256sum`, every shipped preset (`presets::CATALOGUE`) at seeds 1, 7 and 42. Runs: `raw/core-digest-base-1.txt`, `raw/core-digest-base-2.txt` (identical).

| preset | seed | sha256 |
| --- | ---: | --- |
| ordinary | 1 | `08fa343052e138ca8c5eafd6e5fe2242c08b63f9791ec3ecfaeb4ff8776e916a` |
| ordinary | 7 | `b7c1ad24c2c5d42472eb3854eda9ee734650603dfde2a5b93b1f9113cb440a20` |
| ordinary | 42 | `61f64818fd73f42fe88151311faa88773520dc3260d4d38a45aed2ccd177f649` |
| oregon-white-oak | 1 | `4d1c27a1d213b43a80bfc31e2a44f9552152e7d65dc744679750f70b617599e0` |
| oregon-white-oak | 7 | `48b88ddb13cba65ba0d2d94326c315ab2148986a2e4a51e7120a197ba8136988` |
| oregon-white-oak | 42 | `adec96d2e3d4f4136e1bae7d69f91b8df2983ce7e0b2b5cbe14b9a7c56c93a23` |
| norway-spruce | 1 | `b5959a4198b689c0189b1b23f00f14fb99217cbd4b7fb339eb0f57f5a6f71cdb` |
| norway-spruce | 7 | `93c0098a9199f91d6927eed97bee3550ce741c2f3b5adf070e67b91c985bf3b1` |
| norway-spruce | 42 | `62feb91de578071ad0750937a114008766517a0915e9554b9668d1a10245113e` |
| european-beech | 1 | `6ec943d092a3cecdd92b342062945e02922ed1f2b78d3446ff8ece9c2aeb73a4` |
| european-beech | 7 | `8a13415f4455fa1898ddfe4dc0d268ff48dab90c26562b9ba2b436bbca1b8575` |
| european-beech | 42 | `e6e824702b12712585fce39f5bec59dfcafefed3a2955b952bc347dac5b2bbd8` |
| silver-birch | 1 | `08eae0a5dea12d47ecf2b1c14cdca4bc6e6b0c2bf1576a49c4024a429aff53ee` |
| silver-birch | 7 | `72963af6ea3b65a8f7aab729c188c8e8cdb56e4ba182d08710f8b9b81c33a7e9` |
| silver-birch | 42 | `1c22a655caf0196e42203fd546b569de9042f7a27f8534e80535630458f62c81` |
| date-palm | 1 | `8d8611e1437fab00d29017a2ca46277a9da9cbe68200e72f581573027d703303` |
| date-palm | 7 | `82a684c114b51155c29606c4688f070b8cbd07cc9628be3c80c99369e02b0624` |
| date-palm | 42 | `2275ca37b88569b7b5d9accf15ec9913751e8fae3e9ff40aa0fbd5757b94e0f7` |
| telperion | 1 | `f239e333b259edd2899c8faa909c671fa3722991519edf408f20c8781f055ddc` |
| telperion | 7 | `3737fb9cde6110831dc51466b7498d6c104315e88658b262a6856fce66f6c03d` |
| telperion | 42 | `41a47a1f2dcfeb808e05dad7c6a6846b99f36b7138c24b8377a64caf056f8a1c` |
| laurelin | 1 | `23679abb7085b6aca48d40ceacf64460e38c75742584e412b710e55a223a21ac` |
| laurelin | 7 | `120dedb6c5482dd010fff69310f6da10cf88b1dccd63ab7a8ac162f05e477a76` |
| laurelin | 42 | `79cad41f9106d9ec09d636f003d640e851d9d6204f403ab816b33322e232f4c0` |

## Render pipeline: `generation_digest`

`DIGEST_SEEDS=1,7,42 cargo run --profile ci -p telperion-render --example generation_digest`: the pipeline's artifacts for a full request and for the field alone, and the mesh. Full output: `raw/generation-digest-base-1.txt` (sha256 `fb4ce911fce472c24d2dc6d42ea0a7e65da256ddf1b1e1259a865f098c8cd5a4`), byte-identical to `raw/generation-digest-base-2.txt`. sha256 of each line:

| preset | seed | line sha256 |
| --- | ---: | --- |
| ordinary | 1 | `8db22866b73c02e792c6f5da63b6fe9b2505f0bbd2b1e821e1543f74155ca0b7` |
| ordinary | 7 | `3e8bca67b9733d6500f2b29f5004e45ac4e1fcbdaae485ec4d983c56198f516d` |
| ordinary | 42 | `91e47b53829bba06588345787191bfe76befec030b15854504353d9f33864bce` |
| oregon-white-oak | 1 | `c1acc5a6e6dd20a0c608e21c084469263188d6d81ccfe5b86f02b9b725aeb2ac` |
| oregon-white-oak | 7 | `9a51f78b87702e31c25c1a9b952e668bf44da6f198878ec5be2876d7504fe654` |
| oregon-white-oak | 42 | `c59a9ee61d5f39e53b7148db4de56faac25dc11827d6d22e0ac875e4252c76bf` |
| norway-spruce | 1 | `a1cbc0beb1930e6510f18e2ff2d483aca6249aeb2c1078966214bb77016cc88b` |
| norway-spruce | 7 | `f0d01ed1f980f596a564fe4804980df4bd649be9b3a76a1344eefcdd8c9a01ad` |
| norway-spruce | 42 | `2f6bdd909a0b8a3981a13b577656a3d02e8d1c4bea4636269a444b7546a0dc55` |
| european-beech | 1 | `9f5e45885bc36a9b1ae5e4e8488c7cafd15a8eaf742c2bdaf706d903878126e1` |
| european-beech | 7 | `d4c83963eccd0e7049f45ba8752b02714ffc96046108a9f6d3ccf4e9f3f4a2bd` |
| european-beech | 42 | `79a3b632ab400316fcc3ce19442506d6500894c709d26f890f9d030cc00815f7` |
| silver-birch | 1 | `f45a24b165c4a8f2a83721364b2eb5e0f070730f5941bf047a133258a72da77e` |
| silver-birch | 7 | `6c0d9c3d5bf220f70b7482b25e744dd0b02eab360a7f5ad58e920f6c0b9c1f5c` |
| silver-birch | 42 | `a98bf419c243f79d3dd1a9a7c20ca2edf751f72c8b019799750ac732b02b4428` |
| date-palm | 1 | `0c2b1ce7719a1d9dc52c5665a38d8a832c3aa8857f0aac7a27734fe63289efec` |
| date-palm | 7 | `945ca30d7a2783d3ee92e714f5c5785d7400dcaeeba9beee57ba02bbc39eb184` |
| date-palm | 42 | `0ae40b12cd46153bbaaee5ef6172e1a847533c597b4046834771a9ddc07552ae` |
| telperion | 1 | `dab413b1733f3bebf855ba48424ad8025fbd4d0a59bcd9ad6a1bc8825544c865` |
| telperion | 7 | `5986e91ba5458293d4bdc3b04544acabad09c54267b571bb444c4631b57abe5f` |
| telperion | 42 | `247bdd16b0872c6460998718bfab9f714758d9032cca5b0d0aa7f6c19f1bfa26` |
| laurelin | 1 | `4372c2cf997feb57bd24429c2f735ca16d0eb472a4cc16d63b80b2c853594a59` |
| laurelin | 7 | `af43dffe2b89cda0c57f48a8b9de23bcd0ea6f8488278ce0f586dd170afac357` |
| laurelin | 42 | `d72fbce70b26bad2ec37054729aa0adb504b065142186619270207a26ed42495` |

## Resident (device): `resident::digest_tests::digest`

`DIGEST_SEEDS=1,7,42 cargo test --profile ci -p telperion-render --lib generation::resident::digest_tests::digest -- --exact --ignored --nocapture`: per preset and seed, what `Generator::prepare(.., Delivery::Resident)` hands the renderer, FNV-1a hashed: the placed leaves (`read_instances`), the packed curve's point words (with the count), cluster words (with the count), section words (less a zero tail) and its demand. Lines: `raw/resident-digest-base-1.txt` (sha256 `c334223b9dc265eca679f69dea351c2fce238ca29c0408dc7df82b7ca7ec042e`), identical to `raw/resident-digest-base-2.txt`; test logs beside them. sha256 of each line:

| preset | seed | line sha256 |
| --- | ---: | --- |
| ordinary | 1 | `1261e645fd3ea0d8371d30ffd50ee6c3f5c6ac4d0c266f3c5e7fad9819202b9d` |
| ordinary | 7 | `dbee0af069260a3ee47e6158885bf6c23bf2abed29bc3e923f885f1d7ebb9d37` |
| ordinary | 42 | `b08af7716fedbc974fc40f0cabf8202b0bb999622b3a9354a763e9716a47972f` |
| oregon-white-oak | 1 | `50bd0ba3673b335fbf7b89fc9341f42a0675a7dca0000bc02bd9dc4dc2b9e3c1` |
| oregon-white-oak | 7 | `9a2394d38bd34412beb4b4fd90fc22cff27b3c81904657665e3660dd8686d1ad` |
| oregon-white-oak | 42 | `04f055f3844193e81eeed83a4e17ff84f22d6c1ce82af544eea5ba1fca13836e` |
| norway-spruce | 1 | `08bfc36105f94e6823628fe09f054b068a98dbb1e038f4431f38afbacc8e7490` |
| norway-spruce | 7 | `9d21ae3f1014a5a912f1912a21e484c98f049eb52549e592ed8d21dac7d4d3a6` |
| norway-spruce | 42 | `ceacce8082909c81681bae7699e0f2f337891fa7ed9ca1c2bf8bc0ce2a4ca1fb` |
| european-beech | 1 | `a94d8415bcc0cb059506488120008d020567548025ce22a7da9b9aa60ce9f264` |
| european-beech | 7 | `fa5f2401c95989234acbcf7907b10da3c4b31f4db13394f63beb198a1127934a` |
| european-beech | 42 | `9768f531aad78e27f9194414a56141aab00cfed92018ea93d92466202d1a2718` |
| silver-birch | 1 | `8b8a5daeef327797059222c4a6cc9a453c4d435b73d5d8f244886a6d0d8086f2` |
| silver-birch | 7 | `6e5c933b61ac173727a44d40c54ed495e209a6616d2a06a1cdaad2e3aaf44933` |
| silver-birch | 42 | `8a2aaf908ff5a16e480bd4f60c289b386343bf4678444af773d7d30296f53c75` |
| date-palm | 1 | `5c91ea769c4fa5a694ebd3f24e0d3367b84d42a7434fa1174026026e673c6bed` |
| date-palm | 7 | `f5e19a921168130abdba49c2679465c42965617c01e42d249d9156d67ffb10cb` |
| date-palm | 42 | `f62ca76078e587ec1e6b6c1b3a5f9b41491b5520585ed3f7ee36c4a4e447c072` |
| telperion | 1 | `f7c53daa0289ad73ac63a4a3be3f934e5f04361fca28a34d77f02ed396f6b718` |
| telperion | 7 | `0f36564dbff4cc5355d30c2451c1e11575a1d23efa502c77af1f4a71053f9716` |
| telperion | 42 | `b2baa7bc3678dbf2c952e01e0d5b170231de647cc54b5bd54c04bb3fbf90a81e` |
| laurelin | 1 | `9d4af4126cfbc1dd96daf79727bf75ed0126e83aecf44e7fbe4b76a182e64587` |
| laurelin | 7 | `3550656bd04074b671cdf6ef4176486cd8466f491846415ff630ef4dccdce47f` |
| laurelin | 42 | `976b04dd019bf6298d1fdef8a7fbc20dd328066da560910caac3ad039929d143` |

## After task 1

The three digests re-run after task 1's edits equal the baseline: core
`raw/core-digest-after.txt`, render pipeline `raw/generation-digest-after.txt`
(byte-identical), resident `raw/resident-digest-after.txt` (identical lines).

## After task 2

The three digests re-run after task 2's edits (the space crate's passes,
hooks and test-only items) equal the baseline: core
`raw/core-digest-t2.txt` (all 24 lines), render pipeline
`raw/generation-digest-t2.txt` (byte-identical, sha256 `fb4ce911...`),
resident `raw/resident-digest-t2.txt` (identical lines, under
`telperion-heavy 10 --gpu 6`).

## After task 3

The three digests re-run after task 3's edits (the scaffold grower under
`cfg(test)`, `Tree.pipe` deleted) equal the baseline: core
`raw/core-digest-t3.txt` (all 24 lines), resident
`raw/resident-digest-t3.txt` (identical lines, under
`telperion-heavy 10 --gpu 6`), render pipeline
`raw/generation-digest-t3.txt` (byte-identical, sha256 `fb4ce911...`).

## After task 4

The three digests re-run after task 4's edits (a mesh request's wood drawn
from its traced curve, the one station count, the one `Expansion`
preparation) equal the baseline: core `raw/core-digest-t4.txt` (all 24
lines), render pipeline `raw/generation-digest-t4.txt` (byte-identical to
`raw/generation-digest-base-1.txt`; its `mesh` field is the one digest that
covers `Request::mesh()` and so the curve-drawn wood), resident
`raw/resident-digest-t4.txt` (identical lines, under `telperion-heavy 10
--gpu 6`). The two `retains` rules agree on every shipped preset
(`raw/t4-retains-rules.txt`).

## After task 5

The three digests re-run after task 5's edits (the CPU delivery and the
renderer's test-only GPU code out of the production build) equal the
baseline: core `raw/core-digest-t5.txt` (all 24 lines), render pipeline
`raw/generation-digest-t5.txt` (byte-identical, sha256 `fb4ce911...`),
resident `raw/resident-digest-t5.txt` (identical lines, under
`telperion-heavy 10 --gpu 6`).

## After task 8

The three digests re-run after task 8's edits (`validate_skeleton` without
the scaffold-only preconditions, `Delivery` and `Backend` deleted) equal the
baseline: core `raw/core-digest-t8.txt` (all 24 lines), render pipeline
`raw/generation-digest-t8.txt` (byte-identical, sha256 `fb4ce911...`),
resident `raw/resident-digest-t8.txt` (identical lines, under
`telperion-heavy 10 --gpu 6`).

## Task 6 probe: the rings read the renderer's packed frames

Finding 3's stop condition, measured before any sweep change. The rings
(`resident/rings.wgsl`) are the one reader of the sweep's points that reads
their frames; the stations read no point. The probe left the sweep as it is
and passed each point's normal and binormal through the renderer's packing
and decoding (`handoff.wgsl`'s `octahedral`, `curve_walk.wgsl`'s
`unoctahedral`) before the rings wrote them (`raw/rings-probe.wgsl`). The
norway spruce is the one shipped preset that seats leaves on the rings
(`surfaceContact` 1.0), so it is the only preset whose leaves read ring
frames.

`DIGEST_PRESETS=norway-spruce DIGEST_SEEDS=<seed> DIGEST_DUMP=<file>` on
`resident::digest_tests::digest`, base and probe, matrices compared by
`raw/compare.py`; logs `raw/t6-probe-rings-{base,packed}*.log`. The base
lines equal the baseline. The points, clusters, sections and demand are
unchanged; the leaves differ:

| seed | leaves | leaves moved | largest move | rotation or scale changed |
| ---: | ---: | ---: | ---: | --- |
| 1 | 4,024,570 | 1 | 0.194 mm | none |
| 7 | 3,925,478 | 6 | 0.197 mm | none |
| 42 | 4,084,402 | 10 | 0.195 mm | none |

Each moved leaf moved by about 0.19 mm, which looks like one step of the
leaf's quantised position: a rounding boundary crossed, not a drift.

## After task 6: one drawn-record buffer, the sweep writes the renderer's form

Task 6's edits: the dress writes every wave into one buffer of records
(by id, then the axes by slot, with room for the retained leaf bases);
the sweep, stations and field read those records, the 39-word node table
is gone; the sweep writes the packed curve and folds the demand and box,
the handoff pass is gone; the rings read their frames from the packed
points (host decision (a) on finding 3).

- **Core:** `raw/core-digest-t6.txt`, all 24 lines equal `raw/core-digest-t8.txt`.
- **Render pipeline:** `raw/generation-digest-t6.txt` byte-identical (sha256 `fb4ce911...`).
- **Resident:** `raw/resident-digest-t6.txt` (`telperion-heavy 10 --gpu 6`, `DIGEST_BOUNDS=1`, log beside it).
  21 of 24 lines equal the baseline. The norway spruce's three lines are
  re-pinned: their points, clusters, sections and demand hashes are
  unchanged, and only the leaves hash moved. Every tree's box
  (`DIGEST_BOUNDS`, not hashed by the lines) equals the base build's
  box, 24 of 24. The base was taken at `2f437cbd4` with the same switch,
  in `raw/resident-digest-t6-base.log`.

| preset | seed | line sha256 (re-pinned) | measured difference (`raw/t6-leaves-moved.txt`) |
| --- | ---: | --- | --- |
| norway-spruce | 1 | `946eb18751e42a277f00493b48ca0cdddf5fafeb061cc9f778135fd756284984` | 1 of 4,024,570 leaves moved, at most 0.194 mm; no rotation or scale changed |
| norway-spruce | 7 | `3e5e30bcb2361c50ba8424d2564eccc28394c9545fd5289e72edc829f4ed137b` | 6 of 3,925,478 leaves moved, at most 0.197 mm; no rotation or scale changed |
| norway-spruce | 42 | `14c998d0083b02c6b352e03b3254f2b8df4c1a10dc75c9da1324aadfce15fbe5` | 10 of 4,084,402 leaves moved, at most 0.195 mm; no rotation or scale changed |

These are the same counts and distances the probe measured (above): the
leaves were compared value by value (`DIGEST_DUMP`, base binary against
after binary).

- **Stills:** the spruce at seeds 1, 7 and 42 at the hero pose, 960x720
  (`DIGEST_STILLS`), before and after. They are identical pixel for pixel
  (decoded image data compared), so there is no visible difference.
- **CPU and device (for fn-289):** the CPU reference's spruce leaves
  (`prepare_whole_resident`, `DIGEST_CPU`) did not match the device's leaf
  for leaf before this change either: the counts differ at seeds 1 and 7
  (4,024,568 against 4,024,570; 3,925,480 against 3,925,478), and every
  leaf differs by index (`raw/t6-cpu-vs-device.txt`). The 1, 6 and 10
  moved leaves are a device-side change. No second path was built.
