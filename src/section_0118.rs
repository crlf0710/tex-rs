//! @ Let's consider the one-word memory region first, since it's the
//! simplest. The pointer variable |mem_end| holds the highest-numbered location
//! of |mem| that has ever been used. The free locations of |mem| that
//! occur between |hi_mem_min| and |mem_end|, inclusive, are of type
//! |two_halves|, and we write |info(p)| and |link(p)| for the |lh|
//! and |rh| fields of |mem[p]| when it is of this type. The single-word
//! free locations form a linked list
//! $$|avail|,\;\hbox{|link(avail)|},\;\hbox{|link(link(avail))|},\;\ldots$$
//! terminated by |null|.

// @d link(#) == mem[#].hh.rh {the |link| field of a memory word}
/// the `link` field of a memory word
macro_rules! link {
    ($globals:expr, $val:expr) => {
        $globals.mem[$val][crate::section_0113::MEMORY_WORD_HH_RH]
    };
}
// @d info(#) == mem[#].hh.lh {the |info| field of a memory word}
/// the `info` field of a memory word
macro_rules! info_inner {
    ($globals:expr, $val:expr) => {
        $globals.mem[$val][crate::section_0113::MEMORY_WORD_HH_LH]
    };
}

#[cfg(not(feature = "unicode_support"))]
macro_rules! info_tok {
    ($globals:expr, $val:expr) => {
        crate::section_0297::cur_tok_type::new(info_inner!($globals, $val))
    };
}

#[cfg(feature = "unicode_support")]
macro_rules! info_tok {
    ($globals:expr, $val:expr) => {
        crate::section_0297::cur_tok_type::new(
            crate::unicode_support::info_value($globals, info_inner!($globals, $val)))
    };
}

#[cfg(not(feature = "unicode_support"))]
macro_rules! info_tok_assign {
    ($globals:expr, $ptr:expr, $val:expr) => {
        info_inner!($globals, $ptr) = $val.get();
    }
}

#[cfg(feature = "unicode_support")]
macro_rules! info_tok_assign {
    ($globals:expr, $ptr:expr, $val:expr) => {
        info_inner!($globals, $ptr) = crate::unicode_support::register_info_value(
            $globals, $val.get());
    }
}

// @<Glob...@>=
// @!avail : pointer; {head of the list of available one-word nodes}
/// head of the list of available one-word nodes
#[globals_struct_field(TeXGlobals)]
pub(crate) static avail: pointer = null;
// @!mem_end : pointer; {the last one-word node used in |mem|}
/// the last one-word node used in `mem`
#[globals_struct_field(TeXGlobals)]
pub(crate) static mem_end: pointer = null;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
