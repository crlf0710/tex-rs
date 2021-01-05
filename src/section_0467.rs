//! @ Here's part of the |expand| subroutine that we are now ready to complete:
//
// @p procedure ins_the_toks;
pub(crate) fn ins_the_toks(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin link(garbage):=the_toks; ins_list(link(temp_head));
    link!(globals, garbage) = the_toks(globals)?;
    ins_list!(globals, link!(globals, temp_head));
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0162::garbage;
use crate::section_0162::temp_head;
use crate::section_0465::the_toks;