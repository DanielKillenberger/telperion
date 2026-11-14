# Prerelease

A prerelease puts a branch's package on npm under the dist-tag `next`, so a
consumer such as killenberger.com can install and test it before the
release. It is published only when the owner asks, by dispatching
`.github/workflows/release.yml`, which then runs only its `prerelease`
job; nothing publishes one on a push or on a schedule. The released
package is unchanged: `release.yml` still publishes it from a green master
Tests run, under `latest`.

## Publishing one

```sh
gh workflow run release.yml -r <branch>                 # publish the branch's tip
gh workflow run release.yml -r <branch> -f dry_run=true # pack and list it, publish nothing
```

Or, from the Actions tab, open Release, choose Run workflow, pick the
branch and tick dry run if wanted. The release branch is the usual choice.

GitHub dispatches a workflow only when its file is on the default branch,
so `release.yml` with its dispatch trigger has to be on `master` before
the first run; until then `gh workflow run` cannot dispatch it.

The job checks out the commit it was dispatched on (the branch's tip), so
the provenance names the commit that was built. It builds the package
through the package job's own build (`.github/actions/package`: the pinned
toolchain, `npm run build` and the artifact-size check), then sets the
version, packs the tarball, lists its files and runs
`npm publish --tag next --provenance` on it. A dry run runs the same
command with `--dry-run`.

## The version

`<package.json version>-next.<YYYYMMDD>.<short sha>`, for example
`0.2.0-next.20261024.dc42612`: the branch's version, the UTC date of the run
and the commit's seven-character hash. It is set in the job only and never
committed. The job refuses before publishing when the version has no
`-next.` suffix, when the tag is not `next`, or when the version is already
on npm, which is what a second run at the same commit on the same day finds.
A hash of digits only with a leading zero is not valid semver, and that run
fails at the version step; a later commit publishes.

`next` names the newest prerelease, whatever its version. When the branch's
version is already released, the prerelease sorts below it
(`0.2.0-next.…` precedes `0.2.0`), so `next` can trail `latest`.

## Installing it

```sh
npm install telperion@next                          # the newest prerelease
npm install telperion@0.2.0-next.20261024.dc42612   # one exact prerelease
```

The run's log lists the version it published, and
`npm view telperion dist-tags` shows what `next` points at.

## Going back

```sh
npm install telperion@latest   # the released version
npm install telperion@0.1.4    # or an exact one
```

`npm dist-tag rm telperion next` takes the `next` tag off npm; it is
optional, since nothing installs `next` unless it asks for it.

## Authentication

The prerelease publishes through npm trusted publishing, as the release
does: the job's OIDC token (`id-token: write`, npm 11.5.1 or later) under
the trusted publisher configured on npmjs.com for this repository and
`release.yml`. That is why the prerelease is a job of `release.yml` and not
a workflow of its own: the trusted publisher names one workflow file. No
npm token is stored, and no secret needs adding.
