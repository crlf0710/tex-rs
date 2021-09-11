//! @ A mild optimization of the output is performed by the |dvi_pop|
//! routine, which issues a |pop| unless it is possible to cancel a
//! `|push| |pop|' pair. The parameter to |dvi_pop| is the byte address
//! following the old |push| that matches the new |pop|.
//
// @p procedure dvi_pop(@!l:integer);
pub(crate) fn dvi_pop(globals: &mut TeXGlobals, l: integer) {
    // begin if (l=dvi_offset+dvi_ptr)and(dvi_ptr>0) then decr(dvi_ptr)
    if l == globals.dvi_offset + globals.dvi_ptr.get() as integer && globals.dvi_ptr.get() > 0 {
        decr!(globals.dvi_ptr);
    }
    // else dvi_out(pop);
    else {
        dvi_out!(globals, pop.byte());
    }
    // end;
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0016::decr;
use crate::section_0586::pop;
use crate::section_0598::dvi_out;
