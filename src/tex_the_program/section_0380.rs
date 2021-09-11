//! @ Here is a recursive procedure that is \TeX's usual way to get the
//! next token of input. It has been slightly optimized to take account of
//! common cases.

// @p procedure get_x_token; {sets |cur_cmd|, |cur_chr|, |cur_tok|,
//   and expands macros}
/// sets `cur_cmd`, `cur_chr`, `cur_tok`, and expands macros
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn get_x_token(globals: &mut TeXGlobals) -> TeXResult<()> {
    // label restart,done;
    // begin restart: get_next;
    crate::region_forward_label! {
    |'done|
    {
    crate::region_backward_label! {
    'restart <-
    {
        get_next(globals)?;
        crate::trace_expr!("cur_chr = {:?}", globals.cur_chr);
        crate::trace_expr!("cur_cmd = {}", globals.cur_cmd);
        // @^inner loop@>
        // if cur_cmd<=max_command then goto done;
        if globals.cur_cmd <= max_command {
            crate::goto_forward_label!('done);
        }
        // if cur_cmd>=call then
        if globals.cur_cmd >= call {
            // if cur_cmd<end_template then macro_call
            if globals.cur_cmd < end_template {
                macro_call(globals)?;
            }
            // else  begin cur_cs:=frozen_endv; cur_cmd:=endv;
            else {
                globals.cur_cs = frozen_endv as _;
                globals.cur_cmd = endv;
                // goto done; {|cur_chr=null_list|}
                /// `cur_chr=null_list`
                crate::goto_forward_label!('done);
                // end
            }
        } else {
            // else expand;
            expand(globals)?;
        }
        // goto restart;
        crate::goto_backward_label!('restart);
    }
    |'restart|
    }
    }
    // done: if cur_cs=0 then cur_tok:=(cur_cmd*@'400)+cur_chr
    // else cur_tok:=cs_token_flag+cur_cs;
    // end;
    'done <-
    }
    if globals.cur_cs == 0 {
        globals.cur_tok = cur_tok_type::from_cmd_and_chr(globals.cur_cmd, globals.cur_chr);
    } else {
        globals.cur_tok = cur_tok_type::from_cs(globals.cur_cs);
    }
    crate::trace_expr!("cur_tok = {:?}", globals.cur_tok);

    crate::return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::halfword;
use crate::section_0207::endv;
use crate::section_0209::max_command;
use crate::section_0210::call;
use crate::section_0210::end_template;
use crate::section_0222::frozen_endv;
use crate::section_0289::cs_token_flag;
use crate::section_0297::cur_tok_type;
use crate::section_0341::get_next;
use crate::section_0366::expand;
use crate::section_0389::macro_call;
