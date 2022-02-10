//! @ When \TeX\ is in display math mode, |cur_group=math_shift_group|,
//! so it is not necessary for the |start_eq_no| procedure to test for
//! this condition.
//
// @<Declare act...@>=
// procedure start_eq_no;
pub(crate) fn start_eq_no(globals: &mut TeXGlobals) {
    // begin saved(0):=cur_chr; incr(save_ptr);
    saved!(globals, 0) = globals.cur_chr.get() as _;
    incr!(globals.save_ptr);
    // @<Go into ordinary math mode@>;
    crate::section_1139::Go_into_ordinary_math_mode!(globals);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0274::saved;
