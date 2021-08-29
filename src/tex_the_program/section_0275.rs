//! @ Just before an entry of |eqtb| is changed, the following procedure should
//! be called to update the other data structures properly. It is important
//! to keep in mind that reference counts in |mem| include references from
//! within |save_stack|, so these counts must be handled carefully.
//! @^reference counts@>
//
// @p procedure eq_destroy(@!w:memory_word); {gets ready to forget |w|}
/// gets ready to forget `w`
#[allow(unused_variables)]
pub(crate) fn eq_destroy(globals: &mut TeXGlobals, w: memory_word) -> TeXResult<()> {
    // var q:pointer; {|equiv| field of |w|}
    let eq_type_field_w = eq_type_field!(w);
    // begin case eq_type_field(w) of
    // call,long_call,outer_call,long_outer_call: delete_token_ref(equiv_field(w));
    if eq_type_field_w == call
        || eq_type_field_w == long_call
        || eq_type_field_w == outer_call
        || eq_type_field_w == long_outer_call
    {
        delete_token_ref(globals, equiv_field!(w));
    }
    // glue_ref: delete_glue_ref(equiv_field(w));
    else if eq_type_field_w == glue_ref {
        delete_glue_ref(globals, equiv_field!(w));
    }
    // shape_ref: begin q:=equiv_field(w); {we need to free a \.{\\parshape} block}
    else if eq_type_field_w == shape_ref {
        todo!("shape_ref");
        // if q<>null then free_node(q,info(q)+info(q)+1);
        // end; {such a block is |2n+1| words long, where |n=info(q)|}
    }
    // box_ref: flush_node_list(equiv_field(w));
    else if eq_type_field_w == box_ref {
        flush_node_list(globals, equiv_field!(w))?;
    }
    // othercases do_nothing
    else {
        do_nothing!();
    }
    // endcases;
    // end;
    ok_nojump!()
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::memory_word;
use crate::section_0200::delete_token_ref;
use crate::section_0201::delete_glue_ref;
use crate::section_0202::flush_node_list;
use crate::section_0210::*;
