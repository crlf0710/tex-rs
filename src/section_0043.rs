//! @ Once a sequence of characters has been appended to |str_pool|, it
//! officially becomes a string when the function |make_string| is called.
//! This function returns the identification number of the new string as its
//! value.
//
// @p function make_string : str_number; {current string enters the pool}
/// current string enters the pool
pub(crate) fn make_string() -> str_number {
    // begin if str_ptr=max_strings then
    //   overflow("number of strings",max_strings-init_str_ptr);
    // @:TeX capacity exceeded number of strings}{\quad number of strings@>
    // incr(str_ptr); str_start[str_ptr]:=pool_ptr;
    // make_string:=str_ptr-1;
    // end;
    todo!();
}

use crate::section_0038::str_number;