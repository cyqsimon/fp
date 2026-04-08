use std::{
    ffi::{CString, NulError, c_int},
    fs::metadata,
    os::unix::fs::{FileTypeExt, MetadataExt},
};

use camino::Utf8Path;
use libc::{R_OK, S_ISGID, S_ISUID, S_ISVTX, W_OK, X_OK, euidaccess};

use crate::error::FPError;

/// Wrapper around `euidaccess`.
fn check_access(path: &Utf8Path, access_permission_mask: c_int) -> Result<bool, NulError> {
    let c_path = CString::new(path.as_str())?;
    match unsafe { euidaccess(c_path.as_ptr(), access_permission_mask) } {
        0 => Ok(true),
        -1 => Ok(false),
        ret => unreachable!("euidaccess returned {ret} unexpectedly"),
    }
}

/// All supported filters.
#[derive(Copy, Clone, Debug)]
pub enum Filter {
    IsBlock,
    IsChar,
    IsDir,
    Exists,
    IsFile,
    HasSetGid,
    IsSymlink,
    HasStickyBit,
    IsFifo,
    CanRead,
    IsSocket,
    NotEmpty,
    HasSetUid,
    CanWrite,
    CanExecute,
}
impl Filter {
    /// Apply the filter on a path.
    /// Returns `true` if the path satisfies the filter's condition.
    pub fn apply(&self, path: &Utf8Path) -> Result<bool, FPError> {
        let res = match self {
            Self::IsBlock => metadata(path)?.file_type().is_block_device(),
            Self::IsChar => metadata(path)?.file_type().is_char_device(),
            Self::IsDir => metadata(path)?.is_dir(),
            Self::Exists => path.try_exists()?,
            Self::IsFile => metadata(path)?.is_file(),
            Self::HasSetGid => metadata(path)?.mode() & S_ISUID != 0,
            Self::IsSymlink => metadata(path)?.is_symlink(),
            Self::HasStickyBit => metadata(path)?.mode() & S_ISVTX != 0,
            Self::IsFifo => metadata(path)?.file_type().is_fifo(),
            Self::CanRead => check_access(path, R_OK)?,
            Self::IsSocket => metadata(path)?.file_type().is_socket(),
            Self::NotEmpty => metadata(path)?.len() > 0,
            Self::HasSetUid => metadata(path)?.mode() & S_ISGID != 0,
            Self::CanWrite => check_access(path, W_OK)?,
            Self::CanExecute => check_access(path, X_OK)?,
        };
        Ok(res)
    }
}
