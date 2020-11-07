//! @ Now let's consider the ``driver''
//! routines by which \TeX\ deals with file names
//! in a system-independent manner.  First comes a procedure that looks for a
//! file name in the input by calling |get_x_token| for the information.
//
// @p procedure scan_file_name;
#[cfg_attr(feature = "trace", tracing::instrument(level = "trace", skip(globals)))]
pub(crate) fn scan_file_name(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // label done;
    // begin name_in_progress:=true; begin_name;
    globals.name_in_progress = true;
    begin_name(globals);
    // @<Get the next non-blank non-call...@>;
    Get_the_next_non_blank_non_call_token!(globals);
    region_forward_label! {
    |'done|
    {
    // loop@+begin if (cur_cmd>other_char)or(cur_chr>255) then {not a character}
    loop {
        trace_expr!("cur_cmd = {}", globals.cur_cmd);
        if globals.cur_cmd > other_char ||
            ASCII_code::from(globals.cur_chr).numeric_value() > 255 {
            /// not a character
            {
                // begin back_input; goto done;
                back_input(globals);
                goto_forward_label!('done);
            }
            //   end;
        }
        
        trace_expr!("cur_chr = {:?}", globals.cur_chr);
        // if not more_name(cur_chr) then goto done;
        if !more_name(globals, ASCII_code::from(globals.cur_chr)) {
            goto_forward_label!('done);
        }
        // get_x_token;
        get_x_token(globals)?;
        // end;
    }
    // done: end_name; name_in_progress:=false;
    }
    'done <-
    }
    end_name(globals);
    globals.name_in_progress = false;
    // end;
    return_nojump!();
}

use crate::section_0004::TeXGlobals;
use crate::section_0380::get_x_token;
use crate::section_0515::begin_name;
use crate::section_0517::end_name;
use crate::section_0325::back_input;
use crate::section_0207::other_char;
use crate::section_0018::ASCII_code;
use crate::section_0516::more_name;
use crate::section_0081::JumpOutToEndOfTEX;