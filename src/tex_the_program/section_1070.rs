//! @ Here is where we clear the parameters that are supposed to revert to their
//! default values after every paragraph and when internal vertical mode is entered.
//
// @<Declare act...@>=
// procedure normal_paragraph;
#[allow(unused_variables)]
pub(crate) fn normal_paragraph(globals: &mut TeXGlobals) -> TeXResult<()> {
    // begin if looseness<>0 then eq_word_define(int_base+looseness_code,0);
    if looseness!(globals) != 0 {
        eq_word_define(globals, int_base as pointer + looseness_code as pointer, 0);
    }
    // if hang_indent<>0 then eq_word_define(dimen_base+hang_indent_code,0);
    if hang_indent!(globals) != scaled::zero() {
        eq_word_define(globals, dimen_base as pointer + hang_indent_code as pointer,0);
    }
    // if hang_after<>1 then eq_word_define(int_base+hang_after_code,1);
    if hang_after!(globals) != 1 {
        eq_word_define(globals, int_base as pointer + hang_after_code as pointer, 1);
    }
    // if par_shape_ptr<>null then eq_define(par_shape_loc,shape_ref,null);
    if par_shape_ptr!(globals) != null {
        eq_define(globals, par_shape_loc, shape_ref, null)?;
    }
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0115::pointer;
use crate::section_0115::null;
use crate::section_0210::shape_ref;
use crate::section_0230::par_shape_loc;
use crate::section_0230::int_base;
use crate::section_0236::dimen_base;
use crate::section_0236::looseness_code;
use crate::section_0236::hang_after_code;
use crate::section_0247::hang_indent_code;
use crate::section_0277::eq_define;
use crate::section_0278::eq_word_define;
