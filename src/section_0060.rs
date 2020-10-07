//! @ Control sequence names, file names, and strings constructed with
//! \.{\\string} might contain |ASCII_code| values that can't
//! be printed using |print_char|. Therefore we use |slow_print| for them:

// @<Basic print...@>=

// procedure slow_print(@!s:integer); {prints string |s|}

/// prints string `s`.
///
/// NOTE: `s` type changed to str_number here.
#[allow(unused_variables)]
pub(crate) fn slow_print(globals: &mut TeXGlobals, s: str_number) {
    // var j:pool_pointer; {current character code position}
    // begin if (s>=str_ptr) or (s<256) then print(s)
    // else begin j:=str_start[s];
    //   while j<str_start[s+1] do
    //     begin print(so(str_pool[j])); incr(j);
    //     end;
    //   end;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0038::str_number;
