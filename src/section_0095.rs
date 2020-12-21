//! @ The program might sometime run completely amok, at which point there is
//! no choice but to stop. If no previous error has been detected, that's bad
//! news; a message is printed that is really intended for the \TeX\
//! maintenance person instead of the user (unless the user has been
//! particularly diabolical).  The index entries for `this can't happen' may
//! help to pinpoint the problem.
//! @^dry rot@>
//
// @<Error hand...@>=
// procedure confusion(@!s:str_number);
//   {consistency check violated; |s| tells where}
/// consistency check violated; `s` tells where
#[allow(unused_variables)]
pub(crate) fn confusion(globals: &mut TeXGlobals, s: str_number) -> TeXResult<()> {
    // begin normalize_selector;
    normalize_selector(globals);
    // if history<error_message_issued then
    if globals.history < history_kind::error_message_issued {
        // begin print_err("This can't happen ("); print(s); print_char(")");
        print_err!(globals, strpool_str!("This can't happen ("));
        print(globals, s.get() as _);
        print_char(make_globals_io_string_log_view!(globals), ASCII_code_literal!(b')'));
        // @.This can't happen@>
        // help1("I'm broken. Please show this to someone who can fix can fix");
        help1!(globals, strpool_str!("I'm broken. Please show this to someone who can fix can fix"));
        // end
    }
    // else  begin print_err("I can't go on meeting you like this");
    else {
        todo!("print_err");
        // @.I can't go on...@>
        //   help2("One of your faux pas seems to have wounded me deeply...")@/
        //     ("in fact, I'm barely conscious. Please fix it and try again.");
        //   end;
    }
    // succumb;
    succumb(globals)?;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsIoStringLogView;
use crate::section_0038::str_number;
use crate::section_0058::print_char;
use crate::section_0059::print;
use crate::section_0076::history_kind;
use crate::section_0081::TeXResult;
use crate::section_0092::normalize_selector;
use crate::section_0093::succumb;