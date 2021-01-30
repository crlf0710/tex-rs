//! @ An |align_group| code is supposed to remain on the |save_stack|
//! during an entire alignment, until |fin_align| removes it.
//!
//! A devious user might force an |endv| command to occur just about anywhere;
//! we must defeat such hacks.
//
// @<Declare act...@>=
// procedure do_endv;
pub(crate) fn do_endv(globals: &mut TeXGlobals) {
    // begin base_ptr:=input_ptr; input_stack[base_ptr]:=cur_input;
    globals.base_ptr = globals.input_ptr;
    globals.input_stack[globals.base_ptr] = globals.cur_input;
    // while (input_stack[base_ptr].index_field<>v_template) and
    //       (input_stack[base_ptr].loc_field=null) and
    //       (input_stack[base_ptr].state_field=token_list) do decr(base_ptr);
    while globals.input_stack[globals.base_ptr].index_field != v_template &&
        globals.input_stack[globals.base_ptr].loc_field == null &&
        globals.input_stack[globals.base_ptr].state_field == token_list {
        decr!(globals.base_ptr);
    }
    // if (input_stack[base_ptr].index_field<>v_template) or
    //       (input_stack[base_ptr].loc_field<>null) or
    //       (input_stack[base_ptr].state_field<>token_list) then
    if globals.input_stack[globals.base_ptr].index_field != v_template ||
        globals.input_stack[globals.base_ptr].loc_field != null ||
        globals.input_stack[globals.base_ptr].state_field != token_list {
        // fatal_error("(interwoven alignment preambles are not allowed)");
        todo!("fatal error");
    }
    // @.interwoven alignment preambles...@>
    // if cur_group=align_group then
    if globals.cur_group == align_group {
        // begin end_graf;
        end_graf(globals);
        // if fin_col then fin_row;
        if fin_col(globals) {
            fin_row(globals);
        }
        // end
    }
    // else off_save;
    else {
        off_save(globals);
    }
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0115::null;
use crate::section_0269::align_group;
use crate::section_0307::token_list;
use crate::section_0307::v_template;
use crate::section_0791::fin_col;
use crate::section_0799::fin_row;
use crate::section_1096::end_graf;
use crate::section_1064::off_save;
