# Mediaar

Manage your media from one installable binary:

```sh
# user install (binary + zsh completion + man + desktop launcher)
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh
# preview only:
curl -fsSL .../install.sh | sh -s -- --dry-run

# or via cargo
cargo binstall mediaar
cargo install mediaar --locked
```

## Commands

```sh
mediaar desktop   # Catppuccin GPUI desktop app
mediaar tui       # Catppuccin ratatui welcome screen
mediaar --lang es desktop   # force Spanish for this run
mediaar --help
```

Locale files live in `locales/{en,es}/` as Fluent resources (`common.ftl` shared, plus `desktop.ftl` / `tui.ftl`). Language resolution uses usage-config layers: `--lang` → `~/.config/mediaar/config.toml` (UI toggle) → system locale default (`LANG` / `LC_*`, else `en`).

Both UIs ship inside the `mediaar` binary. Desktop is native GPUI (no embedded webview). See [`INFRA.md`](../../INFRA.md) for why GPUI replaced Tauri.

## Develop

```sh
cargo run -p mediaar -- tui
cargo run -p mediaar -- desktop
```

## Distribute

```sh
mise run pack-release
# upload dist/mediaar-*-*.tgz and dist/SHA256SUMS.txt to a GitHub Release
```

CI should:

1. `cargo build -p mediaar --release` (or `mise run pack-release`)
2. Publish the release archive (and optionally crates.io)

`cargo binstall` resolves prebuilt archives via `[package.metadata.binstall]`.
