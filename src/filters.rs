use camino::Utf8Path;

use crate::error::FPError;

#[derive(Copy, Clone, Debug)]
pub enum Filter {
    IsBlock,
    IsChar,
    IsDir,
    Exists,
    IsFile,
    HasSetGid,
    IsSymlink,
    IsFifo,
    CanRead,
    IsSocket,
    NotEmpty,
    HasSetUid,
    CanWrite,
    CanExecute,
}
impl Filter {
    pub fn apply(&self, path: &Utf8Path) -> Result<bool, FPError> {
        let res = match self {
            Self::IsBlock => todo!(),
            Self::IsChar => todo!(),
            Self::IsDir => todo!(),
            Self::Exists => path.try_exists()?,
            Self::IsFile => todo!(),
            Self::HasSetGid => todo!(),
            Self::IsSymlink => todo!(),
            Self::IsFifo => todo!(),
            Self::CanRead => todo!(),
            Self::IsSocket => todo!(),
            Self::NotEmpty => todo!(),
            Self::HasSetUid => todo!(),
            Self::CanWrite => todo!(),
            Self::CanExecute => todo!(),
        };

        Ok(res)
    }
}
