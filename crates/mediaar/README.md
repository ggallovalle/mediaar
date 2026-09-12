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
mediaar desktop   # Catppuccin Tauri + Solid desktop app
mediaar tui       # Catppuccin ratatui welcome screen
mediaar --help
```

Both UIs ship inside the `mediaar` binary. The desktop frontend is embedded at build time from `ui/dist`.

## Develop

```sh
pnpm --dir crates/mediaar/ui install
pnpm --dir crates/mediaar/ui build
cargo run -p mediaar -- tui
cargo run -p mediaar -- desktop
```

For hot-reload UI work, run Vite and the binary together:

```sh
pnpm --dir crates/mediaar/ui dev
# in another shell, with the Vite server up:
cargo run -p mediaar -- desktop
```

## Distribute

```sh
mise run pack-release
# upload dist/mediaar-*-*.tgz and dist/SHA256SUMS.txt to a GitHub Release
```

CI should:

1. `pnpm --dir crates/mediaar/ui build`
2. `cargo build -p mediaar --release` (or `mise run pack-release`)
3. Publish the release archive (and optionally crates.io)

`cargo binstall` resolves prebuilt archives via `[package.metadata.binstall]`.
