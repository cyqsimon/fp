use std::convert::identity;

use crate::cli::OpMode;

impl OpMode {
    /// Apply this binary operation to a list of filter outputs.
    pub fn reduce<I>(&self, filter_outputs: I) -> bool
    where
        I: IntoIterator<Item = bool>,
    {
        let mut it = filter_outputs.into_iter();
        match self {
            Self::And => it.all(identity),
            Self::Nand => !it.all(identity),
            Self::Or => it.any(identity),
            Self::Nor => !it.any(identity),
            Self::Xor => it.filter(|o| *o).count() % 2 == 1,
            Self::Xnor => it.filter(|o| *o).count() % 2 == 0,
        }
    }
}
