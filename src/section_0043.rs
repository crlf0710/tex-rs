//! @ Once a sequence of characters has been appended to |str_pool|, it
//! officially becomes a string when the function |make_string| is called.
//! This function returns the identification number of the new string as its
//! value.
//
// @p function make_string : str_number; {current string enters the pool}
/// current string enters the pool
pub(crate) fn make_string(globals: &mut TeXGlobals) -> str_number {
    // begin if str_ptr=max_strings then
    if globals.str_ptr.get() == max_strings as _ {
        //   overflow("number of strings",max_strings-init_str_ptr);
    }
    // @:TeX capacity exceeded number of strings}{\quad number of strings@>
    // incr(str_ptr); str_start[str_ptr]:=pool_ptr;
    incr!(globals.str_ptr);
    globals.str_start[globals.str_ptr] = globals.pool_ptr;
    // make_string:=str_ptr-1;
    return globals.str_ptr - 1;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0011::max_strings;
use crate::section_0038::str_number;
