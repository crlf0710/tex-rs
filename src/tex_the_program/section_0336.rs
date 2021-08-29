//! @ Before getting into |get_next|, let's consider the subroutine that
//! is called when an `\.{\\outer}' control sequence has been scanned or
//! when the end of a file has been reached. These two cases are distinguished
//! by |cur_cs|, which is zero at the end of a file.
//
// @p procedure check_outer_validity;
#[allow(unused_variables)]
pub(crate) fn check_outer_validity(globals: &mut TeXGlobals) {
    // var p:pointer; {points to inserted token list}
    // @!q:pointer; {auxiliary pointer}
    // begin if scanner_status<>normal then
    //   begin deletions_allowed:=false;
    //   @<Back up an outer control sequence so that it can be reread@>;
    //   if scanner_status>skipping then
    //     @<Tell the user what has run away and try to recover@>
    //   else  begin print_err("Incomplete "); print_cmd_chr(if_test,cur_if);
    // @.Incomplete \\if...@>
    //     print("; all text was ignored after line "); print_int(skip_line);
    //     help3("A forbidden control sequence occurred in skipped text.")@/
    //     ("This kind of error happens when you say `\if...' and forget")@/
    //     ("the matching `\fi'. I've inserted a `\fi'; this might work.");
    //     if cur_cs<>0 then cur_cs:=0
    //     else help_line[2]:=@|
    //       "The file ended while I was skipping conditional text.";
    //     cur_tok:=cs_token_flag+frozen_fi; ins_error;
    //     end;
    //   deletions_allowed:=true;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;