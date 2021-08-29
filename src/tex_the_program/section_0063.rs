//! @ The procedure |print_esc| prints a string that is preceded by
//! the user's escape character (which is usually a backslash).
//
// @<Basic print...@>=
// procedure print_esc(@!s:str_number); {prints escape character, then |s|}
/// prints escape character, then `s`
pub(crate) fn print_esc(globals: &mut TeXGlobals, s: str_number) {
    // var c:integer; {the escape character code}
    /// the escape character code
    let c;
    // begin  @<Set variable |c| to the current escape character@>;
    Set_variable_c_to_the_current_escape_character!(globals, c);
    // if c>=0 then if c<256 then print(c);
    if c >= 0 && c < 256 {
        print(globals, c);
    }
    // slow_print(s);
    slow_print(globals, s.get() as _);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0059::print;
use crate::section_0060::slow_print;
