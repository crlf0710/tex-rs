//! @ To save a value of |eqtb[p]| that was established at level |l|, we
//! can use the following subroutine.
//
// @p procedure eq_save(@!p:pointer;@!l:quarterword); {saves |eqtb[p]|}
/// saves `eqtb[p]`
#[allow(unused_variables)]
pub(crate) fn eq_save(globals: &mut TeXGlobals, p:pointer, l:quarterword) {
    // begin check_full_save_stack;
    check_full_save_stack!(globals);
    // if l=level_zero then save_type(save_ptr):=restore_zero
    if l == level_zero {
        save_type!(globals, globals.save_ptr) = restore_zero;
    }
    // else  begin save_stack[save_ptr]:=eqtb[p]; incr(save_ptr);
    else {
        globals.save_stack[globals.save_ptr] = globals.eqtb[p];
        incr!(globals.save_ptr);
        // save_type(save_ptr):=restore_old_value;
        save_type!(globals, globals.save_ptr) = restore_old_value;
        // end;
    }
    // save_level(save_ptr):=l; save_index(save_ptr):=p; incr(save_ptr);
    save_level!(globals, globals.save_ptr) = l;
    save_index!(globals, globals.save_ptr) = p;
    incr!(globals.save_ptr);
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0113::quarterword;
use crate::section_0115::pointer;
use crate::section_0221::level_zero;
use crate::section_0268::restore_old_value;
use crate::section_0268::restore_zero;
