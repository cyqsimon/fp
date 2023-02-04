use std::io;

/// All errors of FP.
#[derive(Debug, thiserror::Error)]
pub enum FPError {
    #[error(transparent)]
    Io(#[from] io::Error),
}
