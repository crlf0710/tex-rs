//! @* \[12] Displaying boxes.
//! We can reinforce our knowledge of the data structures just introduced
//! by considering two procedures that display a list in symbolic form.
//! The first of these, called |short_display|, is used in ``overfull box''
//! messages to give the top-level description of a list. The other one,
//! called |show_node_list|, prints a detailed description of exactly what
//! is in the data structure.
//!
//! The philosophy of |short_display| is to ignore the fine points about exactly
//! what is inside boxes, except that ligatures and discretionary breaks are
//! expanded. As a result, |short_display| is a recursive procedure, but the
//! recursion is never more than one level deep.
//! @^recursion@>
//!
//! A global variable |font_in_short_display| keeps track of the font code that
//! is assumed to be present when |short_display| begins; deviations from this
//! font will be printed.
//
// @<Glob...@>=
// @!font_in_short_display:integer; {an internal font number}
/// an internal font number
#[globals_struct_field(TeXGlobals)]
pub(crate) static font_in_short_display: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
