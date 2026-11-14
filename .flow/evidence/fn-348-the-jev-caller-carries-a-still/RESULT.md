# fn-348 result: the Jev caller carries a still

## The contract, read 11 Nov 2026

Read live from https://docs.typesafe.ai/api/system-one.md and https://docs.typesafe.ai/images.md before the code was written.

- Shape (system-one.md, `SystemOneRequest.assets`): "Media assets keyed by an opaque placeholder (recommend a per-image UUID). Each placeholder, wherever it appears as or inside a string in state, question instructions, or criteria, is replaced with the asset." An asset is `{"type": "image", "format": "png" | "jpg" | "jpeg", "data_base64": ...}`.
- Several images in a structured state (images.md, "Sending images in a request"): "Each image is listed once, in the request's top-level `assets` map. Its key in that map is a reference string. Write the reference wherever the image belongs, as a whole string value or inside a longer string, and the model receives the image in its place." A question names one by path: "refer to one frame by path, such as `` `frames[2].frame` ``".
- Header (images.md): "send the `TypeSafe-Danger-Zone-Images: true` header with each request that contains an image or a Box question." Without it the API returns 400.
- References (images.md): "Every asset must be referenced. A request with an asset that is not referenced anywhere returns `400`." and "A short reference such as `cat` could also match inside `category`. Use references that can't occur in your text by accident."
- Limits (images.md, "Limits"): "PNG or JPEG, still images only"; "16 MiB per image file. For base64, this is the size after decoding."; "280 to 1568 pixels each. An image with either side under 280 pixels is rejected."; total pixels "78,400 to 2,458,624"; request body and all images together 32 MiB each. "An image outside them is rejected, never resized to fit." Count: no fixed count; "How many images fit in one request depends on these limits and on the context length."
- Models (images.md): "Jev 1.13 and Jev 1.14 both accept images." `jev-latest` answered as `jev-1.14.0`.

The caller checks the per-image limits it can read from the file before any call (format from the file's own header, 16 MiB, 280 to 1568 px on each side, which also bounds the total pixels) and refuses the first asset outside them by name. The 32 MiB request and all-images totals are left to the API's 413.

## The live call (R4)

Still: `renew-oak-1001-whole.jpg` from fn-346 (2440x1980), fitted by `prepare-still.sh` to 1568x1272, 488,351 bytes, SHA-256 `f0c53b0782ea1005dd911eff2331e9d2a3961041a5579fffbdd42431faf4b2d5` (the fitted file sits in the ignored `raw/`). State, question and asset map are `ask-state.json`, `ask-questions.json` and `ask-assets.json` here.

```sh
bash -ic 'target/ci/jev ask --tool ask-still \
  --state .flow/evidence/fn-348-the-jev-caller-carries-a-still/ask-state.json \
  --questions .flow/evidence/fn-348-the-jev-caller-carries-a-still/ask-questions.json \
  --assets .flow/evidence/fn-348-the-jev-caller-carries-a-still/ask-assets.json'
```

| Question | Choice | Confidence | in_leaf | bare | neither (no match) |
| - | - | - | - | - | - |
| `foliage`: do the trees in `still` carry leaves? | `in_leaf` | 0.85 | 0.90 | 0.09 | 0.01 |

Model `jev-1.14.0`, 5,558 input and 43 output tokens, 292 ms. Ledger reference `ask-still:18e6d6556451f68d-3cf88c-0` (`.flow/ledger/jev/2026-11-11T003243Z-ask-still-18e6d6556451f68d-3cf88c-0.json`, 1,263 bytes). The entry's `assets` holds the reference name, the path and the SHA-256 above, and no image bytes. The sheet does show 25 trees in leaf, so the answer matches the picture; it is one call, evidence of the path working and not a measure of accuracy.
