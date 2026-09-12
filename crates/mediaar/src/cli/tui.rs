use usage::{Args, RunWith};

use super::CliState;

/// Start the tui app
#[derive(Args)]
pub(crate) struct Tui;

impl RunWith<CliState> for Tui {
    type Output = ();

    fn run_with(self, state: CliState) {
        if let Err(err) = crate::tui::run(state) {
            eprintln!("tui error: {err}");
            std::process::exit(1);
        }
    }
}
