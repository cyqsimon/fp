use std::io::{self, BufRead};

use camino::Utf8PathBuf;
use clap::Parser;

use crate::{cli::Cli, error::FPError};

mod cli;
mod cli_ext;
mod error;
mod filters;

fn main() -> Result<(), FPError> {
    let cli_opts = Cli::parse();
    let filters = cli_opts.to_filters();

    io::stdin()
        .lock()
        .lines()
        // get fallibly the list of paths with their filter results
        .map(|line| -> Result<_, FPError> {
            let path = Utf8PathBuf::from(line?);
            let filter_outputs = filters
                .iter()
                .map(|f| f.apply(&path))
                .collect::<Result<Vec<_>, _>>()?;
            Ok((path, filter_outputs))
        })
        // log and discard errors
        .filter_map(|res| res.inspect_err(|err| eprintln!("{err}")).ok())
        // apply specified binary operation to each set of filter results
        .filter_map(|(path, outputs)| cli_opts.op_mode.reduce(outputs).then_some(path))
        .for_each(|path| println!("{path}"));

    Ok(())
}
