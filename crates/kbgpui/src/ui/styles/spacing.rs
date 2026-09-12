use gpui::{App, Pixels, Rems, px, rems};

use super::units::BASE_REM_SIZE_IN_PX;

/// A dynamic spacing system that adjusts spacing based on UI density.
///
/// The number following "Base" refers to the base pixel size
/// at the default rem size and spacing settings.
///
/// kbgpui currently uses Zed's **default** density values (the middle
/// column of `derive_dynamic_spacing!` in Zed's `ui` crate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DynamicSpacing {
    /// `0px`|`0px`|`0px (@16px/rem)`
    Base00,
    /// `1px`|`1px`|`2px (@16px/rem)`
    Base01,
    /// `1px`|`2px`|`4px (@16px/rem)`
    Base02,
    /// `2px`|`3px`|`4px (@16px/rem)`
    Base03,
    /// `2px`|`4px`|`6px (@16px/rem)`
    Base04,
    /// `3px`|`6px`|`8px (@16px/rem)`
    Base06,
    /// `4px`|`8px`|`10px (@16px/rem)`
    Base08,
    /// `10px`|`12px`|`14px (@16px/rem)`
    Base12,
    /// `14px`|`16px`|`18px (@16px/rem)`
    Base16,
    /// `18px`|`20px`|`22px (@16px/rem)`
    Base20,
    /// `20px`|`24px`|`28px (@16px/rem)`
    Base24,
    /// `28px`|`32px`|`36px (@16px/rem)`
    Base32,
    /// `36px`|`40px`|`44px (@16px/rem)`
    Base40,
    /// `44px`|`48px`|`52px (@16px/rem)`
    Base48,
}

impl DynamicSpacing {
    fn spacing_ratio(self) -> f32 {
        let default_px = match self {
            Self::Base00 => 0.0,
            Self::Base01 => 1.0,
            Self::Base02 => 2.0,
            Self::Base03 => 3.0,
            Self::Base04 => 4.0,
            Self::Base06 => 6.0,
            Self::Base08 => 8.0,
            Self::Base12 => 12.0,
            Self::Base16 => 16.0,
            Self::Base20 => 20.0,
            Self::Base24 => 24.0,
            Self::Base32 => 32.0,
            Self::Base40 => 40.0,
            Self::Base48 => 48.0,
        };
        default_px / BASE_REM_SIZE_IN_PX
    }

    /// Returns the spacing value in rems.
    pub fn rems(self, _cx: &App) -> Rems {
        rems(self.spacing_ratio())
    }

    /// Returns the spacing value in pixels.
    pub fn px(self, _cx: &App) -> Pixels {
        px(BASE_REM_SIZE_IN_PX * self.spacing_ratio())
    }
}
