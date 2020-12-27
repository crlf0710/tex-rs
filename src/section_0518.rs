//! @ Conversely, here is a routine that takes three strings and prints a file
//! name that might have produced them. (The routine is system dependent, because
//! some operating systems put the file area last instead of first.)
//! @^system dependencies@>
//
// @<Basic printing...@>=
// procedure print_file_name(@!n,@!a,@!e:integer);
pub(crate) fn print_file_name(globals: &mut TeXGlobals, n: integer, a: integer, e: integer) {
    // begin slow_print(a); slow_print(n); slow_print(e);
    slow_print(globals, a);
    slow_print(globals, n);
    slow_print(globals, e);
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0060::slow_print;
