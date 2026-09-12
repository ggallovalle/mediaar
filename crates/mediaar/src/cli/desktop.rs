use usage::{Args, Run};

/// Start the desktop app
#[derive(Args)]
pub(crate) struct Desktop;

impl Run for Desktop {
    type Output = ();

    fn run(self) {
        crate::desktop::run();
    }
}
