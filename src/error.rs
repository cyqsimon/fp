use std::{ffi::NulError, io};

/// All errors of FP.
#[derive(Debug, thiserror::Error)]
pub enum FPError {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    NulByte(#[from] NulError),
}
