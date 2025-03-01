use pastey::paste;
use std::convert::identity;

use crate::{
    cli::{Cli, OpMode},
    filters::Filter,
};

impl Cli {
    /// Get the list of user-specified filters.
    pub fn to_filters(&self) -> Vec<Filter> {
        let mut filters = vec![];

        /// "Add filter conditional".
        macro_rules! afc {
            ($filter: ident) => {
                if self.$filter {
                    filters.push(paste!(Filter::[<$filter:upper_camel>]));
                }
            };
        }

        afc!(is_block);
        afc!(is_char);
        afc!(is_dir);
        afc!(exists);
        afc!(is_file);
        afc!(has_set_gid);
        afc!(is_symlink);
        afc!(has_sticky_bit);
        afc!(is_fifo);
        afc!(can_read);
        afc!(is_socket);
        afc!(not_empty);
        afc!(has_set_uid);
        afc!(can_write);
        afc!(can_execute);

        filters
    }
}

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
