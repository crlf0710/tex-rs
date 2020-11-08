//! @ The |scan_left_brace| routine is called when a left brace is supposed to be
//! the next non-blank token. (The term ``left brace'' means, more precisely,
//! a character whose catcode is |left_brace|.) \TeX\ allows \.{\\relax} to
//! appear before the |left_brace|.

// @p procedure scan_left_brace; {reads a mandatory |left_brace|}
/// reads a mandatory `left_brace`
pub(crate) fn scan_left_brace(globals: &mut TeXGlobals) -> Result<(), JumpOutToEndOfTEX> {
    // begin @<Get the next non-blank non-relax non-call token@>;
    Get_the_next_non_blank_non_relax_non_call_token!(globals);
    trace_expr!("cur_cmd = {}", globals.cur_cmd);
    // if cur_cmd<>left_brace then
    if globals.cur_cmd != left_brace {
        // begin print_err("Missing { inserted");
        print_err!(globals, strpool_str!("Missing { inserted"));
        // @.Missing \{ inserted@>
        // help4("A left brace was mandatory here, so I've put one in.")@/
        //   ("You might want to delete and/or insert some corrections")@/
        //   ("so that I will find a matching right brace soon.")@/
        //   ("(If you're confused by all this, try typing `I}' now.)");
        // back_error; cur_tok:=left_brace_token+"{"; cur_cmd:=left_brace;
        // cur_chr:="{"; incr(align_state);
        // end;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::JumpOutToEndOfTEX;
use crate::section_0207::left_brace;
