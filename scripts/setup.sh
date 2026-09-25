#!/usr/bin/env sh
# One-time setup of a clone: the checked-in git hooks (.githooks), so every
# worktree of the clone checks the design principles before a push.
set -e
git config core.hooksPath .githooks
echo "core.hooksPath = .githooks: the pre-push principles check is on (git push --no-verify skips it)."
