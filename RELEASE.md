# Releasing Mediaar

How to cut a GitHub Release that powers `install.sh`, `cargo binstall`, and the desktop launcher assets.

## Prerequisites

```sh
mise install
gh auth status   # logged into GitHub with repo access
```

Linux desktop builds need the usual Tauri / WebKitGTK system packages.

## Version bump

Bump these together so `--version`, the archive name, and Tauri agree:

| Location | Field |
| --- | --- |
| [`Cargo.toml`](Cargo.toml) | `[workspace.package] version` |
| [`crates/mediaar/src/cli/mod.rs`](crates/mediaar/src/cli/mod.rs) | `#[usage(..., version = "...")]` |
| [`crates/mediaar/tauri.conf.json`](crates/mediaar/tauri.conf.json) | `version` |

Example for `0.2.0`:

```sh
# edit the three places above, then refresh the checked-in usage spec
cargo run -p mediaar -- __usage_spec__ > usage/mediaar.usage.kdl
```

Commit the bump on `main` (or your release branch) before tagging.

## Build the release archive

On the machine whose Rust target you want to ship (today: host only):

```sh
mise run pack-release
# or: scripts/pack-release.sh
# or: scripts/pack-release.sh --skip-build   # if target/release/mediaar is already built
```

Outputs under `dist/`:

| File | Purpose |
| --- | --- |
| `mediaar-{version}-{target}.tgz` | Binary + man + zsh + desktop + icons |
| `SHA256SUMS.txt` | Checksums for `install.sh` verification |

Archive name must match cargo-binstall / install script expectations:

```text
mediaar-{version}-{rust-target-triple}.tgz
# e.g. mediaar-0.2.0-x86_64-unknown-linux-gnu.tgz
```

Flat layout inside the `.tgz`:

```text
mediaar
mediaar.1
_mediaar
mediaar.desktop
icons/32x32.png
icons/128x128.png
icons/256x256.png
```

`install.sh` rewrites `@MEDIAAR_BIN@` / `@MEDIAAR_ICON@` (and legacy `Icon=mediaar`) to absolute paths under `~/.local`.

## Publish a GitHub Release

Tag with a leading `v` (install script strips it):

```sh
VERSION=$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)
git tag -a "v${VERSION}" -m "v${VERSION}"
git push origin "v${VERSION}"

gh release create "v${VERSION}" \
  "dist/mediaar-${VERSION}-$(rustc -vV | sed -n 's/^host: //p').tgz" \
  dist/SHA256SUMS.txt \
  --title "v${VERSION}" \
  --notes "Release notes for v${VERSION}."
```

To replace assets on an existing tag:

```sh
gh release upload "v${VERSION}" \
  "dist/mediaar-${VERSION}-$(rustc -vV | sed -n 's/^host: //p').tgz" \
  dist/SHA256SUMS.txt \
  --clobber
```

`install.sh` and `uninstall.sh` are **not** release assets — users curl them from `main`:

```text
https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh
https://raw.githubusercontent.com/ggallovalle/mediaar/main/uninstall.sh
```

Keep those scripts on `main` in sync with the archive layout you ship.

## Verify

```sh
# resolves latest GitHub release and installs into a throwaway HOME
env -u XDG_DATA_HOME HOME=$(mktemp -d) \
  sh -c 'curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh'

# dry-run only
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh -s -- --dry-run

# pin the version you just published
MEDIAAR_VERSION=0.2.0 sh install.sh --dry-run
```

Check that the installed `.desktop` has absolute `Exec=` and `Icon=` paths, then confirm **Mediaar** appears in your app launcher with the branded icon.

## Optional: crates.io

When the crate is ready to publish (includes embedded `ui/dist` via package `include`):

```sh
mise run ui
cargo publish -p mediaar --dry-run
cargo publish -p mediaar
```

`cargo binstall mediaar` still prefers the GitHub `.tgz` via `[package.metadata.binstall]` when the release exists.

## Multi-target notes

`scripts/pack-release.sh` builds for the **current host** triple only. For more platforms:

1. Build (or cross-compile) `mediaar` for each target
2. Run pack (or assemble the same flat layout) per target
3. Upload every `mediaar-{version}-{target}.tgz` into the **same** GitHub Release
4. Append all checksums into one `SHA256SUMS.txt`

Windows MSVC archives should be `.zip` to match `[package.metadata.binstall.overrides]`.

## Checklist

- [ ] Version bumped in workspace `Cargo.toml`, CLI `#[usage]`, and `tauri.conf.json`
- [ ] `usage/mediaar.usage.kdl` refreshed if the CLI surface changed
- [ ] `mise run pack-release` succeeded
- [ ] Git tag `v{version}` pushed
- [ ] GitHub Release has `.tgz` + `SHA256SUMS.txt`
- [ ] `install.sh --dry-run` / throwaway install works
- [ ] (Optional) `cargo publish -p mediaar`
