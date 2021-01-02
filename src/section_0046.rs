//! @ Here is a similar routine, but it compares two strings in the string pool,
//! and it does not assume that they have the same length.
//
// @p function str_eq_str(@!s,@!t:str_number):boolean;
//   {test equality of strings}
/// test equality of strings
pub(crate) fn str_eq_str(globals: &mut TeXGlobals, s: str_number, t: str_number) -> boolean {
    // label not_found; {loop exit}
    // var j,@!k: pool_pointer; {running indices}
    /// running indices
    let (mut j, mut k): (pool_pointer, pool_pointer);
    // @!result: boolean; {result of comparison}
    /// result of comparison
    let mut result: boolean;
    region_forward_label!(
        |'not_found|
        {
            // begin result:=false;
            result = false;
            // if length(s)<>length(t) then goto not_found;
            if length(globals, s.get() as _) != length(globals, t.get() as _) {
                goto_forward_label!('not_found);
            }
            // j:=str_start[s]; k:=str_start[t];
            j = globals.str_start[s];
            k = globals.str_start[t];
            // while j<str_start[s+1] do
            while j < globals.str_start[s + 1] {
                // begin if str_pool[j]<>str_pool[k] then goto not_found;
                if globals.str_pool[j] != globals.str_pool[k] {
                    goto_forward_label!('not_found);
                }
                // incr(j); incr(k);
                incr!(j);
                incr!(k);
                // end;
            }
            // result:=true;
            result = true;
        }
        // not_found: str_eq_str:=result;
        'not_found <-
    );
    return result;
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
use crate::section_0038::pool_pointer;
use crate::section_0040::length;
