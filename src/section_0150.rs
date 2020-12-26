//! @ A glue specification has a halfword reference count in its first word,
//! @^reference counts@>
//! representing |null| plus the number of glue nodes that point to it (less one).
//! Note that the reference count appears in the same position as
//! the |link| field in list nodes; this is the field that is initialized
//! to |null| when a node is allocated, and it is also the field that is flagged
//! by |empty_flag| in empty nodes.
//!
//! Glue specifications also contain three |scaled| fields, for the |width|,
//! |stretch|, and |shrink| dimensions. Finally, there are two one-byte
//! fields called |stretch_order| and |shrink_order|; these contain the
//! orders of infinity (|normal|, |fil|, |fill|, or |filll|)
//! corresponding to the stretch and shrink values.
//
// @d glue_spec_size=4 {number of words to allocate for a glue specification}
/// number of words to allocate for a glue specification
pub(crate) const glue_spec_size: quarterword = 4;
// @d glue_ref_count(#) == link(#) {reference count of a glue specification}
/// reference count of a glue specification
macro_rules! glue_ref_count {
    ($globals:expr, $ptr:expr) => {
        link!($globals, $ptr)
    }
}
// @d stretch(#) == mem[#+2].sc {the stretchability of this glob of glue}
/// the stretchability of this glob of glue
macro_rules! stretch {
    ($globals:expr, $ptr:expr) => {
        $globals.mem[$ptr + 2][crate::section_0101::MEMORY_WORD_SC]
    }
}
// @d shrink(#) == mem[#+3].sc {the shrinkability of this glob of glue}
/// the shrinkability of this glob of glue
macro_rules! shrink {
    ($globals:expr, $ptr:expr) => {
        $globals.mem[$ptr + 3][crate::section_0101::MEMORY_WORD_SC]
    }
}
// @d stretch_order == type {order of infinity for stretching}
/// order of infinity for stretching
macro_rules! stretch_order {
    ($globals:expr, $ptr:expr) => {
        r#type!($globals, $ptr)
    }
}
// @d shrink_order == subtype {order of infinity for shrinking}
/// order of infinity for shrinking
macro_rules! shrink_order {
    ($globals:expr, $ptr:expr) => {
        subtype!($globals, $ptr)
    }
}

// @d fil=1 {first-order infinity}
// @d fill=2 {second-order infinity}
// @d filll=3 {third-order infinity}
//
// @<Types...@>=
// @!glue_ord=normal..filll; {infinity to the 0, 1, 2, or 3 power}
/// infinity to the 0, 1, 2, or 3 power
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum glue_ord {
    normal = 0,
    /// first-order infinity
    fil = 1,
    /// second-order infinity
    fill = 2,
    /// third-order infinity
    filll = 3,
}

pub(crate) type normal_TYPENUM = U0;
pub(crate) type filll_TYPENUM = U3;

impl glue_ord {
    pub(crate) fn get(self) -> u8 {
        self as u8
    }
}

impl From<u8> for glue_ord {
    fn from(val: u8) -> glue_ord {
        match val {
            0 => glue_ord::normal,
            1 => glue_ord::fil,
            2 => glue_ord::fill,
            3 => glue_ord::filll,
            _ => unreachable!(),
        }
    }
}

use crate::section_0113::quarterword;
use typenum::{U0, U3};
