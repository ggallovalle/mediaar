use usage::{Args, RunWith};

use super::CliState;

/// Start the desktop app
#[derive(Args)]
pub(crate) struct Desktop;

impl RunWith<CliState> for Desktop {
    type Output = ();

    fn run_with(self, state: CliState) {
        crate::desktop::run(state);
    }
}
