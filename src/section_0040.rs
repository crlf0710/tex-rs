//! @ Several of the elementary string operations are performed using \.{WEB}
//! macros instead of \PASCAL\ procedures, because many of the
//! operations are done quite frequently and we want to avoid the
//! overhead of procedure calls. For example, here is
//! a simple macro that computes the length of a string.
//! @.WEB@>
//
// @d length(#)==(str_start[#+1]-str_start[#]) {the number of characters
//   in string number \#}
pub(crate) fn length(globals: &TeXGlobals, s: integer) -> integer {
    globals.str_start[(s + 1) as u32] - globals.str_start[s as u32]
}

use crate::TeXGlobals;
use crate::pascal::integer;
