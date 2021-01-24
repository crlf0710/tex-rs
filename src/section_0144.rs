//! @ The |new_ligature| function creates a ligature node having given
//! contents of the |font|, |character|, and |lig_ptr| fields. We also have
//! a |new_lig_item| function, which returns a two-word node having a given
//! |character| field. Such nodes are used for temporary processing as ligatures
//! are being created.
//
// @p function new_ligature(@!f,@!c:quarterword; @!q:pointer):pointer;
pub(crate) fn new_ligature(
    globals: &mut TeXGlobals,
    f: internal_font_number,
    c: ASCII_code,
    q: pointer,
) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(small_node_size); type(p):=ligature_node;
    p = get_node(globals, small_node_size.into())?;
    r#type!(globals, p) = ligature_node;
    // font(lig_char(p)):=f; character(lig_char(p)):=c; lig_ptr(p):=q;
    assign_font_and_character!(globals, lig_char!(p), f, c);
    lig_ptr!(globals, p) = q;
    // subtype(p):=0; new_ligature:=p;
    subtype!(globals, p) = 0;
    ok_nojump!(p)
    // end;
}
// @#
// function new_lig_item(@!c:quarterword):pointer;
pub(crate) fn new_lig_item(globals: &mut TeXGlobals, c: ASCII_code) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p: pointer;
    // begin p:=get_node(small_node_size); character(p):=c; lig_ptr(p):=null;
    p = get_node(globals, small_node_size.into())?;
    assign_font_and_character!(globals, p, null_font, c);
    lig_ptr!(globals, p) = null;
    // new_lig_item:=p;
    ok_nojump!(p)
    // end;
}

use crate::section_0004::TeXGlobals;
use crate::section_0018::ASCII_code;
use crate::section_0081::TeXResult;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0141::small_node_size;
use crate::section_0143::ligature_node;
use crate::section_0232::null_font;
use crate::section_0548::internal_font_number;
