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
macro_rules! info {
    ($globals:expr, $val:expr) => {
        $globals.mem[$val][crate::section_0113::MEMORY_WORD_HH_LH]
    };
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
