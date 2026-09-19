# Friction reports, fn-75 (pipeline fixes from the first ash run)

Entries follow the friction rule in CLAUDE.md; the owner decides which become specs.

## 2026-09-18 21:05 the task branch was cut without the pipeline it fixes

- **Doing:** re-anchoring on fn-75.1 in the worktree on `fn-75-pipeline-fixes-from-the-first-ash-run` at c2190db2.
- **Hindered by:** the branch was cut from the local `master` at c2190db2, which is a sibling of PR #34's merge (d0f8516b) rather than its descendant: the commit that closed fn-58 was made on a local master that never pulled the landed PR. `crates/telperion-jev/src/pipeline/` did not exist in the worktree, so none of the six criteria had code to change.
- **Cost:** about ten minutes of orientation and one merge commit (09b49e62, `origin/master` into the branch, no conflicts) that now sits in the branch's range; the conductor's `.flow/tmp/spec_base` still names c2190db2.
- **What would remove it:** the close step of a landed spec pulls `origin/master` before it commits and cuts the next branch, or the worktree recipe cuts from `origin/master` rather than the local branch.
- **Early return:** not taken; the merge was clean and the task proceeds on the merged tree.

## 2026-09-18 21:08 the implementer tier is refused

- **Doing:** probing codex reach with one short `codex exec -m gpt-6-astra` call before composing the brief.
- **Hindered by:** `You've hit your usage limit ... try again at Sep 19th, 2026 4:27 PM`.
- **Cost:** one probe call, under five seconds; the session model implements instead.
- **What would remove it:** a reach check in the conductor before dispatch, so the worker is briefed with the fallback already decided.
- **Early return:** not taken; the fallback the prompt names is the session model.

## 2026-09-18 21:50 R5's positive example is a server that omits its intermediate

- **Doing:** checking R5 live once, by hand, after switching the raw request to the host's certificate store (ureq `native-certs`).
- **Hindered by:** `https://plantfinder.mobot.org/PlantFinderDetails.aspx?taxonid=282928` still fails with `invalid peer certificate: UnknownIssuer`, and so does `curl` with the same system store (`unable to get local issuer certificate`); `openssl s_client` shows the server sends only its leaf (`*.mobot.org`, issuer `Network Solutions RSA OV SSL CA 3`) with `Verify return code: 21 (unable to verify the first certificate)`. Firecrawl's browser passes because browsers fetch the missing intermediate over the certificate's AIA URL; rustls and curl do not.
- **Cost:** about five minutes and one live test that asserted the wrong thing, now removed. The spec's positive example was marked [inferred] and the inference does not hold for this page: it is R5's error case, not its success case.
- **What would remove it:** either the fallback the fn-56 friction entry also named (an unavailable source Firecrawl did scrape takes its raw bytes from the CLI's `rawHtml`, with the fidelity difference recorded), or AIA chasing in the raw request; both are the owner's call, neither is in fn-75's criteria.
- **Early return:** not taken; the system-store change stays (it is right for chains the bundled roots lack and the host trusts) and the error case is pinned in `tests/pipeline_fixes.rs`.
