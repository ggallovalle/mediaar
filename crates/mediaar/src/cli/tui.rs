use usage::{Args, Run};

/// Start the tui app
#[derive(Args)]
pub(crate) struct Tui;

impl Run for Tui {
    type Output = ();

    fn run(self) {
        if let Err(err) = crate::tui::run() {
            eprintln!("tui error: {err}");
            std::process::exit(1);
        }
    }
}
