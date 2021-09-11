//! @ Conversely, when \TeX\ is finished on the current level, the former
//! state is restored by calling |pop_nest|. This routine will never be
//! called at the lowest semantic level, nor will it be called unless |head|
//! is a node that should be returned to free memory.
//
// @p procedure pop_nest; {leave a semantic level, re-enter the old}
/// leave a semantic level, re-enter the old
#[allow(unused_variables)]
pub(crate) fn pop_nest(globals: &mut TeXGlobals) {
    // begin free_avail(head); decr(nest_ptr); cur_list:=nest[nest_ptr];
    free_avail!(globals, head!(globals));
    decr!(globals.nest_ptr);
    globals.cur_list = globals.nest[globals.nest_ptr];
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0121::free_avail;
use crate::section_0213::head;
