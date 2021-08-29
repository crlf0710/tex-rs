//! @ The parameter to |init_span| is a pointer to the alignrecord where the
//! next column or group of columns will begin. A new semantic level is
//! entered, so that the columns will generate a list for subsequent packaging.
//
// @<Declare the procedure called |init_span|@>=
// procedure init_span(@!p:pointer);
pub(crate) fn init_span(globals: &mut TeXGlobals, p: pointer) -> TeXResult<()> {
    // begin push_nest;
    push_nest(globals);
    // if mode=-hmode then space_factor:=1000
    if mode!(globals) == -hmode {
        space_factor!(globals) = 1000;
    }
    // else  begin prev_depth:=ignore_depth; normal_paragraph;
    else {
        prev_depth!(globals) = ignore_depth;
        normal_paragraph(globals)?;
        // end;
    }
    // cur_span:=p;
    globals.cur_span = p;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::pointer;
use crate::section_0211::hmode;
use crate::section_0212::ignore_depth;
use crate::section_0216::push_nest;
use crate::section_1070::normal_paragraph;
