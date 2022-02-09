//! ` `

// @<Declare act...@>=
// procedure math_limit_switch;
pub(crate) fn math_limit_switch(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label exit;
    // begin if head<>tail then if type(tail)=op_noad then
    if head!(globals) != tail!(globals) && r#type!(globals, tail!(globals)) == op_noad {
        // begin subtype(tail):=cur_chr; return;
        subtype!(globals, tail!(globals)) = globals.cur_chr.get() as _;
        crate::return_nojump!();
        // end;
    }
    // print_err("Limit controls must follow a math operator");
    // @.Limit controls must follow...@>
    // help1("I'm ignoring this misplaced \limits or \nolimits command."); error;
    // exit:end;
    todo!("error");
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0213::head;
use crate::section_0213::tail;
use crate::section_0682::op_noad;
