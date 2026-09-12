use usage::{Args, RunWith};

use crate::i18n::Locale;

/// Start the tui app
#[derive(Args)]
pub(crate) struct Tui;

impl RunWith<Locale> for Tui {
    type Output = ();

    fn run_with(self, locale: Locale) {
        if let Err(err) = crate::tui::run(locale) {
            eprintln!("tui error: {err}");
            std::process::exit(1);
        }
    }
}
