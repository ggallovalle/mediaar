use crate::ui::{Color, prelude::*};
use gpui::svg;

/// Named icons used by `ui` widgets (Zed `IconName`, subset).
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum IconName {
    Check,
    ChevronDown,
    ChevronRight,
    ChevronUpDown,
    MagnifyingGlass,
}

impl IconName {
    pub fn path(self) -> &'static str {
        match self {
            Self::Check => "icons/check.svg",
            Self::ChevronDown => "icons/chevron_down.svg",
            Self::ChevronRight => "icons/chevron_right.svg",
            Self::ChevronUpDown => "icons/chevron_up_down.svg",
            Self::MagnifyingGlass => "icons/magnifying_glass.svg",
        }
    }
}

/// The size of an [`Icon`].
#[derive(Default, PartialEq, Copy, Clone)]
pub enum IconSize {
    /// 10px
    Indicator,
    /// 12px
    XSmall,
    /// 14px
    Small,
    #[default]
    /// 16px
    Medium,
    /// 48px
    XLarge,
    Custom(Rems),
}

impl IconSize {
    pub fn rems(self) -> Rems {
        match self {
            IconSize::Indicator => rems_from_px(10.0_f32),
            IconSize::XSmall => rems_from_px(12.0_f32),
            IconSize::Small => rems_from_px(14.0_f32),
            IconSize::Medium => rems_from_px(16.0_f32),
            IconSize::XLarge => rems_from_px(48.0_f32),
            IconSize::Custom(size) => size,
        }
    }
}

/// An icon element.
#[derive(Clone, IntoElement)]
pub struct Icon {
    name: IconName,
    color: Color,
    size: IconSize,
}

impl Icon {
    pub fn new(icon: IconName) -> Self {
        Self {
            name: icon,
            color: Color::default(),
            size: IconSize::default(),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn size(mut self, size: IconSize) -> Self {
        self.size = size;
        self
    }
}

impl From<IconName> for Icon {
    fn from(icon: IconName) -> Self {
        Icon::new(icon)
    }
}

impl RenderOnce for Icon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        svg()
            .size(self.size.rems())
            .flex_none()
            .path(self.name.path())
            .text_color(self.color.color(cx))
    }
}
