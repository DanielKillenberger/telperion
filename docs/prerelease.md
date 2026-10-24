# Prerelease

A prerelease puts a branch's package on npm under the dist-tag `next`, so a
consumer such as killenberger.com can install and test it before the
release. It is published only when the owner asks, by dispatching
`.github/workflows/prerelease.yml`; nothing publishes one on a push or on a
schedule. The released package is unchanged: `release.yml` still publishes
it from a green master Tests run, under `latest`.

## Publishing one

```sh
gh workflow run prerelease.yml -r <branch>                 # publish the branch's tip
gh workflow run prerelease.yml -r <branch> -f dry_run=true # pack and list it, publish nothing
gh workflow run prerelease.yml -r <branch> -f ref=<sha>    # publish another ref with the branch's workflow
```

Or, from the Actions tab, open Prerelease, choose Run workflow, pick the
branch and tick dry run if wanted. The release branch is the usual choice.

GitHub dispatches a workflow only when its file is on the default branch,
so `prerelease.yml` has to be on `master` before the first run; until then
`gh workflow run` cannot find it.

The job checks out the ref, builds the package through the package job's own
build (`.github/actions/package`: the pinned toolchain, `npm run build` and
the artifact-size check), then sets the version, packs the tarball, lists
its files and runs `npm publish --tag next --provenance` on it. A dry run
runs the same command with `--dry-run` and needs no token.

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

## The token

npm's trusted publisher for this package names `release.yml`, so the
prerelease authenticates with a token instead: the repository secret
`NPM_TOKEN`. Once, the owner:

1. creates a granular access token on npmjs.com with read and write on the
   `telperion` package, and sets "bypass two-factor authentication" when
   the package or the account requires 2FA to publish;
2. adds it to the repository with `gh secret set NPM_TOKEN` (it prompts for
   the value).

The token reaches only the publish step, through the `.npmrc` that
`actions/setup-node` writes, and only on a real publish; it is never
printed. A real publish without it fails before `npm publish` with a
message naming `NPM_TOKEN` and the command that sets it. A granular token
expires; when it has, the publish fails at npm and the owner replaces the
secret the same way.
