# Releasing krabka-sspi

`krabka-sspi` is this fork's root crate published under a different name (the
library is still called `sspi`). It is a temporary measure: see "About
krabka-sspi" in the [README](../README.md). Only `krabka-sspi` is published;
every other workspace crate has `publish = false`, and the upstream release
workflows (`release-crates.yml`, `nuget.yml`) run only in
`Devolutions/sspi-rs`.

`.github/workflows/publish-krabka-sspi.yml` publishes. A `krabka-sspi-v*` tag
push uploads; a manual run is a dry run unless `dry_run` is turned off.

The canonical repository is
[krabka-io/sspi-rs](https://github.com/krabka-io/sspi-rs); the steps below
run there.

## Credentials

The publish job reads `secrets.CARGO_REGISTRY_TOKEN`, which krabka-io/sspi-rs
inherits from the krabka-io organization secret of that name. Nothing has to
be added to the repository. Check that the organization secret's repository
access includes `sspi-rs` (Organization → Settings → Secrets and variables →
Actions → `CARGO_REGISTRY_TOKEN`). Don't add a repository or environment
secret with the same name: it would override the organization one.

When the secret is not visible to the job, the workflow falls back to
crates.io trusted publishing, which works only once the crate exists and is
configured as below.

## First publish

1. In krabka-io/sspi-rs, open **Settings → Environments → New environment**
   and create `crates-io`.
   - Under **Deployment branches and tags**, choose **Selected branches and
     tags** and add the tag rule `krabka-sspi-v*`. Add no branch rule.
   - Optionally add a required reviewer.
2. Optionally check the release first: **Actions → publish-krabka-sspi → Run
   workflow** on branch `krabka-sspi` with `dry_run` on.
3. Tag the release commit and push the tag:

   ```sh
   git tag -a krabka-sspi-v0.23.0 -m "krabka-sspi 0.23.0"
   git push origin krabka-sspi-v0.23.0
   ```

   The `plan` job checks that the tag names the version in `Cargo.toml`,
   skips the upload if crates.io already has that version, and runs
   `cargo publish --dry-run`. The `publish` job then uploads with the
   organization token.

## Optional: trusted publishing

Once `krabka-sspi` exists on crates.io, you can stop relying on the
long-lived token for this crate. On crates.io, open `krabka-sspi` →
**Settings → Trusted Publishing → Add**, and enter owner `krabka-io`,
repository `sspi-rs`, workflow `publish-krabka-sspi.yml`, environment
`crates-io`. Then remove `sspi-rs` from the organization secret's repository
access, so the job no longer sees the token and authenticates through OIDC
with `rust-lang/crates-io-auth-action`.

## Later releases

Bump `version` in the root `Cargo.toml`, add a CHANGELOG entry, merge, and
push a tag `krabka-sspi-v<version>` on that commit. A rerun of the workflow is
safe: a version crates.io already has is skipped.

## Deprecating and yanking

Do this once an upstream `sspi` release contains the fork's changes
(devolutions/sspi-rs#738 and #764).

1. Move the krabka crates back to upstream: replace
   `sspi = { package = "krabka-sspi", version = "..." }` with
   `sspi = "<upstream version>"`, and release them. Do this first, so no
   published krabka crate still needs `krabka-sspi`.
2. Publish a final `krabka-sspi` patch release whose README and
   `description` say it is deprecated and point to
   [`sspi`](https://crates.io/crates/sspi) and the upstream version that
   contains the fixes. Bump the version (for example `0.23.1`), commit, and push
   tag `krabka-sspi-v0.23.1`. crates.io shows the README of the newest
   version, so this is how the deprecation notice reaches the crate page.
3. Optionally mark the repository archived or update its description.
4. Yank every version, including the deprecation release:

   ```sh
   cargo yank krabka-sspi --version 0.23.0
   cargo yank krabka-sspi --version 0.23.1
   ```

   Yanking keeps existing `Cargo.lock` files working but stops new
   resolutions. Keep the crate itself: crates.io deletes a crate only
   within 72 hours of its first publish, and the deprecation page is useful
   to anyone who finds the name later.
