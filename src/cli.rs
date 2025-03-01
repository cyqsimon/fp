use std::convert::identity;

use clap::{Parser, ValueEnum};

#[derive(Clone, Debug, Parser)]
#[command(
    author,
    version,
    about,
    disable_help_flag = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Set the binary operation used when chaining filters.
    #[arg(long = "mode")]
    pub op_mode: OpMode,

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
}

/// The binary operation used to chain multiple filters.
#[derive(Copy, Clone, Debug, Default, ValueEnum)]
pub enum OpMode {
    /// Path matches if it matches any one of the specified filters. Default.
    #[default]
    Or,

    /// Path matches if it matches none of the specified filters.
    Nor,

    /// Path matches if it matches all of the specified filters.
    And,

    /// Path matches if it does not match any one of the specified filters.
    Nand,

    /// Path matches if an odd number of filters match.
    Xor,

    /// Path matches if an even number of filters match.
    Xnor,
}
impl OpMode {
    /// Apply this binary operation to a list of filter outputs.
    pub fn reduce<I>(&self, filter_outputs: I) -> bool
    where
        I: IntoIterator<Item = bool>,
    {
        let mut it = filter_outputs.into_iter();
        match self {
            Self::Or => it.any(identity),
            Self::Nor => !it.any(identity),
            Self::And => it.all(identity),
            Self::Nand => !it.all(identity),
            Self::Xor => it.filter(|o| *o).count() % 2 == 1,
            Self::Xnor => it.filter(|o| *o).count() % 2 == 0,
        }
    }
}
