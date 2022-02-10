//! ` `

// @<Declare act...@>=
// procedure scan_box(@!box_context:integer);
//   {the next input should specify a box or perhaps a rule}
/// the next input should specify a box or perhaps a rule
#[allow(unused_variables)]
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace"))]
pub(crate) fn scan_box(globals: &mut TeXGlobals, box_context: integer) -> TeXResult<()> {
    // begin @<Get the next non-blank non-relax...@>;
    crate::section_0404::Get_the_next_non_blank_non_relax_non_call_token!(globals);
    // if cur_cmd=make_box then begin_box(box_context)
    if globals.cur_cmd == make_box {
        begin_box(globals, box_context)?;
    }
    // else if (box_context>=leader_flag)and((cur_cmd=hrule)or(cur_cmd=vrule)) then
    else if box_context >= leader_flag && (globals.cur_cmd == hrule || globals.cur_cmd == vrule) {
        // begin cur_box:=scan_rule_spec; box_end(box_context);
        globals.cur_box = scan_rule_spec(globals)?;
        box_end(globals, box_context)?;
        // end
    }
    // else  begin@t@>@;@/
    else {
        // print_err("A <box> was supposed to be here");@/
        print_err!(
            globals,
            crate::strpool_str!("A <box> was supposed to be here")
        );
        // @.A <box> was supposed to...@>
        // help3("I was expecting to see \hbox or \vbox or \copy or \box or")@/
        // ("something like that. So you might find something missing in")@/
        // ("your output. But keep trying; you can fix this later."); back_error;
        help3!(
            globals,
            crate::strpool_str!("I was expecting to see \\hbox or \\vbox or \\copy or \\box or"),
            crate::strpool_str!("something like that. So you might find something missing in"),
            crate::strpool_str!("your output. But keep trying; you can fix this later.")
        );
        back_error(globals)?;
        // end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0073::print_err;
use crate::section_0079::help3;
use crate::section_0081::TeXResult;
use crate::section_0208::hrule;
use crate::section_0208::make_box;
use crate::section_0208::vrule;
use crate::section_0327::back_error;
use crate::section_0463::scan_rule_spec;
use crate::section_1071::leader_flag;
use crate::section_1075::box_end;
use crate::section_1079::begin_box;
