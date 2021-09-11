//! @ The |scan_dimen| routine is similar to |scan_int|, but it sets |cur_val| to
//! a |scaled| value, i.e., an integral number of sp. One of its main tasks
//! is therefore to interpret the abbreviations for various kinds of units and
//! to convert measurements to scaled points.
//!
//! There are three parameters: |mu| is |true| if the finite units must be
//! `\.{mu}', while |mu| is |false| if `\.{mu}' units are disallowed;
//! |inf| is |true| if the infinite units `\.{fil}', `\.{fill}', `\.{filll}'
//! are permitted; and |shortcut| is |true| if |cur_val| already contains
//! an integer and only the units need to be considered.
//!
//! The order of infinity that was found in the case of infinite glue is returned
//! in the global variable |cur_order|.
//
// @<Glob...@>=
// @!cur_order:glue_ord; {order of infinity found by |scan_dimen|}
/// order of infinity found by `scan_dimen`
#[globals_struct_field(TeXGlobals)]
pub(crate) static cur_order: glue_ord = glue_ord::normal;

#[globals_struct_use(TeXGlobals)]
use crate::section_0150::glue_ord;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
