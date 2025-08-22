mod cli;
mod punchclock;
use clap::Parser;
use cli::Cli;

use crate::punchclock::{Myr, Report};

fn main() {
    let cli = Cli::parse();

    let myr = Myr::new(cli.punch_file);
    let entries = myr.parser();

    println!("Day;Group;Name;Hours");
    print!("{}", entries);
}
