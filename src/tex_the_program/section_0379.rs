//! @ Sometimes the expansion looks too far ahead, so we want to insert
//! a harmless \.{\\relax} into the user's input.
//
// @<Declare the procedure called |insert_relax|@>=
// procedure insert_relax;
pub(crate) fn insert_relax(globals: &mut TeXGlobals) {
    // begin cur_tok:=cs_token_flag+cur_cs; back_input;
    globals.cur_tok = cur_tok_type::from_cs(globals.cur_cs);
    back_input(globals);
    // cur_tok:=cs_token_flag+frozen_relax; back_input; token_type:=inserted;
    globals.cur_tok = cur_tok_type::from_cs(frozen_relax as _);
    back_input(globals);
    token_type!(globals) = inserted;
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0222::frozen_relax;
use crate::section_0297::cur_tok_type;
use crate::section_0307::inserted;
use crate::section_0325::back_input;
