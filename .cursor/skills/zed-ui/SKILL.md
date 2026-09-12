---
name: zed-ui
description: >-
  Navigates the Zed editor clone for GPUI widgets and ports them into kbgpui
  without reinventing controls. Use when building or changing Mediaar desktop
  GPUI UI, kbgpui, Switch/Button/Label, cx.theme(), or when the user mentions
  Zed ui, GPUI components, or copying from the Zed repo.
---

# Zed UI

Widgets live in Zed **`ui`**, not GPUI. GPUI is the engine (`div`, layout, events). Tokens live in **`theme`**. Mediaar’s analog is `crates/kbgpui` (`ui` + `theme`).

Clone (read-only reference): `$GHQ_ROOT/github.com/zed-industries/zed` or `$HOME/.local/share/ghq/github.com/zed-industries/zed`.

Do not invent a control. Open the Zed file, copy names/`RenderOnce` structure, map theme through `cx.theme()`.

File paths, search recipes, and crate roots: [reference.md](reference.md).

## Port a widget

1. Identify the control in Zed `crates/ui` (not `crates/gpui`, not `crates/editor`).
2. Read that file and its deps (`Label`, `h_flex`, `Color`, `DynamicSpacing`, `ToggleState`).
3. Port into `crates/kbgpui/src/ui/` with the **same module layout and constructor names**.
4. Colors: `cx.theme().colors()` / `.status()`. Add Catppuccin tokens in `crates/kbgpui/src/theme.rs` if missing. No hex in the widget.
5. Drop `RegisterComponent`, `KeyBinding`, AccessKit (`.role`, `aria_*`) unless Mediaar’s `gpui` crate actually has them (crates.io `0.2.2` does not).
6. Use from `crates/mediaar/src/desktop.rs` via `kbgpui::ui::prelude::*`. Init theme with `kbgpui::theme::init(cx)`.
7. Done when call sites match Zed (`SwitchField::new(id, Some(label), Some(desc), state, |state, window, cx| …)`) and `cargo clippy -p kbgpui -p mediaar -- -D warnings` is clean.

## Rules

- GPUI has no Switch/Button/Label. `Role::Switch` is a11y, not a widget. Prefer `ui::Switch` over `gpui/examples/a11y.rs`.
- `impl Render` = retained view. Widgets = `RenderOnce` + `#[derive(IntoElement)]`.
- Clicks bubble; nested `on_click` double-fires unless `cx.stop_propagation()`.
- `Switch::new` leaves `label_position: None`; `.label()` needs `.label_position(Some(End))` (Zed quirk).
- Semantic `Color` vs raw `Hsla`: `color.color(cx)`.
- `ui`/`theme` in Zed are GPL; port patterns, don’t paste files into Mediaar.
- One installable binary: `mediaar`. `kbgpui` is a library only.
