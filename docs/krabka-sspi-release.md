# Releasing krabka-sspi

`krabka-sspi` is this fork's root crate published under a different name (the
library is still called `sspi`). It is a temporary measure: see "About
krabka-sspi" in the [README](../README.md). Only `krabka-sspi` is published;
every other workspace crate has `publish = false`, and the upstream release
workflows (`release-crates.yml`, `nuget.yml`) run only in
`Devolutions/sspi-rs`.

`.github/workflows/publish-krabka-sspi.yml` publishes. A `krabka-sspi-v*` tag
push uploads; a manual run is a dry run unless `dry_run` is turned off.

## First publish

1. In robot-head/sspi-rs, open **Settings → Environments → New environment**
   and create `crates-io`.
   - Under **Deployment branches and tags**, choose **Selected branches and
     tags** and add the tag rule `krabka-sspi-v*`. Add no branch rule.
   - Optionally add yourself as a required reviewer.
2. Create a crates.io API token (crates.io → Account Settings → API Tokens)
   with the `publish-new` and `publish-update` scopes, limited to the crate
   `krabka-sspi` if crates.io allows it for a name that does not exist yet.
   Add it to the `crates-io` environment as the secret
   `CARGO_REGISTRY_TOKEN`. crates.io accepts trusted publishing only for a
   crate that already exists, so the first upload needs this token.
3. Optionally check the release first: **Actions → publish-krabka-sspi → Run
   workflow** on branch `krabka-sspi` with `dry_run` on.
4. Tag the release commit and push the tag:

   ```sh
   git tag -a krabka-sspi-v0.23.0 -m "krabka-sspi 0.23.0"
   git push origin krabka-sspi-v0.23.0
   ```

   The `plan` job checks that the tag names the version in `Cargo.toml`,
   skips the upload if crates.io already has that version, and runs
   `cargo publish --dry-run`. The `publish` job then uploads with the token.
5. Switch to trusted publishing: on crates.io, open `krabka-sspi` →
   **Settings → Trusted Publishing → Add**, and enter owner `robot-head`,
   repository `sspi-rs`, workflow `publish-krabka-sspi.yml`, environment
   `crates-io`.
6. Delete the `CARGO_REGISTRY_TOKEN` secret from the `crates-io` environment
   and revoke the token on crates.io. Later publishes authenticate through
   OIDC with `rust-lang/crates-io-auth-action`.

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
