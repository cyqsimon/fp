use std::{env, fs, io, path::PathBuf};

use clap::CommandFactory;
use clap_complete::{generate_to, shells::Shell::*};

include!("src/cli.rs");

fn main() -> Result<(), io::Error> {
    // generate completions
    // ----------
    let completion_dir = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR not set by cargo."));
    if completion_dir.exists() {
        fs::remove_dir_all(&completion_dir)?;
    }
    fs::create_dir(&completion_dir)?;

    let mut cmd = Cli::command();
    for shell in [Bash, Fish, Zsh] {
        let _path = generate_to(shell, &mut cmd, "fp", &completion_dir)?;
    }
    println!("cargo:rerun-if-changed=src/cli.rs");

    Ok(())
}
