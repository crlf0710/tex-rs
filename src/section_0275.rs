//! @ Just before an entry of |eqtb| is changed, the following procedure should
//! be called to update the other data structures properly. It is important
//! to keep in mind that reference counts in |mem| include references from
//! within |save_stack|, so these counts must be handled carefully.
//! @^reference counts@>
//
// @p procedure eq_destroy(@!w:memory_word); {gets ready to forget |w|}
/// gets ready to forget `w`
#[allow(unused_variables)]
pub(crate) fn eq_destroy(w: memory_word) {
    // var q:pointer; {|equiv| field of |w|}
    // begin case eq_type_field(w) of
    // call,long_call,outer_call,long_outer_call: delete_token_ref(equiv_field(w));
    // glue_ref: delete_glue_ref(equiv_field(w));
    // shape_ref: begin q:=equiv_field(w); {we need to free a \.{\\parshape} block}
    //   if q<>null then free_node(q,info(q)+info(q)+1);
    //   end; {such a block is |2n+1| words long, where |n=info(q)|}
    // box_ref: flush_node_list(equiv_field(w));
    // othercases do_nothing
    // endcases;
    // end;
}

use crate::section_0113::memory_word;
