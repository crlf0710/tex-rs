//! @ Glue nodes in a horizontal list that is being paragraphed are not supposed to
//! include ``infinite'' shrinkability; that is why the algorithm maintains
//! four registers for stretching but only one for shrinking. If the user tries to
//! introduce infinite shrinkability, the shrinkability will be reset to finite
//! and an error message will be issued. A boolean variable |no_shrink_error_yet|
//! prevents this error message from appearing more than once per paragraph.
//
// @d check_shrinkage(#)==if (shrink_order(#)<>normal)and(shrink(#)<>0) then
pub(crate) macro check_shrinkage($globals:expr, $v:expr) {{
    if shrink_order!($globals, $v) as integer != glue_ord::normal as integer
        && shrink!($globals, $v) != scaled::zero()
    {
        // begin #:=finite_shrink(#);
        $v = finite_shrink($globals, $v)?;
        // end
    }
    use crate::pascal::integer;
    use crate::section_0101::scaled;
    use crate::section_0150::glue_ord;
    use crate::section_0150::shrink;
    use crate::section_0150::shrink_order;
    use crate::section_0826::finite_shrink;
}}

// @<Glob...@>=
// @!no_shrink_error_yet:boolean; {have we complained about infinite shrinkage?}
/// have we complained about infinite shrinkage?
#[globals_struct_field(TeXGlobals)]
pub(crate) static no_shrink_error_yet: boolean = true;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::boolean;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
