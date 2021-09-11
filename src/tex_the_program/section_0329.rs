//! @ Conversely, the variables must be downdated when such a level of input
//! is finished:
//
// @p procedure end_file_reading;
#[allow(unused_variables)]
pub(crate) fn end_file_reading(globals: &mut TeXGlobals) {
    // begin first:=start; line:=line_stack[index];
    globals.first = start!(globals).into();
    globals.line = globals.line_stack[index!(globals)];
    // if name>17 then a_close(cur_file); {forget it}
    if name!(globals) > 17 {
        /// forget it
        a_close(&mut cur_file!(globals));
    }
    // pop_input; decr(in_open);
    pop_input!(globals);
    decr!(globals.in_open);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0028::a_close;
use crate::section_0302::index;
use crate::section_0302::name;
use crate::section_0302::start;
use crate::section_0304::cur_file;
use crate::section_0322::pop_input;
