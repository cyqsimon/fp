use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};

#[derive(Clone, Debug, Parser)]
#[command(
    author,
    version,
    about,
    disable_help_flag = true,
    disable_version_flag = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Set the binary operation used when chaining filters.
    #[arg(long = "mode", value_enum, default_value_t)]
    pub op_mode: OpMode,

    /// Resolve relative paths against this directory
    /// instead of the current working directory.
    #[arg(long = "base-dir", value_name = "DIR")]
    pub base_dir: Option<PathBuf>,

    // The filters applied to the input paths.
    // See https://pubs.opengroup.org/onlinepubs/9699919799/utilities/test.html
    /// Show only block special files.
    #[arg(short = 'b')]
    pub is_block: bool,
    /// Show only character special files.
    #[arg(short = 'c')]
    pub is_char: bool,
    /// Show only directories.
    #[arg(short = 'd')]
    pub is_dir: bool,
    /// Show only files that exist.
    #[arg(short = 'e')]
    pub exists: bool,
    /// Show only regular files.
    #[arg(short = 'f')]
    pub is_file: bool,
    /// Show only files with setgid flag.
    #[arg(short = 'g')]
    pub has_set_gid: bool,
    /// Show only symlinks.
    #[arg(short = 'h', visible_short_alias = 'L')]
    pub is_symlink: bool,
    /// Show only files with sticky bit flag.
    #[arg(short = 'k')]
    pub has_sticky_bit: bool,
    /// Show only FIFO special files (named pipes).
    #[arg(short = 'p')]
    pub is_fifo: bool,
    /// Show only files that the current user has read permissions.
    #[arg(short = 'r')]
    pub can_read: bool,
    /// Show only socket special files.
    #[arg(short = 'S')]
    pub is_socket: bool,
    /// Show only non-empty files.
    #[arg(short = 's')]
    pub not_empty: bool,
    /// Show only files with setuid flag.
    #[arg(short = 'u')]
    pub has_set_uid: bool,
    /// Show only files that the current user has write permissions.
    #[arg(short = 'w')]
    pub can_write: bool,
    /// Show only files that the current user has execute permissions.
    #[arg(short = 'x')]
    pub can_execute: bool,

    // manual impl because the default `-h` conflicts
    /// Print help.
    #[arg(short = '?', long = "help", action = ArgAction::Help)]
    pub show_help: (),
    // manual impl because the default `-V` is non-conformant to POSIX
    /// Print version.
    #[arg(short = 'v', long = "version", action = ArgAction::Version)]
    pub show_version: (),
}

/// The binary operation used to chain multiple filters.
#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum OpMode {
    /// Path matches if it matches all of the specified filters. Default.
    #[default]
    And,

    /// Path matches if it does not match all of the specified filters.
    Nand,

    /// Path matches if it matches any one of the specified filters.
    Or,

    /// Path matches if it matches none of the specified filters.
    Nor,

    /// Path matches if an odd number of filters match.
    Xor,

    /// Path matches if an even number of filters match.
    Xnor,
}
