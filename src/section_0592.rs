//! @* \[32] Shipping pages out.
//! After considering \TeX's eyes and stomach, we come now to the bowels.
//! @^bowels@>
//!
//! The |ship_out| procedure is given a pointer to a box; its mission is
//! to describe that box in \.{DVI} form, outputting a ``page'' to |dvi_file|.
//! The \.{DVI} coordinates $(h,v)=(0,0)$ should correspond to the upper left
//! corner of the box being shipped.
//!
//! Since boxes can be inside of boxes inside of boxes, the main work of
//! |ship_out| is done by two mutually recursive routines, |hlist_out|
//! and |vlist_out|, which traverse the hlists and vlists inside of horizontal
//! and vertical boxes.
//!
//! As individual pages are being processed, we need to accumulate
//! information about the entire set of pages, since such statistics must be
//! reported in the postamble. The global variables |total_pages|, |max_v|,
//! |max_h|, |max_push|, and |last_bop| are used to record this information.
//!
//! The variable |doing_leaders| is |true| while leaders are being output.
//! The variable |dead_cycles| contains the number of times an output routine
//! has been initiated since the last |ship_out|.
//!
//! A few additional global variables are also defined here for use in
//! |vlist_out| and |hlist_out|. They could have been local variables, but
//! that would waste stack space when boxes are deeply nested, since the
//! values of these variables are not needed during recursive calls.
//! @^recursion@>
//
// @<Glob...@>=
// @!total_pages:integer; {the number of pages that have been shipped out}
/// the number of pages that have been shipped out
#[globals_struct_field(TeXGlobals)]
pub(crate) static total_pages: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::pascal::integer;

// @!max_v:scaled; {maximum height-plus-depth of pages shipped so far}
// @!max_h:scaled; {maximum width of pages shipped so far}
// @!max_push:integer; {deepest nesting of |push| commands encountered so far}
// @!last_bop:integer; {location of previous |bop| in the \.{DVI} output}
// @!dead_cycles:integer; {recent outputs that didn't ship anything out}
// @!doing_leaders:boolean; {are we inside a leader box?}
// @#
// @!c,@!f:quarterword; {character and font in current |char_node|}
// @!rule_ht,@!rule_dp,@!rule_wd:scaled; {size of current rule being output}
// @!g:pointer; {current glue specification}
// @!lq,@!lr:integer; {quantities used in calculations for leaders}

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
