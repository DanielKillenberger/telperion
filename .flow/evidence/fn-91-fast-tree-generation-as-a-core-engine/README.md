# fn-91 retained evidence

Start with [FINAL-VALIDATION.md](FINAL-VALIDATION.md), [COMPLETION-ASSESSMENT.md](COMPLETION-ASSESSMENT.md) and [FRICTION-REVIEW.md](FRICTION-REVIEW.md). Benchmark reproduction is documented in [generation.md](../../../scripts/benchmarks/generation.md).

The raw run archive removes 602 logs, line-oriented measurement files, screenshots and other generated artifacts (19,536,031 bytes). The 232 retained files contain reports, structured JSON summaries/provenance, replay sources and patches. Historical reports retain their original raw filenames; references to removed files mean the pinned archive, not a currently present file. Replay scripts that read old measurements need that archive or regenerated inputs.

[ARCHIVE.json](ARCHIVE.json) records every removed file's SHA-256 and byte size. All original evidence is preserved at Git revision `c4871093bd7ef208c2c3f65fb46a223c4e397a0b`, which remains reachable through master history. Recover the original checkout, including raw inputs and the code they measured, without changing your current checkout:

```sh
git worktree add --detach /tmp/telperion-fn91-replay c4871093bd7ef208c2c3f65fb46a223c4e397a0b
```

Run historical analysis from that checkout after reviewing each script's environment and scratch-path assumptions. Reproduction of hardware timings requires comparable hardware; the archive is evidence of the original runs, not a claim that timings will repeat exactly.

A second, byte-verified local copy of the entire evidence directory is at `~/.local/share/telperion/evidence-archives/fn-91-c4871093bd7e.tar.gz`. Its checksum is in the manifest. This copy is not uploaded or assumed available on other machines; use Git recovery there.

Future output follows [evidence-retention.md](../../../docs/evidence-retention.md). Existing raw-output destinations under this fn-91 directory are ignored so running a historical script does not accidentally recommit its outputs.
