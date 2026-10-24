---
satisfies: [R5]
---
# fn-270-growth-continues-from-any-age-in-less.4 Leaves and shoots unfold within the year

## Description
R5 and R5a: sourced leaf expansion and shoot elongation curves; girth and sag blended between years; this year's shed subtrees fade over a stated window; fractional presentation only, never feeding back into growth.
## Acceptance
Fractional settles viewed with no pop; whole years unchanged.

## Done summary
Leaves and shoots unfold within the year on CPU and device (R5, R5a), the seasonal year runs full leaf to full leaf (R5c), the palm's crown turns over by a sourced frond life (R5b). Old wood moves only while shoots grow; falling leaves stay whole and colour to their own abscission day; seasons past budburst are compressed. A build whose least tables cannot fit is refused before any submit. Evidence: `.flow/evidence/fn-270-growth-continues-from-any-age-in-less/UNFOLD.md`. Gate 1,365 passed (one inventory fix after); npm 162/162; codex SHIP.

## Evidence
- Commits: 133357fe0d33c61e89145638eae29b5f471df261, 8cd7051ca7dfc575c305fd7f464955a935b9e4c8, 0c39e4969c4c2a39de429378f0d46f11956fd313, 0711e1a18614475b4257cd4de55bd03a0a2374a1, 29c5f630da6f78b1fbfcdcb76733f99d1722a129, 75a99be07c635d080bcd144c80521dfd257a0723, 1b81b317aa968f5fbde0f56c5582ea2c90bb5f5d, f05b76dd6f0426174028494c97e0331e22b0ec71, 3006e4249f451249eedaba22bba7ecbc0ceba80e, 3542176d05aa33a7b301b27338e2f6d1c29a07ae, 2741e94424b27f3138e0de708a28558a5ca0eed2, 32c49f4dfd8bb9dc1393a0a5def5020200bae8ca, f3b65a3763e0a83b57884c417356846a795339f4, 93439db77673bb2b6e1e453f6a358118cc875fbf, 9fdbf5ed3027da8a78df32ae657f8f7e86bdb117, eedf2414ff3afa3636eb908bb31f60683755b3bd, 4378b1afbf35cdc7d448503eed748595f708ae2a, 4963ee3dc1f9d42e4487768cf3e355ed471482ff, 295a31799b872821a313f381bc77f756abf2bcc0, cc02d34e01574b16a31b30bf23712e58259b15e7, dca60fb376ca0c680af1f62279ad0f5e24e5a868, 409f690ca175dd18fa93e3266d5a39db21707889, 233dce2f18846d6d9ac03642c86256e8f7ca6e9a, b6242c8a792e66ca6a87831548c6b06e191aecb7, d10af139525b11db4221549d0302ce92cdcef21d, 1c84c96a66baa4286151de1a141f284f72cc45ef, 87e28eb76b48bb844a921adc54724730ac34c6ea, dc793916df275803f5c24146530818e1489617bb, 371bfaeb55d27ddcbb0ab0020564dd40e98e865d, 696f6bd33e8fd938d939acadd56188f58f320621, 5d28502fd7fa59452529dc2df142ef3a2ef57e40, abd499d24544786b8074c438e00aabad7183f293
- Tests: telperion-heavy 12 --gpu 4 --threads 1 -- cargo test --profile ci --workspace --no-fail-fast, telperion-heavy 10 --threads 4 -- npm test
- PRs: