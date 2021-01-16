//! @ The |box_end| procedure does the right thing with |cur_box|, if
//! |box_context| represents the context as explained above.
//
// @<Declare act...@>=
// procedure box_end(@!box_context:integer);
pub(crate) fn box_end(globals: &mut TeXGlobals, box_context: integer) -> TeXResult<()> {
    // var p:pointer; {|ord_noad| for new box in math mode}
    // begin if box_context<box_flag then @<Append box |cur_box| to the current list,
    //     shifted by |box_context|@>
    if box_context < box_flag {
        todo!("append box to list");
    }
    // else if box_context<ship_out_flag then @<Store \(c)|cur_box| in a box register@>
    else if box_context < ship_out_flag {
        Store_cur_box_in_a_box_register!(globals, box_context);
    }
    // else if cur_box<>null then
    else if globals.cur_box != null {
        todo!("append or shipout");
        // if box_context>ship_out_flag then @<Append a new leader node that
        //     uses |cur_box|@>
        // else ship_out(cur_box);
    }
    // end;
    ok_nojump!()
}

use crate::pascal::integer;
use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_1071::box_flag;
use crate::section_1071::ship_out_flag;
