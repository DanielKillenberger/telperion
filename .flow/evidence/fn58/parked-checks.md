# fn-58 parked checks, run live on 2026-09-18

Three parked unknowns of R2, run against the installed Firecrawl CLI v1.23.3
and the recorded TypeSafe evidence. Source bytes stay outside the repository:
every fetched file lives under the session scratchpad
(`/tmp/claude-1000/-home-daniel-Projects-telperion/d31a3722-5b78-457e-adaf-3260359983df/scratchpad/parked/`)
and only checksums, sizes and verdicts are recorded here.

Credits: the scrape metadata reported `creditsUsed` 1, the one web search
reported `creditsUsed` 2, and the three research-index searches reported no
credit field. `firecrawl credit-usage` read 590 remaining before the checks and
588 remaining after them, so the checks cost 2 to 3 credits of the 12 allowed.

## 1. Research index coverage for forestry

Commands:

```
firecrawl research search-papers "Gould Harrington Devine 2011 growth of Oregon white oak Quercus garryana" --limit 5 --json
firecrawl research search-papers "Vospernik Monserud Sterba 2010 do individual-tree growth models correctly represent height diameter ratios of Norway spruce open-grown trees" --limit 5 --json
firecrawl research search-papers "Bavarian yield tables Ertragstafeln Assmann Franz 1963 Norway spruce mean height by site class" --limit 5 --json
firecrawl search "Gould Harrington Devine 2011 growth of Oregon white oak Quercus garryana" --limit 3 --json
```

| Target | In the paper index | Best record returned |
| --- | --- | --- |
| Gould, Harrington, Devine 2011, Growth of Oregon white oak | no | "Consumer-based limitations drive oak recruitment failure" (pmid:20715631), score 0.307 |
| Vospernik, Monserud, Sterba 2010, open-grown trees | yes | "Do individual-tree growth models correctly represent height:diameter ratios of Norway spruce and Scots pine?" pmcid:PMC2987550, doi 10.1016/j.foreco.2010.07.055, score 0.883 |
| Bavarian Ertragstafeln, Assmann and Franz 1963 | no | "Generic biomass functions for Norway spruce in Central Europe" (pmid:14676030), score 0.278 |

The web search found the missing paper at position 1,
`https://research.fs.usda.gov/treesearch/39910`, "Growth of Oregon white oak
(Quercus garryana)", with the BioOne full text at position 2.

The index answers with source ids, not URLs, so the adapter builds the URL from
the DOI, then the PMC id, then the PubMed id.

**Verdict.** The research index holds the forestry literature that PubMed and
PMC index, which is the journal subset: a Forest Ecology and Management paper
is there, a US Forest Service station publication and a German yield table are
not. Discovery for a species runs on web search for the grey literature,
extension pages and yield tables that carry most measured values, and calls the
research index for the journal subset only. Both stay in the contract; neither
alone covers discovery.

## 2. Adapter fidelity: raw HTML against the original response

Commands:

```
firecrawl scrape "https://research.fs.usda.gov/silvics/oregon-white-oak" -f rawHtml,markdown,links --json
curl -sL -D headers.txt -o curl-owic.html "https://research.fs.usda.gov/silvics/oregon-white-oak"
```

| Bytes | Size | sha256 |
| --- | --- | --- |
| curl response body | 103163 | 21d0db9ce11e8ee1966c5adc97bb3140f64ef2efc8744de0be72f587fdbeeb5f |
| fn30's recorded curl fetch | 103163 | 21d0db9ce11e8ee1966c5adc97bb3140f64ef2efc8744de0be72f587fdbeeb5f |
| adapter `rawHtml` | 105177 | ffa3d68d06859f687ea76ab936877594bd867831252d4a00fa5d8c412bee4b70 |
| adapter `markdown` | 55579 | 17f7417762ff968bccbcee8793d288f80a4eaf4358ffc933495e15504426045f |

With `--json` and two or more formats the CLI printed the whole payload on
stdout and created no `.firecrawl/` directory, so the adapter reads stdout and
uses its cache directory only for the PDF bytes it saves itself.

The metadata carried statusCode 200, sourceURL and url both
`https://research.fs.usda.gov/silvics/oregon-white-oak`, contentType
`text/html; charset=utf-8`, cacheState miss, proxyUsed basic.

The curl bytes reproduce fn30's checksum exactly, so the page is byte stable
and the difference is the adapter's. The two differ from byte 15 on:

```
curl    : <!DOCTYPE html>\n<html lang="en" dir="ltr">\n<head>\n  <meta charset="utf-8" />
adapter : <!DOCTYPE html><html lang="en" dir="ltr" class=" js"><head><meta http-equiv="Content-Type" content="text/html; charset=UTF-8">
```

**Verdict.** The adapter's raw HTML is not the original response bytes. It is a
serialization of the rendered DOM: the class the page's own script adds is
present, the charset meta is rewritten, and the source's newlines and
indentation are gone, for 2014 bytes more than the response carried. The fetch
stage therefore takes its raw checksum from a second plain request.
Implemented as `adapter::firecrawl::fetch_raw`, a plain GET through `ureq`
returning the final URL, the content type and the bytes, selected by the
`raw_from: RawSource` field on `FirecrawlCli`: `RawSource::Direct` (the default
of `FirecrawlCli::new`) checksums the response bytes, `RawSource::Cli` keeps
the CLI's serialization for a caller that wants the rendered DOM. A PDF always
takes the direct route, because the CLI's JSON cannot carry binary.

## 3. TypeSafe pinned model version

No API call was made. Checked in the worktree:

- `crates/telperion-jev/src/caller.rs` sends `MODEL = "jev-latest"` in the
  request body and records `model` from the response, falling back to the same
  constant when the response omits the field.
- `.flow/ledger/` does not exist in this worktree and is gitignored
  (`.gitignore` line 12), so no recorded ledger entry is available.
- No JSON under `.flow/evidence/fn58/` or `.flow/evidence/fn57/` carries a
  `model` field. The fn57 pilot records hold `label`, `expect` and `request`
  only, so the response side was never stored. The only occurrences of
  `jev-latest` anywhere are request-side: the caller constant and
  `.flow/evidence/fn57/pilot/jev.py`, which sets the same default.

**Verdict.** Unanswered from recorded evidence: nothing on disk shows what a
live response's `model` field said, so it is not known whether TypeSafe returns
a pinned version or echoes the alias. What R6 would record today is the alias
`jev-latest`. The parked unknown stays open and is answered by the first live
call whose ledger entry is kept: the caller already writes the response's
`model` into the entry, so no code change is needed, only one preserved entry.
Until then R6 interleaves the two drivers' trials in one window, which is the
spec's stated fallback when no version is exposed.
