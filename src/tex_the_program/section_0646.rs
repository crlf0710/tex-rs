//! @ To figure out the glue setting, |hpack| and |vpack| determine how much
//! stretchability and shrinkability are present, considering all four orders
//! of infinity. The highest order of infinity that has a nonzero coefficient
//! is then used as if no other orders were present.
//!
//! For example, suppose that the given list contains six glue nodes with
//! the respective stretchabilities 3pt, 8fill, 5fil, 6pt, $-3$fil, $-8$fill.
//! Then the total is essentially 2fil; and if a total additional space of 6pt
//! is to be achieved by stretching, the actual amounts of stretch will be
//! 0pt, 0pt, 15pt, 0pt, $-9$pt, and 0pt, since only `fil' glue will be
//! considered. (The `fill' glue is therefore not really stretching infinitely
//! with respect to `fil'; nobody would actually want that to happen.)
//!
//! The arrays |total_stretch| and |total_shrink| are used to determine how much
//! glue of each kind is present. A global variable |last_badness| is used
//! to implement \.{\\badness}.
//
// @<Glob...@>=
// @!total_stretch, @!total_shrink: array[glue_ord] of scaled;
//   {glue found by |hpack| or |vpack|}
/// glue found by `hpack` or `vpack`
#[globals_struct_field(TeXGlobals)]
pub(crate) static total_stretch: glue_ord_array<scaled> = glue_ord_array::default();

#[globals_struct_field(TeXGlobals)]
pub(crate) static total_shrink: glue_ord_array<scaled> = glue_ord_array::default();

#[globals_struct_use(TeXGlobals)]
use crate::section_0646::glue_ord_array;

type glue_ord_array_LENGTH_TYPENUM = typenum::op!(filll_TYPENUM - normal_TYPENUM + U1);

define_array_keyed_with_ranged_unsigned_integer_with_fixed_start_and_length!(
    pub(crate) glue_ord_array[glue_ord] =>
    u8; U8; normal_TYPENUM; glue_ord_array_LENGTH_TYPENUM
);

// @!last_badness:integer; {badness of the most recently packaged box}
/// badness of the most recently packaged box
#[globals_struct_field(TeXGlobals)]
pub(crate) static last_badness: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use crate::section_0150::filll_TYPENUM;
use crate::section_0150::glue_ord;
use crate::section_0150::normal_TYPENUM;
use globals_struct::{globals_struct_field, globals_struct_use};
use typenum::U1;
