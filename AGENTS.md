# AGENTS.md

Guidance for coding agents working in this repository.

## Product shape

Mediaar is **one Cargo package / one binary** (`crates/mediaar`) with two frontends:

- `mediaar desktop` — GPUI + Catppuccin Latte/Mocha
- `mediaar tui` — ratatui + `catppuccin` crate, same flavors

Reusable desktop widgets live in **library** crate `kbgpui` (`ui` module, Zed-shaped). Do **not** split desktop into a second installable crate.

## Source of truth

| Concern | Source |
| --- | --- |
| CLI commands | usage-rs derives in `crates/mediaar/src/cli/` |
| Checked-in CLI spec | `usage/mediaar.usage.kdl` (regenerate via `__usage_spec__`) |
| Tool versions / tasks | `mise.toml` |
| Infra overview | `docs/infra.md` |
| Zed GPUI/`ui` navigation | skill `zed-ui` (`.cursor/skills/zed-ui/`) |
| Package README | `crates/mediaar/README.md` |
| Tauri→GPUI comparison | tag `comparison/gpui-vs-tauri` (`benches/`; not on `main`) |

When adding/changing CLI flags or subcommands, update the Rust CLI first, then refresh `usage/mediaar.usage.kdl`.

## Hard rules

1. **Single binary** — desktop UI is native GPUI compiled into `mediaar`.
2. **Catppuccin only for themes** — Latte (light) and Mocha (dark) on both desktop and TUI.
3. **Minimal diffs** — no drive-by refactors, no unsolicited markdown beyond what was asked, match existing style.

## Where to edit

| Change | Path |
| --- | --- |
| CLI / dispatch | `crates/mediaar/src/cli/` |
| Desktop host (GPUI) | `crates/mediaar/src/desktop.rs` |
| Desktop widgets | `crates/kbgpui` (`ui` / `theme`, same layout as Zed) |
| TUI | `crates/mediaar/src/tui/` |
| Fluent locales | `crates/mediaar/locales/{en,es}/{common,desktop,tui}.ftl` |
| Icons / packaging | `crates/mediaar/icons/`, `packaging/` |
| User install script | `install.sh` |
| User uninstall script | `uninstall.sh` |
| Release packaging | `scripts/pack-release.sh`, `packaging/` |

## Build / verify

```sh
cargo build -p mediaar --release
./target/release/mediaar --help
./target/release/mediaar tui      # needs a real TTY
./target/release/mediaar desktop  # GPUI welcome window
```

## Do not

- Reintroduce Tauri / Solid / Vite without an explicit decision (see `docs/infra.md`)
- Check experiment `benches/` metrics back into `main` (keep them on `comparison/gpui-vs-tauri`)
- Add purple-default generic AI chrome; stick to Catppuccin Latte/Mocha and the existing welcome composition
- Commit secrets (`.env`), `node_modules`, or unrelated lockfile churn

## Commit hygiene

- Commit only when asked
- Prefer small, purpose-led messages (why over what)
- Do not push unless asked
