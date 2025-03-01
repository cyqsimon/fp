use std::io::{self, BufRead};

use camino::Utf8PathBuf;
use clap::Parser;
use log::{error, LevelFilter};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};

use crate::{cli::Cli, error::FPError};

mod cli;
mod error;
mod filters;

fn main() -> Result<(), FPError> {
    let Cli {
        op_mode,

        is_block,
        is_char,
        is_dir,
        exists,
        is_file,
        has_set_gid,
        is_symlink,
        is_fifo,
        can_read,
        is_socket,
        not_empty,
        has_set_uid,
        can_write,
        can_execute,
    } = Cli::parse();

    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    io::stdin()
        .lock()
        .lines()
        .filter_map(|line| match line {
            Ok(line) => Some(Utf8PathBuf::from(line)),
            Err(err) => {
                error!("Error reading line: {err:?}.");
                None
            }
        })
        // .filter(|path| todo!())
        // .filter(|path| todo!())
        // .filter(|path| todo!())
        // .filter(|path| todo!())
        // .filter(|path| todo!())
        .for_each(|path| println!("{path}"));

    Ok(())
}
