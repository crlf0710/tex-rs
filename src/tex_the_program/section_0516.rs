//! @ And here's the second. The string pool might change as the file name is
//! being scanned, since a new \.{\\csname} might be entered; therefore we keep
//! |area_delimiter| and |ext_delimiter| relative to the beginning of the current
//! string, instead of assigning an absolute address like |pool_ptr| to them.
//! @^system dependencies@>
//
// @p function more_name(@!c:ASCII_code):boolean;
#[allow(unused_variables)]
#[cfg_attr(
    feature = "trace_verbose",
    tracing::instrument(level = "trace", skip(globals))
)]
pub(crate) fn more_name(globals: &mut TeXGlobals, c: ASCII_code) -> boolean {
    // begin if c=" " then more_name:=false
    if c == ASCII_code_literal!(b' ') {
        return false;
    }
    // else  begin str_room(1); append_char(c); {contribute |c| to the current string}
    else {
        /// contribute `c` to the current string
        str_room(globals, c.len_bytes() as _);
        append_char(make_globals_string_view!(globals), c);
        // if (c=">")or(c=":") then
        if c == ASCII_code_literal!(b'>') || c == ASCII_code_literal!(b':') {
            // begin area_delimiter:=cur_length; ext_delimiter:=0;
            globals.area_delimiter = pool_pointer::new(cur_length!(globals) as _);
            globals.ext_delimiter = pool_pointer::zero();
            // end
        }
        // else if (c=".")and(ext_delimiter=0) then ext_delimiter:=cur_length;
        else if c == ASCII_code_literal!(b'.') && globals.ext_delimiter.is_zero() {
            globals.ext_delimiter = pool_pointer::new(cur_length!(globals) as _);
        }
        // more_name:=true;
        return true;
        // end;
    }
    // end;
}

use crate::pascal::boolean;
use crate::section_0004::make_globals_string_view;
use crate::section_0004::TeXGlobals;
use crate::section_0004::TeXGlobalsStringView;
use crate::section_0018::ASCII_code;
use crate::section_0018::ASCII_code_literal;
use crate::section_0038::pool_pointer;
use crate::section_0041::cur_length;
use crate::section_0042::append_char;
use crate::section_0042::str_room;
