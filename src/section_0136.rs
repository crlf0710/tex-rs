//! @ The |new_null_box| function returns a pointer to an |hlist_node| in
//! which all subfields have the values corresponding to `\.{\\hbox\{\}}'.
//! (The |subtype| field is set to |min_quarterword|, for historic reasons
//! that are no longer relevant.)
//
// @p function new_null_box:pointer; {creates a new box node}
/// creates a new box node
pub(crate) fn new_null_box(globals: &mut TeXGlobals) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(box_node_size); type(p):=hlist_node;
    p = get_node(globals, box_node_size as _)?;
    r#type!(globals, p) = hlist_node;
    // subtype(p):=min_quarterword;
    subtype!(globals, p) = min_quarterword;
    // width(p):=0; depth(p):=0; height(p):=0; shift_amount(p):=0; list_ptr(p):=null;
    width!(globals, p) = scaled::zero();
    depth!(globals, p) = scaled::zero();
    height!(globals, p) = scaled::zero();
    shift_amount!(globals, p) = scaled::zero();
    list_ptr!(globals, p) = null;
    // glue_sign(p):=normal; glue_order(p):=normal; set_glue_ratio_zero(glue_set(p));
    glue_sign!(globals, p) = glue_sign::normal as _;
    glue_order!(globals, p) = glue_ord::normal as _;
    set_glue_ratio_zero!(glue_set!(globals, p));
    // new_null_box:=p;
    ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0101::scaled;
use crate::section_0110::min_quarterword;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0135::box_node_size;
use crate::section_0135::glue_sign;
use crate::section_0135::hlist_node;
use crate::section_0150::glue_ord;
