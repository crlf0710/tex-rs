//! @ The procedure |print_esc| prints a string that is preceded by
//! the user's escape character (which is usually a backslash).
//
// @<Basic print...@>=
// procedure print_esc(@!s:str_number); {prints escape character, then |s|}
/// prints escape character, then `s`
pub(crate) fn print_esc(globals: &mut TeXGlobals, s: str_number) {
    // var c:integer; {the escape character code}
    // begin  @<Set variable |c| to the current escape character@>;
    // if c>=0 then if c<256 then print(c);
    // slow_print(s);
    slow_print(globals, s.get() as _);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0060::slow_print;
