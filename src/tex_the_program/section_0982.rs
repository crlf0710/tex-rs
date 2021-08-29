//! @ An array |page_so_far| records the heights and depths of everything
//! on the current page. This array contains six |scaled| numbers, like the
//! similar arrays already considered in |line_break| and |vert_break|; and it
//! also contains |page_goal| and |page_depth|, since these values are
//! all accessible to the user via |set_page_dimen| commands. The
//! value of |page_so_far[1]| is also called |page_total|.  The stretch
//! and shrink components of the \.{\\skip} corrections for each insertion are
//! included in |page_so_far|, but the natural space components of these
//! corrections are not, since they have been subtracted from |page_goal|.
//!
//! The variable |page_depth| records the depth of the current page; it has been
//! adjusted so that it is at most |page_max_depth|. The variable
//! |last_glue| points to the glue specification of the most recent node
//! contributed from the contribution list, if this was a glue node; otherwise
//! |last_glue=max_halfword|. (If the contribution list is nonempty,
//! however, the value of |last_glue| is not necessarily accurate.)
//! The variables |last_penalty| and |last_kern| are similar.  And
//! finally, |insert_penalties| holds the sum of the penalties associated with
//! all split and floating insertions.
//
// @d page_goal==page_so_far[0] {desired height of information on page being built}
/// desired height of information on page being built
macro_rules! page_goal {
    ($globals:expr) => {
        $globals.page_so_far[0]
    }
}
// @d page_total==page_so_far[1] {height of the current page}
/// height of the current page
macro_rules! page_total {
    ($globals:expr) => {
        $globals.page_so_far[1]
    }
}
// @d page_shrink==page_so_far[6] {shrinkability of the current page}
/// shrinkability of the current page
macro_rules! page_shrink {
    ($globals:expr) => {
        $globals.page_so_far[6]
    }
}

// @d page_depth==page_so_far[7] {depth of the current page}
/// depth of the current page
macro_rules! page_depth {
    ($globals:expr) => {
        $globals.page_so_far[7]
    }
}

// @<Glob...@>=
// @!page_so_far:array [0..7] of scaled; {height and glue of the current page}
/// height and glue of the current page
#[globals_struct_field(TeXGlobals)]
pub(crate) static page_so_far: [scaled; 8] = [scaled::zero(); 8];
// @!last_glue:pointer; {used to implement \.{\\lastskip}}
/// used to implement `\lastskip`
#[globals_struct_field(TeXGlobals)]
pub(crate) static last_glue: pointer = null;
// @!last_penalty:integer; {used to implement \.{\\lastpenalty}}
/// used to implement `\lastpenalty`
#[globals_struct_field(TeXGlobals)]
pub(crate) static last_penalty: integer = 0;
// @!last_kern:scaled; {used to implement \.{\\lastkern}}
/// used to implement `\lastkern`
#[globals_struct_field(TeXGlobals)]
pub(crate) static last_kern: scaled = scaled::zero();
// @!insert_penalties:integer; {sum of the penalties for insertions
//   that were held over}
/// sum of the penalties for insertions that were held over
#[globals_struct_field(TeXGlobals)]
pub(crate) static insert_penalties: integer = 0;

#[globals_struct_use(TeXGlobals)]
use crate::section_0101::scaled;

#[globals_struct_use(TeXGlobals)]
use crate::section_0115::pointer;

use crate::section_0004::TeXGlobals;
use globals_struct::{globals_struct_field, globals_struct_use};
