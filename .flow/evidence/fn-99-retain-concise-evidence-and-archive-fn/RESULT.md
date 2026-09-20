# Cleanup result

Removed 602 generated fn-91 artifacts totaling 19,536,031 bytes. Kept 232 existing reports, structured JSON summaries/provenance, replay sources and patches, plus a recovery README and checksum manifest. Original evidence is recoverable from merged revision c4871093bd7ef208c2c3f65fb46a223c4e397a0b and a byte-verified local archive; nothing is uploaded and history is not rewritten.

Validation: `python .flow/evidence/fn-99-retain-concise-evidence-and-archive-fn/verify.py` passed archive checksums/byte counts for all 602 files, ignore rules, retained Markdown links, and production/test/demo exclusions. This host-side check requires the local archive named in fn-91's README. `git diff --check` and `flowctl validate --spec fn-99` passed. No engine tests were rerun because code and fixtures are untouched.

The archive-dependent historical script inputs are explicitly documented, rather than pretending those scripts still have raw data present in this checkout. The new retention rule requires future uncommitted evidence to get durable storage before its only copy is deleted.

Friction: one local SSH-agent failure, resolved with HTTPS without changing the configured remote. No repository fix or new spec is proposed for that machine setup issue.
