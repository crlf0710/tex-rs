//! @ The |begin_file_reading| procedure starts a new level of input for lines
//! of characters to be read from a file, or as an insertion from the
//! terminal. It does not take care of opening the file, nor does it set |loc|
//! or |limit| or |line|.
//! @^system dependencies@>
//
// @p procedure begin_file_reading;
#[allow(unused_variables)]
pub(crate) fn begin_file_reading(globals: &mut TeXGlobals) {
    // begin if in_open=max_in_open then overflow("text input levels",max_in_open);
    // @:TeX capacity exceeded text input levels}{\quad text input levels@>
    // if first=buf_size then overflow("buffer size",buf_size);
    // @:TeX capacity exceeded buffer size}{\quad buffer size@>
    // incr(in_open); push_input; index:=in_open;
    incr!(globals.in_open);
    push_input!(globals);
    index!(globals) = globals.in_open.get();
    // line_stack[index]:=line; start:=first; state:=mid_line;
    globals.line_stack[index!(globals)] = globals.line;
    start!(globals) = globals.first.get();
    state!(globals) = mid_line;
    // name:=0; {|terminal_input| is now |true|}
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0303::mid_line;
