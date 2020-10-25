//! @ The following subroutine compares string |s| with another string of the
//! same length that appears in |buffer| starting at position |k|;
//! the result is |true| if and only if the strings are equal.
//! Empirical tests indicate that |str_eq_buf| is used in such a way that
//! it tends to return |true| about 80 percent of the time.
//
// @p function str_eq_buf(@!s:str_number;@!k:integer):boolean;
//   {test equality of strings}
/// test equality of strings
#[allow(unused_variables)]
pub(crate) fn str_eq_buf(globals: &mut TeXGlobals, s: str_number, k: integer) -> boolean {
    // label not_found; {loop exit}
    // var j: pool_pointer; {running index}
    // @!result: boolean; {result of comparison}
    // begin j:=str_start[s];
    // while j<str_start[s+1] do
    //   begin if so(str_pool[j])<>buffer[k] then
    //     begin result:=false; goto not_found;
    //     end;
    //   incr(j); incr(k);
    //   end;
    // result:=true;
    // not_found: str_eq_buf:=result;
    // end;
    todo!();
}

use crate::pascal::{boolean, integer};
use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
