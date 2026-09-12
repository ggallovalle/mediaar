mod cli;
mod desktop;
mod tui;

use cli::Mediaar;
use usage::Run;

fn main() {
    Mediaar::parse().command.run()
}
