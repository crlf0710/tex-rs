//! @ Finally, the \.{\\mathchoice} primitive creates a |choice_node|, which
//! has special subfields |display_mlist|, |text_mlist|, |script_mlist|,
//! and |script_script_mlist| pointing to the mlists for each style.
//
// @d choice_node=unset_node+2 {|type| of a choice node}
/// `type` of a choice node
pub(crate) const choice_node: quarterword = unset_node + 2;

// @d display_mlist(#)==info(#+1) {mlist to be used in display style}
// @d text_mlist(#)==link(#+1) {mlist to be used in text style}
// @d script_mlist(#)==info(#+2) {mlist to be used in script style}
// @d script_script_mlist(#)==link(#+2) {mlist to be used in scriptscript style}
//
// @p function new_choice:pointer; {create a choice node}
// var p:pointer; {the new node}
// begin p:=get_node(style_node_size); type(p):=choice_node;
// subtype(p):=0; {the |subtype| is not used}
// display_mlist(p):=null; text_mlist(p):=null; script_mlist(p):=null;
// script_script_mlist(p):=null;
// new_choice:=p;
// end;
//

use crate::section_0113::quarterword;
use crate::section_0159::unset_node;
