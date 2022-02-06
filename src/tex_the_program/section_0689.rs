//! @ Finally, the \.{\\mathchoice} primitive creates a |choice_node|, which
//! has special subfields |display_mlist|, |text_mlist|, |script_mlist|,
//! and |script_script_mlist| pointing to the mlists for each style.
//
// @d choice_node=unset_node+2 {|type| of a choice node}
/// `type` of a choice node
pub(crate) const choice_node: quarterword = unset_node + 2;

// @d display_mlist(#)==info(#+1) {mlist to be used in display style}
/// mlist to be used in display style
pub(crate) macro display_mlist($globals:expr, $v:expr) {
    crate::section_0118::info_inner!($globals, $v + 1)
}
// @d text_mlist(#)==link(#+1) {mlist to be used in text style}
/// mlist to be used in text style
pub(crate) macro text_mlist($globals:expr, $v:expr) {
    crate::section_0118::link!($globals, $v + 1)
}
// @d script_mlist(#)==info(#+2) {mlist to be used in script style}
/// mlist to be used in script style
pub(crate) macro script_mlist($globals:expr, $v:expr) {
    crate::section_0118::info_inner!($globals, $v + 2)
}
// @d script_script_mlist(#)==link(#+2) {mlist to be used in scriptscript style}
/// mlist to be used in scriptscript style
pub(crate) macro script_script_mlist($globals:expr, $v:expr) {
    crate::section_0118::link!($globals, $v + 2)
}
// @p function new_choice:pointer; {create a choice node}
/// create a choice node
pub(crate) fn new_choice(globals: &mut TeXGlobals) -> TeXResult<pointer> {
    // var p:pointer; {the new node}
    /// the new node
    let p;
    // begin p:=get_node(style_node_size); type(p):=choice_node;
    p = get_node(globals, style_node_size as _)?;
    r#type!(globals, p) = choice_node;
    // subtype(p):=0; {the |subtype| is not used}
    /// the `subtype` is not used
    const _: () = ();
    subtype!(globals, p) = 0;
    // display_mlist(p):=null; text_mlist(p):=null; script_mlist(p):=null;
    display_mlist!(globals, p) = null;
    text_mlist!(globals, p) = null;
    script_mlist!(globals, p) = null;
    // script_script_mlist(p):=null;
    script_script_mlist!(globals, p) = null;
    // new_choice:=p;
    let new_choice = p;
    // end;
    crate::ok_nojump!(new_choice)
}

use crate::section_0004::TeXGlobals;
use crate::section_0081::TeXResult;
use crate::section_0113::quarterword;
use crate::section_0115::null;
use crate::section_0115::pointer;
use crate::section_0125::get_node;
use crate::section_0133::r#type;
use crate::section_0133::subtype;
use crate::section_0159::unset_node;
use crate::section_0688::style_node_size;
