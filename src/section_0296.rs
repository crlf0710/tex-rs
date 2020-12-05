//! @ The |print_meaning| subroutine displays |cur_cmd| and |cur_chr| in
//! symbolic form, including the expansion of a macro or mark.
//
// @p procedure print_meaning;
pub(crate) fn print_meaning(globals: &mut TeXGlobals) {
    // begin print_cmd_chr(cur_cmd,cur_chr);
    print_cmd_chr(globals, globals.cur_cmd, globals.cur_chr);
    // if cur_cmd>=call then
    if globals.cur_cmd >= call {
        // begin print_char(":"); print_ln; token_show(cur_chr);
        print_char(make_globals_io_string_view!(globals), ASCII_code_literal!(b':'));
        print_ln(make_globals_io_view!(globals));
        token_show(globals, globals.cur_chr.get() as _);
        // end
    }
    // else if cur_cmd=top_bot_mark then
    else if globals.cur_cmd == top_bot_mark {
        todo!();
        // begin print_char(":"); print_ln;
        // token_show(cur_mark[cur_chr]);
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoView;
use crate::section_0004::TeXGlobalsIoStringView;
use crate::section_0057::print_ln;
use crate::section_0058::print_char;
use crate::section_0210::call;
use crate::section_0210::top_bot_mark;
use crate::section_0295::token_show;
use crate::section_0298::print_cmd_chr;