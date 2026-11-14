# fn-338 friction

## 9 Nov 2026, worker, R1

- Doing: reading Alméras and Fournier 2009 in full from the HAL accepted manuscript through firecrawl's PDF parser.
- Slowed by: the scrape (93 k characters) was too large to return, so it went to a JSON file and had to be extracted with `jq`; the extraction drops most equation numbers and Table 1's units, so eq. numbers were matched from the text's references and Table 1's Δα unit was settled by reproducing the table (`probes/r1_premise.py` §1). Figure values come out as parser-reconstructed tables that cannot be trusted.
- Cost: about 5 minutes and roughly 35 k tokens of reading.
- Would remove it: a retained plain-text copy of each source a spec family cites (here fn-337 and fn-338 both read this paper), so the next reader starts from it rather than re-scraping.
