//! @ When \TeX's work on one level is interrupted, the state is saved by
//! calling |push_nest|. This routine changes |head| and |tail| so that
//! a new (empty) list is begun; it does not change |mode| or |aux|.
//
// @p procedure push_nest; {enter a new semantic level, save the old}
/// enter a new semantic level, save the old
pub(crate) fn push_nest(globals: &mut TeXGlobals) {
    // begin if nest_ptr>max_nest_stack then
    if globals.nest_ptr > globals.max_nest_stack {
        // begin max_nest_stack:=nest_ptr;
        globals.max_nest_stack = globals.nest_ptr;
        // if nest_ptr=nest_size then overflow("semantic nest size",nest_size);
        if globals.nest_ptr == nest_size {
            todo!("overflow");
        }
        // @:TeX capacity exceeded semantic nest size}{\quad semantic nest size@>
        // end;
    }
    // nest[nest_ptr]:=cur_list; {stack the record}
    /// stack the record
    const _: () = ();
    globals.nest[globals.nest_ptr] = globals.cur_list;
    // incr(nest_ptr); head:=get_avail; tail:=head; prev_graf:=0; mode_line:=line;
    incr!(globals.nest_ptr);
    head!(globals) = get_avail(globals);
    tail!(globals) = head!(globals);
    prev_graf!(globals) = 0;
    mode_line!(globals) = globals.line;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0011::nest_size;
use crate::section_0016::incr;
use crate::section_0120::get_avail;
use crate::section_0213::head;
use crate::section_0213::mode_line;
use crate::section_0213::prev_graf;
use crate::section_0213::tail;
