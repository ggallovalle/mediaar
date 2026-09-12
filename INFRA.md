# Infrastructure

Mediaar is a single Rust binary that embeds both a Tauri desktop UI and a ratatui TUI. There is no separate desktop package, service mesh, or cloud runtime in-tree yet.

## Toolchain

Pinned in [`mise.toml`](mise.toml):

| Tool | Role |
| --- | --- |
| Rust `1.98.0` | CLI, TUI, Tauri host |
| Node `26.3.1` | SolidJS frontend build |
| usage `4.1.0` | Spec / docs tooling (`usage` CLI) |

Use mise for local env consistency:

```sh
mise install
mise run ui
mise run build
```

Package managers: **Cargo** (Rust), **pnpm** (UI).

## Layout

```
mediaar/
├── Cargo.toml                 # workspace root
├── install.sh                 # curl|sh user installer (mise-style)
├── usage/mediaar.usage.kdl    # checked-in usage spec (keep in sync with CLI)
├── packaging/                 # man, zsh completion, desktop entry, icons
├── scripts/pack-release.sh    # builds GitHub/binstall .tgz + SHA256SUMS
├── crates/mediaar/            # the only distributable package
│   ├── src/                   # usage-rs CLI + ratatui + Tauri entry
│   ├── ui/                    # SolidJS 2 RC + Tailwind + Catppuccin
│   ├── tauri.conf.json
│   ├── icons/
│   ├── capabilities/
│   └── build.rs               # builds ui/dist if missing, then tauri_build
└── mise.toml
```

## Runtime shape

```
mediaar desktop  →  in-process Tauri window (embedded ui/dist)
mediaar tui      →  in-process ratatui app
```

One artifact: `target/release/mediaar`.

CLI surface is declared with **usage-rs** and should match [`usage/mediaar.usage.kdl`](usage/mediaar.usage.kdl). Refresh the checked-in spec with:

```sh
cargo run -p mediaar -- __usage_spec__ > usage/mediaar.usage.kdl
```

## Build pipeline

1. **UI** — `pnpm --dir crates/mediaar/ui build` → `crates/mediaar/ui/dist`
2. **Binary** — `cargo build -p mediaar --release`
3. **Embed** — `tauri-build` packs `ui/dist` into the binary when Tauri `custom-protocol` is enabled

`build.rs` will run `pnpm install` + `pnpm build` if `ui/dist/index.html` is missing, unless `MEDIAAR_SKIP_UI_BUILD=1`.

Required Tauri features (workspace `Cargo.toml`):

- `custom-protocol` — production asset loading (without this, desktop hits `http://localhost:1420`)
- `image-png` — window / app icons

### Linux desktop deps

Tauri needs system WebKitGTK / GTK (e.g. `webkit2gtk-4.1`). Install the usual Tauri Linux prerequisites for your distro before `mediaar desktop`.

## Local commands

| Goal | Command |
| --- | --- |
| Build UI | `mise run ui` |
| Release binary | `mise run build` |
| Desktop (dev profile) | `mise run dev-desktop` |
| TUI | `mise run dev-tui` |
| Hot-reload UI | `pnpm --dir crates/mediaar/ui dev` (Vite `:1420`) |

For hot-reload, the Vite server must be up if you temporarily run without embedded assets. Normal release / `custom-protocol` builds do **not** need Vite.

## Distribution

### curl | sh (user install)

[`install.sh`](install.sh) is a mise-style user installer (not system-wide). It places:

| Artifact | Path |
| --- | --- |
| Binary | `~/.local/bin/mediaar` |
| Man page | `~/.local/share/man/man1/mediaar.1` |
| Zsh completion | `~/.local/share/zsh/site-functions/_mediaar` |
| Desktop entry | `~/.local/share/applications/mediaar.desktop` |
| Icons | `~/.local/share/icons/hicolor/*/apps/mediaar.png` |

```sh
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/install.sh | sh -s -- --dry-run
curl -fsSL https://raw.githubusercontent.com/ggallovalle/mediaar/main/uninstall.sh | sh
MEDIAAR_VERSION=0.1.0 MEDIAAR_GITHUB_REPO=ggallovalle/mediaar sh install.sh
```

The desktop entry’s `Exec=` is rewritten to the absolute installed binary so app launchers can start `mediaar desktop` without relying on GUI `PATH`.

### cargo

```sh
cargo binstall mediaar
cargo install mediaar --locked
```

`[package.metadata.binstall]` expects GitHub release archives named like:

`mediaar-{version}-{target}.tgz` (zip on Windows MSVC).

Build a release archive (binary + man + zsh + desktop + icons):

```sh
mise run pack-release
# → dist/mediaar-{version}-{target}.tgz + dist/SHA256SUMS.txt
```

CI should:

1. `pnpm --dir crates/mediaar/ui build`
2. `cargo build -p mediaar --release` (or `mise run pack-release`)
3. Upload the `.tgz` (+ `SHA256SUMS.txt`) to GitHub Releases (and optionally `cargo publish -p mediaar`)

Published crate includes `ui/dist` via package `include` so source installs can skip a fresh UI build when dist is present.

Packaging sources live under [`packaging/`](packaging/) (desktop entry, completions, man, icons).

## Config / secrets

- [`.env.schema`](.env.schema) — Varlock schema placeholder; no runtime env contract yet
- Do not commit `.env` / `.env.local`

## Themes

Both UIs use **Catppuccin**:

- Light: Latte
- Dark: Mocha
