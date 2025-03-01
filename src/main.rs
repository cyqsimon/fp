use std::io::{self, BufRead};

use camino::Utf8PathBuf;
use clap::Parser;

use crate::{cli::Cli, error::FPError};

mod cli;
mod cli_ext;
mod error;

fn main() -> Result<(), FPError> {
    let cli_opts = Cli::parse();

    io::stdin()
        .lock()
        .lines()
        .filter_map(|line| line.inspect_err(|err| eprintln!("{err}")).ok())
        .map(Utf8PathBuf::from)
        // TODO
        .for_each(|path| println!("{path}"));

    Ok(())
}
