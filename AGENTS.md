# AGENTS.md

Guidance for coding agents working in this repository.

## Product shape

Mediaar is **one Cargo package / one binary** (`crates/mediaar`) with two frontends:

- `mediaar desktop` — Tauri 2 + SolidJS 2 RC + Tailwind, Catppuccin Latte/Mocha
- `mediaar tui` — ratatui + `catppuccin` crate, same flavors

Do **not** split desktop into a second installable crate or require `pnpm tauri dev` at runtime for shipped builds.

## Source of truth

| Concern | Source |
| --- | --- |
| CLI commands | usage-rs derives in `crates/mediaar/src/cli/` |
| Checked-in CLI spec | `usage/mediaar.usage.kdl` (regenerate via `__usage_spec__`) |
| Tool versions / tasks | `mise.toml` |
| Infra overview | `INFRA.md` |
| Package README | `crates/mediaar/README.md` |

When adding/changing CLI flags or subcommands, update the Rust CLI first, then refresh `usage/mediaar.usage.kdl`.

## Hard rules

1. **Single binary** — desktop UI is embedded via Tauri `custom-protocol` + `ui/dist`. Never ship a flow that depends on a localhost Vite server for release.
2. **Keep `custom-protocol` and `image-png` enabled** on the `tauri` dependency. Missing `custom-protocol` causes “Could not connect to localhost”.
3. **Catppuccin only for themes** — Latte (light) and Mocha (dark) on both desktop and TUI.
4. **Solid 2 RC stack** — `solid-js` / `@solidjs/web` RC + `@solidjs/vite-plugin` (not `vite-plugin-solid` v2 / Solid 1).
5. **Minimal diffs** — no drive-by refactors, no unsolicited markdown beyond what was asked, match existing style.

## Where to edit

| Change | Path |
| --- | --- |
| CLI / dispatch | `crates/mediaar/src/cli/` |
| Desktop host | `crates/mediaar/src/desktop.rs` |
| TUI | `crates/mediaar/src/tui/` |
| Welcome UI | `crates/mediaar/ui/src/` |
| Fluent locales | `crates/mediaar/locales/{en,es}/{common,desktop,tui}.ftl` |
| Tauri config / icons / capabilities | `crates/mediaar/tauri.conf.json`, `icons/`, `capabilities/` |
| Build / embed hooks | `crates/mediaar/build.rs` |
| User install script | `install.sh` |
| User uninstall script | `uninstall.sh` |
| Release packaging | `scripts/pack-release.sh`, `packaging/` |

## Build / verify

```sh
mise run ui
cargo build -p mediaar --release
./target/release/mediaar --help
./target/release/mediaar tui      # needs a real TTY
./target/release/mediaar desktop  # should show welcome UI, not localhost errors
```

If `ui/dist` is missing, `build.rs` tries `pnpm` automatically. Prefer `mise run ui` explicitly in CI.

## Do not

- Reintroduce a separate `mediaar-desktop` / `src-tauri` workspace member for distribution
- Remove `custom-protocol` “for easier dev” without a documented alternate path
- Add purple-default generic AI chrome; stick to Catppuccin Latte/Mocha and the existing welcome composition
- Commit secrets (`.env`), `node_modules`, or unrelated lockfile churn

## Commit hygiene

- Commit only when asked
- Prefer small, purpose-led messages (why over what)
- Do not push unless asked
