//! @ When a control sequence is to be defined, by \.{\\def} or \.{\\let} or
//! something similar, the |get_r_token| routine will substitute a special
//! control sequence for a token that is not redefinable.
//
// @<Declare subprocedures for |prefixed_command|@>=
// procedure get_r_token;
#[allow(unused_variables)]
#[cfg_attr(
    feature = "trace_verbose",
    tracing::instrument(level = "trace", skip(globals))
)]
pub(crate) fn get_r_token(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label restart;
    crate::trace_expr_verbose!("cur_cs = {}", globals.cur_cs);
    // begin restart: repeat get_token;
    loop {
        get_token(globals)?;
        // until cur_tok<>space_token;
        if globals.cur_tok != space_token {
            break;
        }
    }
    // if (cur_cs=0)or(cur_cs>frozen_control_sequence) then
    if globals.cur_cs == 0 || globals.cur_cs > frozen_control_sequence as _ {
        crate::trace_expr_verbose!("cur_cs = {}", globals.cur_cs);
        todo!("not yet implemented in {}", file!());
        //   begin print_err("Missing control sequence inserted");
        // @.Missing control...@>
        //   help5("Please don't say `\def cs{...}', say `\def\cs{...}'.")@/
        //   ("I've inserted an inaccessible control sequence so that your")@/
        //   ("definition will be completed without mixing me up too badly.")@/
        //   ("You can recover graciously from this error, if you're")@/
        //   ("careful; see exercise 27.2 in The TeXbook.");
        // @:TeXbook}{\sl The \TeX book@>
        //   if cur_cs=0 then back_input;
        //   cur_tok:=cs_token_flag+frozen_protection; ins_error; goto restart;
        //   end;
    }
    // end;
    crate::ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0222::frozen_control_sequence;
use crate::section_0289::space_token;
use crate::section_0365::get_token;
