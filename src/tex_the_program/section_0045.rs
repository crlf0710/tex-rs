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
    let result: boolean;
    crate::region_forward_label! {
    |'not_found|
    {
        #[cfg(not(feature = "unicode_support"))]
        {
            todo!();
            // begin j:=str_start[s];
            // while j<str_start[s+1] do
            //   begin if so(str_pool[j])<>buffer[k] then
            //     begin result:=false; goto not_found;
            //     end;
            //   incr(j); incr(k);
            //   end;
        }
        #[cfg(feature = "unicode_support")]
        {
            let stored_bytes_len = globals.str_start[s + 1] - globals.str_start[s];
            let stored_bytes = (0..stored_bytes_len).map(|k|
                globals.str_pool[globals.str_start[s] + k]);
            let fss_utf_bytes = (k..).flat_map(|k|
                FssUtfEncodedIP32::new(globals.buffer[k as u16].0 as i32).into_iter());
            for (p1, p2) in stored_bytes.zip(fss_utf_bytes) {
                if p1.0 != p2 {
                    result = false;
                    crate::goto_forward_label!('not_found);
                }
            }

            use crate::unicode_support::FssUtfEncodedIP32;
        }
        // result:=true;
        result = true;
    }
    // not_found: str_eq_buf:=result;
    'not_found <-
    }
    return result;
    // end;
}

use crate::pascal::{boolean, integer};
use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
