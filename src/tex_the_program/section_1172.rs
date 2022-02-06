//! @ The routine that scans the four mlists of a \.{\\mathchoice} is very
//! much like the routine that builds discretionary nodes.
//
// @<Declare act...@>=
// procedure append_choices;
pub(crate) fn append_choices(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin tail_append(new_choice); incr(save_ptr); saved(-1):=0;
    tail_append!(globals, new_choice(globals)?);
    incr!(globals.save_ptr);
    saved!(globals, @neg 1) = 0;
    // push_math(math_choice_group); scan_left_brace;
    push_math(globals, math_choice_group.into());
    scan_left_brace(globals)?;
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::incr;
use crate::section_0081::TeXResult;
use crate::section_0214::tail_append;
use crate::section_0269::math_choice_group;
use crate::section_0274::saved;
use crate::section_0403::scan_left_brace;
use crate::section_0689::new_choice;
use crate::section_1136::push_math;
