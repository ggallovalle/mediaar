# Zed clone map

All paths relative to the Zed clone root.

## Layers

```text
crates/zed            editor binary
crates/workspace      panes, docks (uses ui)
crates/editor         buffer view (not the widget kit)
crates/ui             Switch, Button, Label, Modal…
crates/theme          cx.theme(), ThemeColors, StatusColors
crates/icons          IconName
crates/ui_input       single-line field on ui
crates/ui_macros      RegisterComponent, derive_dynamic_spacing
crates/component      Component previews
crates/gpui           App, Window, Entity, div(), Styled
crates/gpui_platform  OS backends
```

Licenses: `gpui` / `gpui_platform` Apache-2.0; `ui` / `theme` / most of the editor GPL-3.0-or-later.

## Crate roots (often not lib.rs)

| Crate | Lib path |
| --- | --- |
| `gpui` | `crates/gpui/src/gpui.rs` |
| `ui` | `crates/ui/src/ui.rs` |
| `theme` | `crates/theme/src/theme.rs` |
| `icons` | `crates/icons/src/icons.rs` |
| `ui_input` | `crates/ui_input/src/ui_input.rs` |
| `ui_macros` | `crates/ui_macros/src/ui_macros.rs` |
| `gpui_platform` | `crates/gpui_platform/src/gpui_platform.rs` |

## Search

| Want | Where |
| --- | --- |
| Widget index | `crates/ui/src/components.rs` |
| Switch / checkbox | `crates/ui/src/components/toggle.rs` |
| Buttons | `crates/ui/src/components/button/` |
| Label | `crates/ui/src/components/label/` |
| `h_flex` / `v_flex` | `crates/ui/src/components/stack.rs`, `traits/styled_ext.rs` |
| `ToggleState` | `crates/ui/src/traits/toggleable.rs` |
| `Color::Muted` | `crates/ui/src/styles/color.rs` |
| `DynamicSpacing` | `crates/ui/src/styles/spacing.rs` |
| `ThemeColors` | `crates/theme/src/styles/colors.rs` |
| Status / info | `crates/theme/src/styles/status.rs` |
| `ActiveTheme` | `crates/theme/src/theme.rs` |
| Div / click / hover | `crates/gpui/src/elements/div.rs` |
| Entity ownership | `crates/gpui/src/_ownership_and_data_flow.rs` |
| A11y | `crates/gpui/src/_accessibility.rs`, `window/a11y.rs` |
| GPUI-only demos | `crates/gpui/examples/` (hand-rolled switch in `a11y.rs` — ignore) |
| Call sites | `rg 'use ui::prelude' crates --glob '*.rs'` |

Zed feature code:

```rust
use ui::prelude::*;
use ui::{SwitchField, ToggleState};
```

## GPUI

- `App` owns entity state. `cx.new` → `Entity<T>`. Update via `entity.update(cx, |this, cx| { …; cx.notify(); })`.
- `Render` = retained view. `RenderOnce` + `IntoElement` = stateless widget.
- Style methods: `styled.rs` + `gpui_macros`. Interaction: `elements/div.rs` (`.id()`, `.on_click()`, `.hover()`, `.group_hover()`).
- Clicks: bubble phase. Nested handlers both fire without `cx.stop_propagation()`.
- crates.io `gpui 0.2.2` (Mediaar) is older than this tree: no AccessKit `.role` / `aria_*`. Copy geometry from `ui`; don’t assume git GPUI APIs exist.

## `ui` tree

```text
ui.rs  prelude.rs  component_prelude.rs  components.rs
components/toggle.rs  button/  label/  stack.rs  icon.rs  list/  modal.rs …
styles/{color,spacing,typography,units,elevation,animation}.rs
traits/{toggleable,styled_ext,clickable,disableable}.rs
utils.rs   # is_light(cx)
```

Widget shape:

```rust
#[derive(IntoElement)]
pub struct Switch { /* … */ }

impl Switch {
    pub fn new(id: impl Into<ElementId>, state: ToggleState) -> Self { … }
    pub fn on_click(self, impl Fn(&ToggleState, &mut Window, &mut App)) -> Self { … }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement { … }
}
```

- Colors from `cx.theme()`, except `SwitchColor::Custom(Hsla)`.
- Space with `DynamicSpacing::BaseXX.rems(cx)` / `.px(cx)` (density Compact/Default/Comfortable). Switch: Base32×Base20 track, Base12 thumb, Base02 pad, Base06 gap. Macro: `crates/ui_macros/src/dynamic_spacing.rs`.
- Labels: `Label::new("…").size(LabelSize::Small).color(Color::Muted)`.
- `RegisterComponent` + `preview()` is Zed’s gallery; skip in kbgpui.

Controls: **Switch** / **SwitchField** / **Checkbox** in `toggle.rs`. **ToggleButton** is a selected-looking button (`button/toggle_button.rs`), not a pill. `ButtonLike` is the button base.

`Switch::new` sets `label_position: None`, so `.label()` is a no-op until `.label_position(Some(End))`.

## `theme`

- `Appearance::{Light,Dark}`, `ActiveTheme` on `App`
- `Theme::colors()` / `Theme::status()`
- Switch on: `status.info.opacity(0.4)` + `text_accent.opacity(0.2)`. Off: `element_disabled` + `border`.
- `ui` `is_light`: `cx.theme().appearance.is_light()`

## Feature crates (consumers, not the kit)

`workspace`, `editor`, `project_panel`, `settings_ui`, `title_bar`, `picker`, `agent_ui`. App: `crates/zed/src/main.rs`.

## Mediaar (`kbgpui`)

| Zed | Mediaar |
| --- | --- |
| `use ui::prelude::*;` | `use kbgpui::ui::prelude::*;` |
| `crates/ui/src/ui.rs` | `crates/kbgpui/src/ui.rs` |
| `crates/theme` | `crates/kbgpui/src/theme.rs` |
| user themes | Catppuccin Latte / Mocha |
| `DynamicSpacing` + density | default-density values |
| AccessKit on Switch | omitted |
| Host | `crates/mediaar/src/desktop.rs` |

## Anti-patterns

- Hand-rolled pill in `desktop.rs`
- Treating `gpui/examples/a11y.rs` as Switch
- `struct Switch` grep in `gpui` (`SwitchAnchor`, LSP switch-header)
- Assuming crates.io GPUI includes Zed `ui`
- Copying `editor`/`workspace` when you needed `ui`
- Semantic `Color` used as `Hsla`
