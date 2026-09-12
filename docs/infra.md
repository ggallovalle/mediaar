# Infrastructure

Mediaar is a single Rust binary that embeds both a GPUI desktop UI and a ratatui TUI. There is no separate desktop package, service mesh, or cloud runtime in-tree yet.

## Why GPUI (not Tauri)

Desktop previously used **Tauri 2 + SolidJS** (system WebView). It now uses native **[GPUI](https://gpui.rs/)** (Zed’s GPU UI toolkit) for the same welcome surface.

| Concern | Why GPUI won |
| --- | --- |
| Runtime RAM | ~44 MB process-tree RSS vs ~460 MB with WebKit helpers |
| Startup | Faster cold window ready; much snappier `--help` |
| Shipping shape | One Rust binary, no Node/`ui/dist` embed step |
| Source surface | Desktop UI is a few hundred lines of Rust instead of a Vite/Solid app |

Tradeoff: the release **binary is larger on disk** (Blade/Vulkan/Wayland stack linked in) even though RAM and app source got smaller.

The full before/after numbers, screenshots, and harness live on the tagged experiment (not on `main`):

- Tag [`comparison/gpui-vs-tauri`](https://github.com/ggallovalle/mediaar/tree/comparison/gpui-vs-tauri) — metrics + screenshots under `benches/`
- Tag [`snapshot/gpui-desktop`](https://github.com/ggallovalle/mediaar/tree/snapshot/gpui-desktop) — same experiment snapshot
- Branch [`experiment/gpui-desktop`](https://github.com/ggallovalle/mediaar/tree/experiment/gpui-desktop) — convenient checkout of that tree

```sh
git fetch origin tag comparison/gpui-vs-tauri
git show comparison/gpui-vs-tauri:benches/COMPARISON.md
```

## Toolchain

Pinned in [`mise.toml`](../mise.toml):

| Tool | Role |
| --- | --- |
| Rust `1.98.0` | CLI, TUI, GPUI host |
| usage `6.8.0` | Spec / docs tooling (`usage` CLI) |

Biome remains available for formatting leftover config files; there is no desktop Node frontend on `main`.

Use mise for local env consistency:

```sh
mise install
mise run build
```

Package manager: **Cargo** (Rust).

## Layout

```text
mediaar/
├── Cargo.toml                 # workspace root
├── install.sh                 # curl|sh user installer (mise-style)
├── usage/mediaar.usage.kdl    # checked-in usage spec (keep in sync with CLI)
├── packaging/                 # man, zsh completion, desktop entry, icons
├── scripts/pack-release.sh    # builds GitHub/binstall .tgz + SHA256SUMS
├── crates/kbgpui/             # GPUI widgets (`ui`) + Catppuccin theme (Zed-shaped)
├── crates/mediaar/            # the only distributable package
│   ├── src/                   # usage-rs CLI + ratatui + GPUI entry
│   ├── locales/
│   └── icons/
└── mise.toml
```

## Runtime shape

```text
mediaar desktop  →  in-process GPUI window
mediaar tui      →  in-process ratatui app
```

One artifact: `target/release/mediaar`.

CLI surface is declared with **usage-rs** and should match [`usage/mediaar.usage.kdl`](../usage/mediaar.usage.kdl). Refresh the checked-in spec with:

```sh
cargo run -p mediaar -- __usage_spec__ > usage/mediaar.usage.kdl
```

## Build pipeline

1. **Binary** — `cargo build -p mediaar --release`
2. GPUI compiles into the same binary (no webview embed step)

### Linux desktop deps

GPUI needs a working Wayland or X11 display stack (Vulkan/Metal backends via Blade). Install typical GPU/display libs for your distro before `mediaar desktop`.

## Local commands

| Goal | Command |
| --- | --- |
| Release binary | `mise run build` |
| Desktop (dev profile) | `mise run dev-desktop` |
| TUI | `mise run dev-tui` |

## Distribution

### curl | sh (user install)

[`install.sh`](../install.sh) is a mise-style user installer (not system-wide). It places:

| Artifact | Path |
| --- | --- |
| Binary | `~/.local/bin/mediaar` |
| Man page | `~/.local/share/man/man1/mediaar.1` |
| Zsh completion | `~/.local/share/zsh/site-functions/_mediaar` |
| Desktop entry | `~/.local/share/applications/mediaar.desktop` |
| Icons | `~/.local/share/icons/hicolor/*/apps/mediaar.png` |

### GitHub Release / binstall

[`scripts/pack-release.sh`](../scripts/pack-release.sh) builds a versioned `.tgz` plus `SHA256SUMS.txt` under `dist/`. See [`release.md`](release.md).
