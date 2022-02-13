//! @ A control sequence that has been \.{\\def}'ed by the user is expanded by
//! \TeX's |macro_call| procedure.
//!
//! Before we get into the details of |macro_call|, however, let's consider the
//! treatment of primitives like \.{\\topmark}, since they are essentially
//! macros without parameters. The token lists for such marks are kept in a
//! global array of five pointers; we refer to the individual entries of this
//! array by symbolic names |top_mark|, etc. The value of |top_mark| is either
//! |null| or a pointer to the reference count of a token list.
//
// @d top_mark_code=0 {the mark in effect at the previous page break}
// @d first_mark_code=1 {the first mark between |top_mark| and |bot_mark|}
// @d bot_mark_code=2 {the mark in effect at the current page break}
// @d split_first_mark_code=3 {the first mark found by \.{\\vsplit}}
// @d split_bot_mark_code=4 {the last mark found by \.{\\vsplit}}
#[doc(hidden)]
#[derive(Clone, Copy)]
pub(crate) enum mark_code_kind {
    top_mark_code = 0,
    first_mark_code = 1,
    bot_mark_code = 2,
    split_first_mark_code = 3,
    split_bot_mark_code = 4,
}

impl mark_code_kind {
    pub(crate) fn get(self) -> u8 {
        self as u8
    }
}

// @d top_mark==cur_mark[top_mark_code]
pub(crate) macro top_mark($globals:expr) {
    $globals.cur_mark[crate::section_0382::mark_code_kind::top_mark_code]
}
// @d first_mark==cur_mark[first_mark_code]
pub(crate) macro first_mark($globals:expr) {
    $globals.cur_mark[crate::section_0382::mark_code_kind::first_mark_code]
}
// @d bot_mark==cur_mark[bot_mark_code]
pub(crate) macro bot_mark($globals:expr) {
    $globals.cur_mark[crate::section_0382::mark_code_kind::bot_mark_code]
}
// @d split_first_mark==cur_mark[split_first_mark_code]
pub(crate) macro split_first_mark($globals:expr) {
    $globals.cur_mark[crate::section_0382::mark_code_kind::split_first_mark_code]
}
// @d split_bot_mark==cur_mark[split_bot_mark_code]
//
// @<Glob...@>=
// @!cur_mark:array[top_mark_code..split_bot_mark_code] of pointer;
//   {token lists for marks}
/// token lists for marks
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_mark: mark_code_array<pointer> = mark_code_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0382::mark_code_array;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

type mark_code_array_LENGTH_TYPENUM = typenum::U5;

define_array_keyed_with_ranged_unsigned_integer_from_0_with_fixed_length!(
    pub(crate) mark_code_array[mark_code_kind] => u8; U8; mark_code_array_LENGTH_TYPENUM
);

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
