//! @ Some operations are allowed only in privileged modes, i.e., in cases
//! that |mode>0|. The |privileged| function is used to detect violations
//! of this rule; it issues an error message and returns |false| if the
//! current |mode| is negative.
//
// @<Declare act...@>=
// function privileged:boolean;
pub(crate) fn privileged(globals: &mut TeXGlobals) -> TeXResult<boolean> {
    // begin if mode>0 then privileged:=true
    if mode!(globals) > 0 {
        crate::ok_nojump!(true)
    }
    // else  begin report_illegal_case; privileged:=false;
    else {
        report_illegal_case(globals)?;
        crate::ok_nojump!(false)
        // end;
    }
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0213::mode;
use crate::section_1050::report_illegal_case;
