use std::io;

/// All errors of FP.
#[derive(Debug, displaydoc::Display, thiserror::Error)]
pub enum FPError {
    /// OS IO error: {0:?}.
    Io(io::Error),
    /// Logger initialise error: {0:?}.
    Logger(log::SetLoggerError),
}
impl From<io::Error> for FPError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<log::SetLoggerError> for FPError {
    fn from(err: log::SetLoggerError) -> Self {
        Self::Logger(err)
    }
}
