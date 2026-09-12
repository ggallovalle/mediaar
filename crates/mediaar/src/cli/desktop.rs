use usage::{Args, RunWith};

use crate::i18n::Locale;

/// Start the desktop app
#[derive(Args)]
pub(crate) struct Desktop;

impl RunWith<Locale> for Desktop {
    type Output = ();

    fn run_with(self, locale: Locale) {
        crate::desktop::run(locale);
    }
}
