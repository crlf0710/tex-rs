//! @ When a glue register or parameter becomes zero, it will always point to
//! |zero_glue| because of the following procedure. (Exception: The tabskip
//! glue isn't trapped while preambles are being scanned.)
//
// @<Declare subprocedures for |prefixed_command|@>=
// procedure trap_zero_glue;
pub(crate) fn trap_zero_glue(globals: &mut TeXGlobals) {
    // begin if (width(cur_val)=0)and(stretch(cur_val)=0)and(shrink(cur_val)=0) then
    if width!(globals, globals.cur_val as pointer) == scaled::zero()
        && stretch!(globals, globals.cur_val as pointer) == scaled::zero()
        && shrink!(globals, globals.cur_val as pointer) == scaled::zero()
    {
        // begin add_glue_ref(zero_glue);
        add_glue_ref!(globals, zero_glue);
        // delete_glue_ref(cur_val); cur_val:=zero_glue;
        delete_glue_ref(globals, globals.cur_val as pointer);
        globals.cur_val = zero_glue.into();
        // end;
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0162::zero_glue;
use crate::section_0201::delete_glue_ref;
