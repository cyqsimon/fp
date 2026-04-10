use std::io::{self, BufRead};

use camino::{Utf8Path, Utf8PathBuf};
use clap::Parser;

use crate::{cli::Cli, error::FPError};

mod cli;
mod cli_ext;
mod error;
mod filters;

fn main() -> Result<(), FPError> {
    #[cfg(not(target_family = "unix"))]
    compile_error!("fp is only written for Unix-like OSes for now. PRs welcomed.");

    let cli_opts = Cli::parse();
    let base_dir = cli_opts
        .base_dir
        .as_deref()
        .map(<&Utf8Path>::try_from)
        .transpose()
        .map_err(|err| err.into_io_error())?;
    let filters = cli_opts.to_filters();

    io::stdin()
        .lock()
        .lines()
        // get fallibly the list of paths with their filter results
        .map(|line| -> Result<_, FPError> {
            let path_in = Utf8PathBuf::from(line?);
            let path = match base_dir {
                Some(dir) => dir.join(&path_in),
                None => path_in.clone(),
            };
            let filter_outputs = filters
                .iter()
                .map(|f| f.apply(&path))
                .collect::<Result<Vec<_>, _>>()?;
            Ok((path_in, filter_outputs))
        })
        // log and discard errors
        .filter_map(|res| res.inspect_err(|err| eprintln!("{err}")).ok())
        // apply specified binary operation to each set of filter results
        .filter_map(|(path, outputs)| cli_opts.op_mode.reduce(outputs).then_some(path))
        .for_each(|path| println!("{path}"));

    Ok(())
}
