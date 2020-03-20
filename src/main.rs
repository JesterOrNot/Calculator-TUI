mod lib;

use cursive::Cursive;
use lib::*;
use structopt::StructOpt;

fn main() {
    let cli = Cli::from_args();
    let mut app = Cursive::default();
    let theme = default_theme(&app);
    select_command(&mut app, cli.cmd);
    app.set_theme(theme);
    app.run();
}
