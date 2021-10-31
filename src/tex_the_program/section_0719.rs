//! \[36\] Typesetting math formulas.
//!
//! \TeX's most important routine for dealing with formulas is called
//! |mlist_to_hlist|.  After a formula has been scanned and represented as an
//! mlist, this routine converts it to an hlist that can be placed into a box
//! or incorporated into the text of a paragraph. There are three implicit
//! parameters, passed in global variables: |cur_mlist| points to the first
//! node or noad in the given mlist (and it might be |null|); |cur_style| is a
//! style code; and |mlist_penalties| is |true| if penalty nodes for potential
//! line breaks are to be inserted into the resulting hlist. After
//! |mlist_to_hlist| has acted, |link(temp_head)| points to the translated hlist.
//!
//! Since mlists can be inside mlists, the procedure is recursive. And since this
//! is not part of \TeX's inner loop, the program has been written in a manner
//! that stresses compactness over efficiency.
//! @^recursion@>
//
// @<Glob...@>=
// @!cur_mlist:pointer; {beginning of mlist to be translated}
/// beginning of mlist to be translated
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_mlist: pointer = null;
// @!cur_style:small_number; {style code at current place in the list}
/// style code at current place in the list
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_style: small_number = small_number::default();
// @!cur_size:small_number; {size code corresponding to |cur_style|}
/// size code corresponding to `cur_style`
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_size: small_number = small_number::default();
// @!cur_mu:scaled; {the math unit width corresponding to |cur_size|}
/// the math unit width corresponding to `cur_size`
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_mu: scaled = scaled::default();
// @!mlist_penalties:boolean; {should |mlist_to_hlist| insert penalties?}
/// should `mlist_to_hlist` insert penalties?
#[globals_struct_field(TeXGlobals)]
pub(crate) static mlist_penalties: boolean = false;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;
#[globals_struct_use(TeXGlobals)]
use crate::section_0101::scaled;

use globals_struct::{globals_struct_field, globals_struct_use};
